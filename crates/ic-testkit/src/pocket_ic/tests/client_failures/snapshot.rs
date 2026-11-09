use super::*;
use crate::pocket_ic::{
    common::rest::{RawCanisterCall, RawCanisterResult},
    snapshot::*,
};
use ic_platform::candid_codec::{
    arguments::EncodeArgs,
    decoder::{Arity, Limits, decode_args},
};
use ic_platform_types::Principal;

fn encode_args<T: EncodeArgs>(args: T) -> Result<Vec<u8>, ic_platform::candid_codec::Error> {
    ic_platform::candid_codec::arguments::encode_args(&args, 100_000)
}
use ic_platform::management_canister_types::{
    CanisterTimer, ChunkHash, OnLowWasmMemoryHookStatus, SnapshotMetadataGlobal,
};
use sha2::{Digest, Sha256};

fn limits() -> TransferLimits {
    TransferLimits {
        call: CallLimits {
            http_bytes: 100_000,
            candid_bytes: 50_000,
            type_entries: 1024,
            header_bytes: 65536,
            work: 1_000_000,
            allocation: 1_000_000,
        },
        chunk_bytes: 4,
        total_bytes: 100,
    }
}
fn metadata() -> ReadCanisterSnapshotMetadataResult {
    ReadCanisterSnapshotMetadataResult {
        source: None,
        taken_at_timestamp: 0,
        wasm_module_size: 3,
        globals: vec![
            Some(SnapshotMetadataGlobal::I64(42)),
            None,
            Some(SnapshotMetadataGlobal::I32(7)),
        ],
        wasm_memory_size: 0,
        stable_memory_size: 0,
        wasm_chunk_store: vec![],
        canister_version: 0,
        certified_data: vec![9],
        global_timer: Some(CanisterTimer::Active(123)),
        on_low_wasm_memory_hook_status: Some(OnLowWasmMemoryHookStatus::Executed),
    }
}
fn transfer(metadata: ReadCanisterSnapshotMetadataResult) -> SnapshotTransfer {
    SnapshotTransfer::new(
        Principal::from_slice(&[7]),
        Principal::from_slice(&[8]),
        vec![1],
        metadata,
        limits(),
    )
    .unwrap()
}
fn call_steps(bytes: Vec<u8>) -> Vec<Step> {
    vec![
        Step::reply(
            "/instances/7/update/submit_ingress_message",
            200,
            r#"{"Ok":{"effective_principal":{"CanisterId":"Bw=="},"message_id":"AQI="}}"#,
        ),
        Step::reply(
            "/instances/7/update/await_ingress_message",
            200,
            Box::leak(
                serde_json::to_string(&RawCanisterResult::Ok(bytes))
                    .unwrap()
                    .into_boxed_str(),
            ),
        ),
    ]
}
fn uploaded_steps() -> Vec<Step> {
    call_steps(
        encode_args((UploadCanisterSnapshotMetadataResult {
            snapshot_id: vec![2],
        },))
        .unwrap(),
    )
}
fn call(request: &str) -> RawCanisterCall {
    serde_json::from_str(request.split_once('\n').unwrap().1).unwrap()
}

#[tokio::test]
async fn sync_copy_preserves_exact_dto_bytes_sparse_globals_and_sender() {
    let mut steps = uploaded_steps();
    steps.extend(call_steps(
        encode_args((ReadCanisterSnapshotDataResult {
            chunk: vec![1, 2, 3],
        },))
        .unwrap(),
    ));
    steps.extend(call_steps(encode_args(()).unwrap()));
    let server = Server::new(steps);
    let client = server.client(Duration::from_secs(1));
    let mut copy = transfer(metadata());
    copy.advance(&client).await.unwrap();
    assert_eq!(copy.uploaded_snapshot(), Some([2].as_slice()));
    assert_eq!(copy.transferred_bytes(), 0);
    copy.advance(&client).await.unwrap();
    copy.advance(&client).await.unwrap();
    assert!(matches!(copy.status(), TransferStatus::Complete));
    let requests = server.finish();
    assert_eq!(requests.len(), 6);
    let upload = call(&requests[0]);
    assert_eq!(upload.sender, [8]);
    assert_eq!(
        upload.canister_id,
        Principal::management_canister().as_slice()
    );
    assert_eq!(
        upload.effective_principal,
        crate::pocket_ic::common::rest::RawEffectivePrincipal::CanisterId(vec![7])
    );
    let expected = UploadCanisterSnapshotMetadataArgs {
        canister_id: Principal::from_slice(&[7]),
        replace_snapshot: None,
        wasm_module_size: 3,
        globals: vec![
            SnapshotMetadataGlobal::I64(42),
            SnapshotMetadataGlobal::I32(7),
        ],
        wasm_memory_size: 0,
        stable_memory_size: 0,
        certified_data: vec![9],
        global_timer: Some(CanisterTimer::Active(123)),
        on_low_wasm_memory_hook_status: Some(OnLowWasmMemoryHookStatus::Executed),
    };
    assert_eq!(upload.payload, encode_args((expected,)).unwrap());
    assert_eq!(
        call(&requests[2]).payload,
        encode_args((ReadCanisterSnapshotDataArgs {
            canister_id: Principal::from_slice(&[7]),
            snapshot_id: vec![1],
            kind: SnapshotDataKind::WasmModule { offset: 0, size: 3 }
        },))
        .unwrap()
    );
    assert_eq!(
        call(&requests[4]).payload,
        encode_args((UploadCanisterSnapshotDataArgs {
            canister_id: Principal::from_slice(&[7]),
            snapshot_id: vec![2],
            kind: SnapshotDataOffset::WasmModule { offset: 0 },
            chunk: vec![1, 2, 3]
        },))
        .unwrap()
    );
}

#[tokio::test]
async fn partial_and_oversized_regions_refuse_without_uploading_or_losing_custody() {
    for chunk in [vec![1, 2], vec![1, 2, 3, 4, 5]] {
        let mut steps = uploaded_steps();
        steps.extend(call_steps(
            encode_args((ReadCanisterSnapshotDataResult { chunk },)).unwrap(),
        ));
        let server = Server::new(steps);
        let client = server.client(Duration::from_secs(1));
        let mut copy = transfer(metadata());
        copy.advance(&client).await.unwrap();
        assert!(matches!(
            copy.advance(&client).await,
            Err(SnapshotError::Invalid(_))
        ));
        assert_eq!(copy.uploaded_snapshot(), Some([2].as_slice()));
        assert!(matches!(copy.status(), TransferStatus::Reading(_)));
        assert!(copy.advance(&client).await.is_err());
        assert_eq!(server.finish().len(), 4);
    }
}

#[tokio::test]
async fn chunk_hash_and_total_budget_are_checked_before_upload() {
    for wrong_hash in [true, false] {
        let data = vec![4, 5, 6];
        let mut meta = metadata();
        meta.wasm_module_size = 0;
        meta.wasm_chunk_store = vec![ChunkHash {
            hash: if wrong_hash {
                vec![0; 32]
            } else {
                Sha256::digest(&data).to_vec()
            },
        }];
        let mut copy = transfer(meta);
        if !wrong_hash {
            copy.limits.total_bytes = 2;
        }
        let mut steps = uploaded_steps();
        steps.extend(call_steps(
            encode_args((ReadCanisterSnapshotDataResult { chunk: data },)).unwrap(),
        ));
        let server = Server::new(steps);
        let client = server.client(Duration::from_secs(1));
        copy.advance(&client).await.unwrap();
        assert!(matches!(
            copy.advance(&client).await,
            Err(SnapshotError::Invalid(_))
        ));
        assert_eq!(server.finish().len(), 4);
    }
}

#[tokio::test]
async fn interrupted_upload_retains_id_ingress_and_no_replay() {
    let mut steps = uploaded_steps();
    steps.extend(call_steps(
        encode_args((ReadCanisterSnapshotDataResult {
            chunk: vec![1, 2, 3],
        },))
        .unwrap(),
    ));
    let mut lost = call_steps(vec![]);
    lost[1].status = 0;
    steps.extend(lost);
    let server = Server::new(steps);
    let client = server.client(Duration::from_secs(1));
    let mut copy = transfer(metadata());
    copy.advance(&client).await.unwrap();
    let error = copy.advance(&client).await.unwrap_err();
    assert!(matches!(
        error,
        SnapshotError::Call {
            source: ClientError::Transport { .. },
            ingress: Some(_)
        }
    ));
    assert!(matches!(copy.status(), TransferStatus::Writing(_)));
    assert_eq!(copy.ingress().unwrap().message_id, [1, 2]);
    assert_eq!(copy.uploaded_snapshot(), Some([2].as_slice()));
    assert!(copy.advance(&client).await.is_err());
    assert_eq!(server.finish().len(), 6);
}

#[tokio::test]
async fn cancelled_metadata_upload_preserves_accepted_ingress() {
    let mut steps = uploaded_steps();
    let (observed, request) = tokio::sync::oneshot::channel();
    let (release, held) = std::sync::mpsc::channel();
    steps[1].hold = Some((observed, held));
    let server = Server::new(steps);
    let client = server.client(Duration::from_secs(30));
    let mut copy = transfer(metadata());
    {
        let upload = copy.advance(&client);
        tokio::pin!(upload);
        tokio::select! {
            result = &mut upload => panic!("upload ended before cancellation: {result:?}"),
            observed = request => observed.expect("observe accepted ingress wait"),
        }
        // Dropping this in-flight future cancels only after the accepted ingress
        // receipt and its subsequent wait request are both observed.
    }
    release.send(()).expect("release cancelled upload wait");
    assert!(matches!(copy.status(), TransferStatus::UploadingMetadata));
    assert!(copy.ingress().is_some());
    assert!(copy.uploaded_snapshot().is_none());
    assert!(copy.advance(&client).await.is_err());
    assert_eq!(server.finish().len(), 2);
}

#[tokio::test]
async fn reply_limits_and_codec_errors_keep_accepted_ingress() {
    for mode in ["http", "candid", "malformed", "shape", "quota"] {
        let reply = if mode == "malformed" {
            b"broken".to_vec()
        } else {
            encode_args((vec![0u8; 1000],)).unwrap()
        };
        let server = Server::new(call_steps(reply));
        let client = server.client(Duration::from_secs(1));
        let mut budget = limits().call;
        if mode == "http" {
            budget.http_bytes = 100;
        }
        if mode == "candid" {
            budget.candid_bytes = 200;
        }
        if mode == "quota" {
            budget.work = 0;
        }
        let error = client
            .read_canister_snapshot_metadata(
                Principal::anonymous(),
                ReadCanisterSnapshotMetadataArgs {
                    canister_id: Principal::from_slice(&[7]),
                    snapshot_id: vec![1],
                },
                budget,
            )
            .await
            .unwrap_err();
        let SnapshotError::Call { source, ingress } = error else {
            panic!("wrong error")
        };
        assert!(ingress.is_some());
        match mode {
            "http" => assert!(matches!(source, ClientError::ResponseTooLarge { .. })),
            "candid" => assert!(matches!(source, ClientError::Protocol(_))),
            _ => assert!(matches!(source, ClientError::CandidDecode(_))),
        }
        assert_eq!(server.finish().len(), 2);
    }
}

#[test]
fn invalid_metadata_is_refused_before_any_upload() {
    for mode in [
        "timer",
        "hook",
        "size",
        "overflow",
        "hash",
        "duplicate",
        "chunk",
    ] {
        let mut meta = metadata();
        let mut budget = limits();
        match mode {
            "timer" => meta.global_timer = None,
            "hook" => meta.on_low_wasm_memory_hook_status = None,
            "size" => meta.wasm_module_size = 101,
            "overflow" => {
                meta.wasm_module_size = u64::MAX;
                meta.wasm_memory_size = 1;
            }
            "hash" => meta.wasm_chunk_store = vec![ChunkHash { hash: vec![0; 31] }],
            "duplicate" => meta.wasm_chunk_store = vec![ChunkHash { hash: vec![0; 32] }; 2],
            "chunk" => budget.chunk_bytes = 0,
            _ => unreachable!(),
        }
        assert!(
            SnapshotTransfer::new(
                Principal::anonymous(),
                Principal::anonymous(),
                vec![1],
                meta,
                budget
            )
            .is_err()
        );
    }
}

#[tokio::test]
async fn async_copy_bounds_each_region_and_preserves_valid_chunk_store() {
    let mut meta = metadata();
    meta.wasm_module_size = 5;
    meta.wasm_memory_size = 2;
    meta.stable_memory_size = 1;
    meta.wasm_chunk_store = vec![ChunkHash {
        hash: Sha256::digest([7, 8]).to_vec(),
    }];
    let mut steps = uploaded_steps();
    for chunk in [vec![1, 2, 3, 4], vec![5], vec![6, 7], vec![8], vec![7, 8]] {
        steps.extend(call_steps(
            encode_args((ReadCanisterSnapshotDataResult { chunk },)).unwrap(),
        ));
        steps.extend(call_steps(encode_args(()).unwrap()));
    }
    let server = Server::new(steps);
    let client = server.client(Duration::from_secs(1));
    let mut copy = transfer(meta);
    while !matches!(copy.status(), TransferStatus::Complete) {
        copy.advance(&client).await.unwrap();
    }
    assert_eq!(copy.transferred_bytes(), 10);
    let requests = server.finish();
    let kinds: Vec<_> = [2, 6, 10, 14, 18]
        .into_iter()
        .map(|i| {
            let (args,): (ReadCanisterSnapshotDataArgs,) = decode_args(
                &call(&requests[i]).payload,
                Limits {
                    bytes: 100_000,
                    header: 65536,
                    types: 1024,
                    work: 1_000_000,
                    allocation: 1_000_000,
                },
                Arity::Exact,
            )
            .unwrap();
            args.kind
        })
        .collect();
    assert!(matches!(
        kinds[0],
        SnapshotDataKind::WasmModule { offset: 0, size: 4 }
    ));
    assert!(matches!(
        kinds[1],
        SnapshotDataKind::WasmModule { offset: 4, size: 1 }
    ));
    assert!(matches!(
        kinds[2],
        SnapshotDataKind::WasmMemory { offset: 0, size: 2 }
    ));
    assert!(matches!(
        kinds[3],
        SnapshotDataKind::StableMemory { offset: 0, size: 1 }
    ));
    assert!(matches!(kinds[4], SnapshotDataKind::WasmChunk { .. }));
}

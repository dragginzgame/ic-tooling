use super::*;
use crate::pocket_ic::snapshot::*;
use candid::{CandidType, DecoderConfig};

impl PocketIc {
    async fn snapshot_call<A: CandidType, R: for<'de> ArgumentDecoder<'de>>(
        &self,
        canister: Principal,
        sender: Principal,
        method: &str,
        args: A,
        limits: CallLimits,
        ingress: &mut Option<RawMessageId>,
    ) -> Result<R, SnapshotError> {
        let result = async {
            let payload = candid::encode_one(args)?;
            if payload.len() > limits.candid_bytes {
                return Err(ClientError::Protocol(
                    "snapshot request exceeds Candid byte budget".into(),
                ));
            }
            let message: Result<RawMessageId, RejectResponse> = self
                .request_limited(
                    HttpMethod::Post,
                    "update/submit_ingress_message",
                    RawCanisterCall {
                        canister_id: Principal::management_canister().as_slice().to_vec(),
                        effective_principal: RawEffectivePrincipal::CanisterId(
                            canister.as_slice().to_vec(),
                        ),
                        sender: sender.as_slice().to_vec(),
                        method: method.into(),
                        payload,
                        sender_info: None,
                    },
                    Some(limits.http_bytes),
                )
                .await?;
            let message = message.map_err(ClientError::from)?;
            *ingress = Some(message.clone());
            let result: RawCanisterResult = self
                .request_limited(
                    HttpMethod::Post,
                    "update/await_ingress_message",
                    &message,
                    Some(limits.http_bytes),
                )
                .await
                .map_err(|error| error.with_ingress(message))?;
            let reply =
                Result::<Vec<u8>, RejectResponse>::from(result).map_err(ClientError::from)?;
            if reply.len() > limits.candid_bytes {
                return Err(ClientError::Protocol(
                    "snapshot reply exceeds Candid byte budget".into(),
                ));
            }
            let mut config = DecoderConfig::new();
            config.set_full_error_message(false);
            config.set_decoding_quota(limits.decoding_quota);
            config.set_skipping_quota(limits.skipping_quota);
            candid::decode_args_with_config(&reply, &config).map_err(ClientError::from)
        }
        .await;
        result.map_err(|source| SnapshotError::Call {
            source,
            ingress: ingress.clone(),
        })
    }

    pub async fn read_canister_snapshot_metadata(
        &self,
        sender: Principal,
        args: ReadCanisterSnapshotMetadataArgs,
        limits: CallLimits,
    ) -> Result<ReadCanisterSnapshotMetadataResult, SnapshotError> {
        Ok(self
            .snapshot_call::<_, (_,)>(
                args.canister_id,
                sender,
                "read_canister_snapshot_metadata",
                args,
                limits,
                &mut None,
            )
            .await?
            .0)
    }
    pub async fn read_canister_snapshot_data(
        &self,
        sender: Principal,
        args: ReadCanisterSnapshotDataArgs,
        limits: CallLimits,
    ) -> Result<ReadCanisterSnapshotDataResult, SnapshotError> {
        Ok(self
            .snapshot_call::<_, (_,)>(
                args.canister_id,
                sender,
                "read_canister_snapshot_data",
                args,
                limits,
                &mut None,
            )
            .await?
            .0)
    }
    pub async fn upload_canister_snapshot_metadata(
        &self,
        sender: Principal,
        args: UploadCanisterSnapshotMetadataArgs,
        limits: CallLimits,
    ) -> Result<UploadCanisterSnapshotMetadataResult, SnapshotError> {
        Ok(self
            .snapshot_call::<_, (_,)>(
                args.canister_id,
                sender,
                "upload_canister_snapshot_metadata",
                args,
                limits,
                &mut None,
            )
            .await?
            .0)
    }
    pub async fn upload_canister_snapshot_data(
        &self,
        sender: Principal,
        args: UploadCanisterSnapshotDataArgs,
        limits: CallLimits,
    ) -> Result<(), SnapshotError> {
        self.snapshot_upload_data(sender, args, limits, &mut None)
            .await
    }

    async fn snapshot_upload_data(
        &self,
        sender: Principal,
        args: UploadCanisterSnapshotDataArgs,
        limits: CallLimits,
        ingress: &mut Option<RawMessageId>,
    ) -> Result<(), SnapshotError> {
        self.snapshot_call(
            args.canister_id,
            sender,
            "upload_canister_snapshot_data",
            args,
            limits,
            ingress,
        )
        .await
    }
}

impl SnapshotTransfer {
    /// Copy one chunk, or create the upload and return its ID before copying data.
    /// Dropping this future leaves the attempted status and accepted ingress in self.
    pub async fn advance_async(&mut self, client: &PocketIc) -> Result<(), SnapshotError> {
        if matches!(self.status, TransferStatus::Complete) {
            return Ok(());
        }
        if !matches!(self.status, TransferStatus::Ready) {
            return Err(SnapshotError::Invalid(
                "interrupted transfer requires explicit reconciliation",
            ));
        }
        self.ingress = None;
        if self.uploaded.is_none() {
            self.status = TransferStatus::UploadingMetadata;
            let (uploaded,): (UploadCanisterSnapshotMetadataResult,) = client
                .snapshot_call(
                    self.canister,
                    self.sender,
                    "upload_canister_snapshot_metadata",
                    UploadCanisterSnapshotMetadataArgs {
                        canister_id: self.canister,
                        replace_snapshot: None,
                        wasm_module_size: self.metadata.wasm_module_size,
                        globals: self.metadata.globals.iter().flatten().cloned().collect(),
                        wasm_memory_size: self.metadata.wasm_memory_size,
                        stable_memory_size: self.metadata.stable_memory_size,
                        certified_data: self.metadata.certified_data.clone(),
                        global_timer: self.metadata.global_timer.clone(),
                        on_low_wasm_memory_hook_status: self
                            .metadata
                            .on_low_wasm_memory_hook_status
                            .clone(),
                    },
                    self.limits.call,
                    &mut self.ingress,
                )
                .await?;
            self.uploaded = Some(uploaded.snapshot_id);
            self.status = TransferStatus::Ready;
            return Ok(());
        }
        let sizes = [
            self.metadata.wasm_module_size,
            self.metadata.wasm_memory_size,
            self.metadata.stable_memory_size,
        ];
        while self.region < 3 && self.offset == sizes[self.region] {
            self.region += 1;
            self.offset = 0;
        }
        let (kind, destination, expected) = if self.region < 3 {
            let size = self
                .limits
                .chunk_bytes
                .min(sizes[self.region] - self.offset);
            let offset = self.offset;
            match self.region {
                0 => (
                    SnapshotDataKind::WasmModule { offset, size },
                    SnapshotDataOffset::WasmModule { offset },
                    Some(size),
                ),
                1 => (
                    SnapshotDataKind::WasmMemory { offset, size },
                    SnapshotDataOffset::WasmMemory { offset },
                    Some(size),
                ),
                _ => (
                    SnapshotDataKind::StableMemory { offset, size },
                    SnapshotDataOffset::StableMemory { offset },
                    Some(size),
                ),
            }
        } else if let Some(chunk) = self.metadata.wasm_chunk_store.get(self.region - 3) {
            (
                SnapshotDataKind::WasmChunk {
                    hash: chunk.hash.clone(),
                },
                SnapshotDataOffset::WasmChunk,
                None,
            )
        } else {
            self.status = TransferStatus::Complete;
            return Ok(());
        };
        self.status = TransferStatus::Reading(kind.clone());
        let (data,): (ReadCanisterSnapshotDataResult,) = client
            .snapshot_call(
                self.canister,
                self.sender,
                "read_canister_snapshot_data",
                ReadCanisterSnapshotDataArgs {
                    canister_id: self.canister,
                    snapshot_id: self.original.clone(),
                    kind: kind.clone(),
                },
                self.limits.call,
                &mut self.ingress,
            )
            .await?;
        let bytes = data.chunk.len() as u64;
        if bytes > self.limits.chunk_bytes || expected.is_some_and(|size| bytes != size) {
            return Err(SnapshotError::Invalid(
                "snapshot returned an incomplete or oversized chunk",
            ));
        }
        if let SnapshotDataKind::WasmChunk { hash } = kind
            && Sha256::digest(&data.chunk)[..] != hash
        {
            return Err(SnapshotError::Invalid("snapshot chunk hash mismatch"));
        }
        let transferred = self
            .transferred
            .checked_add(bytes)
            .filter(|bytes| *bytes <= self.limits.total_bytes)
            .ok_or(SnapshotError::Invalid("snapshot exceeds total byte budget"))?;
        self.status = TransferStatus::Writing(destination.clone());
        self.ingress = None;
        client
            .snapshot_upload_data(
                self.sender,
                UploadCanisterSnapshotDataArgs {
                    canister_id: self.canister,
                    snapshot_id: self.uploaded.clone().unwrap(),
                    kind: destination,
                    chunk: data.chunk,
                },
                self.limits.call,
                &mut self.ingress,
            )
            .await?;
        self.transferred = transferred;
        if self.region < 3 {
            self.offset += bytes;
        } else {
            self.region += 1;
        }
        self.status = TransferStatus::Ready;
        Ok(())
    }
}

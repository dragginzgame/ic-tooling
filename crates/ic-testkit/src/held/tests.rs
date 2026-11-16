use super::{Budget, Catalog, Error, Limits, Reply};

#[test]
fn owns_ordered_duplicate_slots_and_opaque_page_spanning_bytes() {
    let mut bytes = vec![0xff; 65_537];
    bytes[65_536] = 0;
    let catalog = Catalog::new(
        &[
            Reply {
                method: "réply",
                bytes: &bytes,
            },
            Reply {
                method: "réply",
                bytes: b"different\0",
            },
        ],
        Limits {
            slots: 2,
            name_bytes: 12,
            reply_bytes: 65_547,
        },
    )
    .unwrap();
    bytes.fill(1);
    drop(bytes);
    assert_eq!(catalog.len(), 2);
    let first = catalog.get(0).unwrap();
    assert_eq!(first.method, "réply");
    assert_eq!(first.bytes.len(), 65_537);
    assert!(first.bytes[..65_536].iter().all(|byte| *byte == 0xff));
    assert_eq!(first.bytes[65_536], 0);
    assert_eq!(catalog.get(1).unwrap().bytes, b"different\0");
    assert!(catalog.get(2).is_none());
}

#[test]
fn aggregate_boundaries_include_every_duplicate_slot() {
    let replies = [Reply {
        method: "x",
        bytes: b"ab",
    }; 2];
    let exact = Limits {
        slots: 2,
        name_bytes: 2,
        reply_bytes: 4,
    };
    assert!(Catalog::new(&replies, exact).is_ok());
    for (limits, expected, cap) in [
        (Limits { slots: 1, ..exact }, Budget::Slots, 1),
        (
            Limits {
                name_bytes: 1,
                ..exact
            },
            Budget::Names,
            1,
        ),
        (
            Limits {
                reply_bytes: 3,
                ..exact
            },
            Budget::Replies,
            3,
        ),
    ] {
        assert!(
            matches!(Catalog::new(&replies, limits), Err(Error::Limit { budget, limit }) if budget == expected && limit == cap)
        );
    }
    let empty = Catalog::new(
        &[],
        Limits {
            slots: 0,
            name_bytes: 0,
            reply_bytes: 0,
        },
    )
    .unwrap();
    assert!(empty.is_empty());
    assert!(empty.get(0).is_none());
}

#[test]
fn refuses_invalid_later_names_without_returning_partial_catalogs() {
    for name in ["", "bad\0name", "bad\nname", "bad\u{85}name"] {
        let replies = [
            Reply {
                method: "valid",
                bytes: b"one",
            },
            Reply {
                method: name,
                bytes: b"two",
            },
        ];
        assert!(matches!(
            Catalog::new(
                &replies,
                Limits {
                    slots: 2,
                    name_bytes: 64,
                    reply_bytes: 6
                }
            ),
            Err(Error::InvalidName { slot: 1 })
        ));
    }
}

#[test]
fn compiled_module_caps_are_exact_and_empty_catalog_is_valid() {
    let catalog = Catalog::new(
        &[],
        Limits {
            slots: 0,
            name_bytes: 0,
            reply_bytes: 0,
        },
    )
    .unwrap();
    let target = ic_platform_types::Principal::anonymous();
    let wasm = catalog.compile(target, 0, 10_000).unwrap();
    wasmparser::Validator::new().validate_all(&wasm).unwrap();
    assert_eq!(catalog.compile(target, 0, wasm.len()).unwrap(), wasm);
    assert!(matches!(
        catalog.compile(target, 0, wasm.len() - 1),
        Err(super::CompileError::ModuleLimit { .. })
    ));
    assert!(matches!(
        catalog.compile(target, 0, 0),
        Err(super::CompileError::ModuleLimit { limit: 0 })
    ));
    let catalog = Catalog::new(
        &[Reply {
            method: "hold",
            bytes: b"",
        }],
        Limits {
            slots: 1,
            name_bytes: 4,
            reply_bytes: 0,
        },
    )
    .unwrap();
    assert!(matches!(
        catalog.compile(target, 0, 64),
        Err(super::CompileError::Encoding(
            ic_platform_protocol::http_request::EncodeError::OutputLimit { max_bytes: 64 }
        ))
    ));
}

#[test]
fn generated_module_preserves_opaque_data_and_escapes_export_names() {
    use wasmparser::{ExternalKind, Parser, Payload};

    let bytes: Vec<u8> = (0..65_537).map(|n| (n % 256) as u8).collect();
    let name = "réply\"\\()";
    let catalog = Catalog::new(
        &[
            Reply {
                method: name,
                bytes: &bytes,
            },
            Reply {
                method: name,
                bytes: b"second",
            },
        ],
        Limits {
            slots: 2,
            name_bytes: 100,
            reply_bytes: 70_000,
        },
    )
    .unwrap();
    let target = ic_platform_types::Principal::anonymous();
    let wasm = catalog.compile(target, u128::MAX, 100_000).unwrap();
    wasmparser::Validator::new().validate_all(&wasm).unwrap();
    assert_eq!(
        catalog.compile(target, u128::MAX, wasm.len()).unwrap(),
        wasm
    );
    assert!(matches!(
        catalog.compile(target, u128::MAX, wasm.len() - 1),
        Err(super::CompileError::ModuleLimit { .. })
    ));
    let mut exports = vec![];
    let mut data = vec![];
    for payload in Parser::new(0).parse_all(&wasm) {
        match payload.unwrap() {
            Payload::ExportSection(section) => {
                for item in section {
                    let item = item.unwrap();
                    assert_eq!(item.kind, ExternalKind::Func);
                    exports.push(item.name.to_owned());
                }
            }
            Payload::DataSection(section) => {
                for item in section {
                    data.push(item.unwrap().data.to_vec());
                }
            }
            _ => {}
        }
    }
    assert_eq!(exports, [format!("canister_update {name}")]);
    assert_eq!(data.len(), 5); // shared literals, then reply and barrier per slot
    assert_eq!(data[1], bytes);
    assert_eq!(data[3], b"second");
    assert_ne!(data[2], data[4]); // slot identity participates in the encoded request
}

#[test]
#[ignore = "requires an explicitly owned IC_TESTKIT_POCKET_IC_URL server"]
fn installed_modules_keep_replies_bound_to_slots_and_reject_failures() {
    use pocket_ic::PocketIcBuilder;
    use pocket_ic::common::rest::{
        CanisterHttpMethod, CanisterHttpPricingVersion, CanisterHttpReject,
        CanisterHttpReplication, CanisterHttpReply, CanisterHttpResponse, MockCanisterHttpResponse,
    };

    let endpoint = std::env::var("IC_TESTKIT_POCKET_IC_URL")
        .expect("module runtime test requires an externally owned server endpoint");
    let pic = PocketIcBuilder::new()
        .with_application_subnet()
        .with_server_url(endpoint.parse().unwrap())
        .build();
    let a = pic.create_canister();
    let b = pic.create_canister();
    let page: Vec<u8> = (0..65_537).map(|n| (n % 251) as u8).collect();
    for (target, first, second) in [
        (a, page.as_slice(), b"second-a".as_slice()),
        (b, b"first-b".as_slice(), b"must-not-reply".as_slice()),
    ] {
        let catalog = Catalog::new(
            &[
                Reply {
                    method: "hold",
                    bytes: first,
                },
                Reply {
                    method: "hold",
                    bytes: second,
                },
            ],
            Limits {
                slots: 2,
                name_bytes: 8,
                reply_bytes: 70_000,
            },
        )
        .unwrap();
        let target_id = ic_platform_types::Principal::from_text(target.to_text()).unwrap();
        assert_eq!(target_id.as_slice(), target.as_slice());
        let wasm = catalog.compile(target_id, 50_000_000_000, 100_000).unwrap();
        pic.add_cycles(target, 100_000_000_000_000);
        pic.install_canister(target, wasm, vec![], None);
    }
    let sender = "2vxsx-fae".parse().unwrap();
    let mut messages = vec![];
    for (index, target) in [a, a, b, b].into_iter().enumerate() {
        messages.push(pic.submit_call(target, sender, "hold", vec![]).unwrap());
        for _ in 0..20 {
            if pic.get_canister_http().len() == index + 1 {
                break;
            }
            pic.tick();
        }
        assert_eq!(pic.get_canister_http().len(), index + 1);
    }
    let mut pending = pic.get_canister_http();
    pending.sort_by_key(|request| request.request_id);
    let mut urls: Vec<_> = pending.iter().map(|request| request.url.clone()).collect();
    urls.sort();
    let mut expected: Vec<_> = [a, b]
        .into_iter()
        .flat_map(|target| {
            (0..2).map(move |slot| format!("https://ic-testkit.invalid/held/{target}/{slot}"))
        })
        .collect();
    expected.sort();
    assert_eq!(urls, expected);
    for request in pending.into_iter().rev() {
        assert_eq!(request.http_method, CanisterHttpMethod::GET);
        assert_eq!(request.max_response_bytes, Some(256));
        assert!(request.headers.is_empty());
        assert_eq!(
            request.replication,
            CanisterHttpReplication::FullyReplicated
        );
        assert_eq!(request.pricing_version, CanisterHttpPricingVersion::Legacy);
        let slot = if request.url.ends_with("/0") {
            0_u64
        } else {
            1_u64
        };
        assert_eq!(request.body, slot.to_le_bytes());
        let response = if request.url == format!("https://ic-testkit.invalid/held/{b}/1") {
            CanisterHttpResponse::CanisterHttpReject(CanisterHttpReject {
                reject_code: 4,
                message: "deliberate rejection".into(),
            })
        } else {
            CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
                status: 200,
                headers: vec![],
                body: b"not application bytes".to_vec(),
            })
        };
        pic.mock_canister_http_response(MockCanisterHttpResponse {
            subnet_id: request.subnet_id,
            request_id: request.request_id,
            response,
            additional_responses: vec![],
        });
    }
    let mut messages = messages.into_iter();
    assert_eq!(pic.await_call(messages.next().unwrap()).unwrap(), page);
    assert_eq!(
        pic.await_call(messages.next().unwrap()).unwrap(),
        b"second-a"
    );
    assert_eq!(
        pic.await_call(messages.next().unwrap()).unwrap(),
        b"first-b"
    );
    assert!(pic.await_call(messages.next().unwrap()).is_err());
    for target in [a, b] {
        assert!(pic.update_call(target, sender, "hold", vec![]).is_err());
    }
    assert!(pic.get_canister_http().is_empty());
    // PocketIC's synchronous owner deletes its instance; the outer launcher owns
    // server shutdown. This test never changes simulated time or enables progress.
}

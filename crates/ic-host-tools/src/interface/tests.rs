use super::*;

use ic_host_artifacts::wasm::{Export, InspectionLimits, inspect};
use std::collections::BTreeSet;

#[test]
fn projection_preserves_names_modes_and_internal_exports_without_acceptance_policy() {
    // This fixture tests projection over structural facts, not Wasm validation.
    let mut facts = inspect(
        b"\0asm\x01\0\0\0",
        InspectionLimits {
            module_bytes: 8,
            sections: 0,
            exports: 0,
            custom_sections: 0,
        },
    )
    .unwrap();
    for (name, kind) in [
        ("canister_query read with spaces", ExportKind::Function),
        ("canister_update read with spaces", ExportKind::Function),
        ("canister_composite_query composed", ExportKind::Function),
        (
            "canister_update <ic-cdk internal> timer_executor",
            ExportKind::Function,
        ),
        ("canister_query ", ExportKind::Function),
        ("canister_query not_a_function", ExportKind::Global),
        ("canister_init", ExportKind::Function),
        ("canister_query", ExportKind::Function),
        ("get_candid_pointer", ExportKind::Function),
    ] {
        facts.exports.insert(name, Export { kind, index: 0 });
    }
    let methods = exported_methods(&facts).collect::<BTreeSet<_>>();
    assert_eq!(
        methods,
        BTreeSet::from([
            ExportedMethod {
                name: "read with spaces",
                mode: MethodMode::Query
            },
            ExportedMethod {
                name: "read with spaces",
                mode: MethodMode::Update
            },
            ExportedMethod {
                name: "composed",
                mode: MethodMode::CompositeQuery
            },
            ExportedMethod {
                name: "<ic-cdk internal> timer_executor",
                mode: MethodMode::Update
            },
            ExportedMethod {
                name: "",
                mode: MethodMode::Query
            },
        ])
    );
}

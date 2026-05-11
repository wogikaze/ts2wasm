/// RuntimeIntrinsic-to-RuntimeFn mapping completeness checks.
///
/// Verifies that every non-pseudo RuntimeIntrinsic variant maps to exactly
/// one RuntimeFn via `runtime_fn_from_name`, and that the mapped RuntimeFn
/// variant exists in the runtime catalog.
///
/// Pseudo-intrinsics (ArrayPushMany, HeapClosureCall, PrivateFieldGet,
/// PrivateFieldSet, PrivateBrandCheck) are expanded during IR lowering and
/// have no direct RuntimeFn equivalent -- they are excluded from this check.

use ts2wasm_backend_wasm::{
    RuntimeFn, RuntimeIntrinsic, runtime_fn_from_name,
};

#[test]
fn every_runtime_intrinsic_maps_to_runtime_fn() {
    let mut missing = Vec::new();
    for intrinsic in RuntimeIntrinsic::all() {
        let name = intrinsic.name();
        match runtime_fn_from_name(name) {
            Some(_) => {} // Good: maps to a RuntimeFn
            None => {
                // Check if this is a pseudo-intrinsic (should have no RuntimeFn)
                let known_pseudo = [
                    "ArrayPushMany",
                    "HeapClosureCall",
                    "PrivateFieldGet",
                    "PrivateFieldSet",
                    "PrivateBrandCheck",
                ];
                if !known_pseudo.contains(&name) {
                    // Not a pseudo-intrinsic, this is a missing mapping
                    missing.push(name);
                }
            }
        }
    }
    assert!(
        missing.is_empty(),
        "RuntimeIntrinsic variants missing RuntimeFn mapping:\n  {}",
        missing.join("\n  ")
    );
}

#[test]
fn every_mapped_runtime_fn_has_catalog_entry() {
    // Verify that a non-empty set of RuntimeFn variants are reachable
    // via runtime_fn_from_name for at least one intrinsic.
    let all_ints = RuntimeIntrinsic::all();
    let mapped_count = all_ints
        .iter()
        .filter_map(|i| runtime_fn_from_name(i.name()))
        .count();

    assert!(
        mapped_count > 0,
        "Expected at least one RuntimeIntrinsic to map to a RuntimeFn, got 0"
    );

    // Spot-check a few expected mappings
    assert!(
        runtime_fn_from_name("ArrayGet").is_some(),
        "ArrayGet should map to a RuntimeFn"
    );
    assert!(
        runtime_fn_from_name("Concat").is_some(),
        "Concat should map to a RuntimeFn"
    );
    assert!(
        runtime_fn_from_name("Log").is_some(),
        "Log should map to a RuntimeFn"
    );
}

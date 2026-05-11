/// RuntimeFn mapping completeness checks.
///
/// Verifies that every non-pseudo RuntimeFn variant maps to exactly
/// one RuntimeFn via `runtime_fn_from_name`.
///
/// Pseudo-intrinsics (ArrayPushMany, HeapClosureCall, PrivateFieldGet,
/// PrivateFieldSet, PrivateBrandCheck) are expanded during IR lowering and
/// have no direct WAT function entry -- they are included in the catalog
/// but don't need to map through `runtime_fn_from_name`.
use ts2wasm_backend_wasm::{RuntimeFn, runtime_fn_from_name};

#[test]
fn every_runtime_fn_maps_to_runtime_fn_from_name() {
    let mut missing = Vec::new();
    for variant in RuntimeFn::emission_order() {
        let name = format!("{:?}", variant);
        match runtime_fn_from_name(&name) {
            Some(_) => {} // Good: maps to a RuntimeFn
            None => {
                // Check if this is a pseudo-intrinsic (not registered in from_name)
                let known_pseudo = [
                    "ArrayPushMany",
                    "HeapClosureCall",
                    "PrivateFieldGet",
                    "PrivateFieldSet",
                    "PrivateBrandCheck",
                ];
                if !known_pseudo.contains(&name.as_str()) {
                    missing.push(name);
                }
            }
        }
    }
    // Pseudo-intrinsics in the emission order are expected not to map
    // through runtime_fn_from_name since they have no standalone WAT entry.
    assert!(
        missing.is_empty(),
        "RuntimeFn variants missing runtime_fn_from_name mapping:\n  {}",
        missing.join("\n  ")
    );
}

#[test]
fn some_mapped_runtime_fn_reachable() {
    let mapped_count = RuntimeFn::emission_order()
        .iter()
        .filter_map(|i| runtime_fn_from_name(&format!("{:?}", i)))
        .count();

    assert!(
        mapped_count > 0,
        "Expected at least one RuntimeFn to map, got 0"
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

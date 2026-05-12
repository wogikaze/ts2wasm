/// Runtime signature tests: verify that `RuntimeFn::stack_effect()` produces
/// self-consistent results and that the `check_runtime_signature` helper
/// correctly validates runtime calls against declared signatures.
///
/// These tests are the backend-side complement of the catalog-level
/// `RuntimeSignature` definition in `ts2wasm_runtime_catalog::signature`.
use ts2wasm_backend_wasm::wasm_ir::check_runtime_signature;
use ts2wasm_runtime_abi::StackEffect;
use ts2wasm_runtime_catalog::{RuntimeFn, RuntimeSignature};

// ---------------------------------------------------------------------------
// Smoke test: well-known runtime signatures
// ---------------------------------------------------------------------------

#[test]
fn truthy_bool_smoke() {
    let sig = RuntimeFn::TruthyBool.stack_effect();
    assert_eq!(sig.params, 1);
    assert_eq!(sig.results, 1);
}

#[test]
fn private_brand_type_error_smoke() {
    let sig = RuntimeFn::PrivateBrandTypeError.stack_effect();
    assert_eq!(sig.params, 0);
    assert_eq!(sig.results, 1);
}

#[test]
fn array_get_smoke() {
    let sig = RuntimeFn::ArrayGet.stack_effect();
    assert_eq!(sig.params, 2);
    assert_eq!(sig.results, 1);
}

#[test]
fn property_get_smoke() {
    let sig = RuntimeFn::PropertyGet.stack_effect();
    assert_eq!(sig.params, 3);
    assert_eq!(sig.results, 1);
}

#[test]
fn property_set_smoke() {
    let sig = RuntimeFn::PropertySet.stack_effect();
    assert_eq!(sig.params, 4);
    assert_eq!(sig.results, 1);
}

// ---------------------------------------------------------------------------
// RuntimeSignature from signature module is the same type
// ---------------------------------------------------------------------------

#[test]
fn runtime_signature_new() {
    let sig = RuntimeSignature::new(2, 1);
    assert_eq!(sig.params, 2);
    assert_eq!(sig.results, 1);
}

#[test]
fn runtime_signature_take_n_return_one() {
    let sig = RuntimeSignature::take_n_return_one(3);
    assert_eq!(sig.params, 3);
    assert_eq!(sig.results, 1);
}

#[test]
fn runtime_signature_take_one_return_one() {
    let sig = RuntimeSignature::take_one_return_one();
    assert_eq!(sig.params, 1);
    assert_eq!(sig.results, 1);
}

// ---------------------------------------------------------------------------
// StackEffect from runtime-abi
// ---------------------------------------------------------------------------

#[test]
fn stack_effect_take_one_return_one() {
    let e = StackEffect::take_one_return_one();
    assert_eq!(e.params, 1);
    assert_eq!(e.results, 1);
}

#[test]
fn stack_effect_property_get() {
    let e = StackEffect::property_get();
    assert_eq!(e.params, 3);
    assert_eq!(e.results, 1);
}

#[test]
fn stack_effect_property_set() {
    let e = StackEffect::property_set();
    assert_eq!(e.params, 4);
    assert_eq!(e.results, 1);
}

// ---------------------------------------------------------------------------
// check_runtime_signature helper
// ---------------------------------------------------------------------------

#[test]
fn check_runtime_signature_truthy_bool() {
    check_runtime_signature(RuntimeFn::TruthyBool, 1, 1);
}

#[test]
fn check_runtime_signature_array_get() {
    check_runtime_signature(RuntimeFn::ArrayGet, 2, 1);
}

#[test]
fn check_runtime_signature_property_set() {
    check_runtime_signature(RuntimeFn::PropertySet, 4, 1);
}

#[test]
#[should_panic(expected = "expected 2 params, declared 1")]
fn check_runtime_signature_mismatch_panics() {
    // TruthyBool takes 1 param, but we say 2 -- should panic
    check_runtime_signature(RuntimeFn::TruthyBool, 2, 1);
}

// ---------------------------------------------------------------------------
// Round-trip: RuntimeSignature and StackEffect share the same shape
// ---------------------------------------------------------------------------

#[test]
fn runtime_signature_round_trip() {
    // Verify that StackEffect and RuntimeSignature agree on key accessors
    let sig = RuntimeSignature::new(1, 1);
    let eff = StackEffect::take_one_return_one();
    assert_eq!(sig.params, eff.params);
    assert_eq!(sig.results, eff.results);
}

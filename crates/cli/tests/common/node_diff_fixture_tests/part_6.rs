use super::*;

#[test]
fn class_fields_methods_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes-and-inheritance/class-fields-methods.ts");
}

#[test]
fn class_super_static_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-super-static.ts");
}

#[test]
fn class_static_fields_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-static-fields.ts");
}

#[test]
fn class_private_members_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-private-members.ts");
}

#[test]
fn class_extends_builtin_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-extends-builtin.ts");
}

// Class coverage expansion (issue I-20260514-YHDZJJ)
#[test]
fn class_constructor_return_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-constructor-return.ts");
}

#[test]
fn class_getter_setter_inherited_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-getter-setter-inherited.ts");
}

#[test]
fn class_instanceof_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-instanceof.ts");
}

#[test]
fn class_method_override_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-method-override.ts");
}

#[test]
fn class_private_fields_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/classes/class-private-fields.ts");
}

// FNCSEM: function and method call semantic suite (issue I-20260512-FNCSEM)

#[test]
fn fncsem_v2_call_extra_args_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/call-extra-args.ts");
}

#[test]
fn fncsem_call_fewer_args_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/call-fewer-args.ts");
}

#[test]
fn fncsem_call_arity_mismatch_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/call-arity-mismatch.ts");
}

#[test]
fn fncsem_call_extra_args_reject_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/call-extra-args-reject.ts");
}

#[test]
fn fncsem_v2_call_fewer_args_reject_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/call-fewer-args-reject.ts");
}

#[test]
fn fncsem_v2_method_receiver_preserve_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/method-call-receiver-preserve.ts");
}

#[test]
fn fncsem_v2_builtin_call_hir_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/builtin-call-hir.ts");
}

#[test]
fn fncsem_dynamic_call_assign_matches_node_output() {
    assert_fixture_matches_node("fixtures/core-semantics/dynamic-call-assign-unsupported.ts");
}

// FNCSEM: semantic call test fixture suite (fixtures/semantic/functions/)

#[test]
fn fncsem_v2_argument_count_edges_matches_node_output() {
    assert_fixture_matches_node("fixtures/semantic/functions/argument-count-edges.ts");
}

#[test]
fn fncsem_computed_call_unsupported_reports_unsupported_syntax() {
    assert_build_fails_with_unsupported_syntax(
        "fixtures/semantic/functions/dynamic-call-unsupported.ts",
        "only identifier calls are supported",
    );
}

#[test]
fn string_static_methods_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-static-methods.ts");
}

// Object.prototype methods
#[test]
fn object_prototype_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/object-prototype.ts");
}

// Array.prototype copying methods
#[test]
fn array_copying_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/array-copying-methods.ts");
}

#[test]
fn string_static_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/string-static.ts");
}

// Error subclasses with stack and cause support
#[test]
fn error_stack_cause_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/error-subclasses.ts");
}

// Set algebraic methods (isDisjointFrom, isSubsetOf, isSupersetOf, union, intersection, difference, symmetricDifference)
#[test]
fn set_algebra_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/set-algebra.ts");
}

#[test]
fn date_methods_comprehensive_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/date-methods-comprehensive.ts");
}

// URI encode/decode/conformance (encodeURI, decodeURI, encodeURIComponent, decodeURIComponent)
#[test]
fn global_encode_uri_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-encode-uri.ts");
}

#[test]
fn global_decode_uri_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-decode-uri.ts");
}

#[test]
fn global_uri_component_fixture_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/global-uri-component.ts");
}

#[test]
fn strict_mode_basic_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/strict-mode-basic.ts");
}

#[test]
fn weakref_finalization_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/weakref-finalization.ts");
}

#[test]
fn reflect_apply_construct_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/reflect-apply-construct.ts");
}

#[test]
fn builtin_constructor_descriptors_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/builtin-constructor-descriptors.ts");
}

#[test]
fn reflect_construct_is_constructor_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/reflect-construct-is-constructor.ts");
}

#[test]
fn dynamic_eval_host_path_matches_node_output() {
    assert_fixture_matches_node("fixtures/builtins-and-io/dynamic-eval-host-path.ts");
}

#[test]
fn module_exports_assign_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/modules-and-typed-optimizations/module-exports-assign.ts",
    );
}

// Common builtin API coverage (issue I-20260514-HGZJCJ)
#[test]
fn common_builtin_api_fixtures_match_node_output() {
    for fixture in [
        "fixtures/builtins-and-io/boolean-symbol-prototype.ts",
        "fixtures/builtins-and-io/native-error-types.ts",
        "fixtures/builtins-and-io/strict-mode-basic.ts",
        "fixtures/builtins-and-io/number-format-integer.ts",
        "fixtures/builtins-and-io/number-format-no-args.ts",
        "fixtures/builtins-and-io/number-format-precision.ts",
        "fixtures/builtins-and-io/string-constructor-call.ts",
        "fixtures/builtins-and-io/reflect-basic.ts",
        "fixtures/builtins-and-io/array-every-some-complex.ts",
    ] {
        assert_fixture_matches_node(fixture);
    }
}

// GC pressure: private fields surviving allocation pressure
#[test]
fn private_field_gc_pressure_matches_node_output() {
    assert_fixture_matches_node("fixtures/object-semantics-kernel/private-field-gc-pressure.ts");
}

// GC pressure: returned closure capturing an object containing an array,
// invoked after allocation pressure triggers GC
#[test]
fn returned_closure_nested_object_gc_pressure_matches_node_output() {
    assert_fixture_matches_node(
        "fixtures/core-semantics/returned-closure-nested-object-gc-pressure.ts",
    );
}

// covers: I-20260515-PMTJTQ
// Comma expression statement in statement position after assignment.
#[test]
fn comma_expression_statement_matches_node_output() {
    assert_fixture_matches_node("fixtures/parser/comma-expression-statement.ts");
}

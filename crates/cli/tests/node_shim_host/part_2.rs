use super::*;

#[test]
fn dynamic_direct_eval_applies_writeback_before_throw_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-throw-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\nafter\n");
}

#[test]
fn dynamic_direct_eval_applies_created_binding_before_throw_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-throw-created-binding-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\nafter\n");
}

#[test]
fn dynamic_direct_eval_applies_created_function_before_throw_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-throw-created-function-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\nafter\n");
}

#[test]
fn dynamic_direct_eval_created_function_before_throw_is_visible_to_normal_code_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-throw-created-function-normal-code-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\nafter\n");
}

#[test]
fn dynamic_direct_eval_strict_lexical_shadow_does_not_write_back_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-lexical-shadow-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n1\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_var_stays_eval_local_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-strict-caller-var-local-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nundefined\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_delete_identifier_is_syntax_error_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-caller-delete-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\n1\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_delete_arguments_is_syntax_error_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-strict-caller-delete-arguments-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\n9\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_var_arguments_is_syntax_error_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-strict-caller-var-arguments-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\n9\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_array_binding_arguments_is_syntax_error_node_shim_host_import()
{
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-caller-array-binding-arguments-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\n9\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_object_binding_eval_is_syntax_error_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-caller-object-binding-eval-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\nafter\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_function_eval_is_syntax_error_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-strict-caller-function-eval-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\nafter\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_async_function_eval_is_syntax_error_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-caller-async-function-eval-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\nafter\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_ignores_restricted_words_in_strings_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-caller-string-restricted-words-node-shim.ts";
    assert_node_shim_stdout(fixture, "var arguments\nafter\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_ignores_restricted_words_in_regexp_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-caller-regexp-restricted-words-node-shim.ts";
    assert_node_shim_stdout(fixture, "var arguments\nafter\n");
}

#[test]
fn dynamic_direct_eval_strict_caller_ignores_restricted_words_in_keyword_regexp_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-strict-caller-regexp-after-keyword-node-shim.ts";
    assert_node_shim_stdout(fixture, "true\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_env_descriptor_conflict_is_catchable_reference_error() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-conflict-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_template_expression_reference_is_catchable_reference_error() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-tdz-template-expression-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_typeof_reference_is_catchable_reference_error() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-typeof-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_parenthesized_reference_is_catchable_reference_error() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-parenthesized-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_member_reference_is_catchable_reference_error() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-member-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_computed_member_reference_is_catchable_reference_error() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-computed-member-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_optional_member_reference_is_catchable_reference_error() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-optional-member-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_tdz_optional_computed_member_reference_is_catchable_reference_error() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-tdz-optional-computed-member-node-shim.ts";
    assert_node_shim_stdout(fixture, "ReferenceError\nafter\n");
}

#[test]
fn dynamic_direct_eval_function_expression_name_is_not_predeclared_for_normal_code() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-function-expression-name-normal-code-unsupported.ts";
    assert_build_fails_with(fixture, "UnresolvedName", "unresolved name: `hiddenProbe`");
}

#[test]
fn dynamic_direct_eval_nested_function_body_var_is_not_predeclared_for_normal_code() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-nested-function-var-normal-code-unsupported.ts";
    assert_build_fails_with(fixture, "UnresolvedName", "unresolved name: `hidden`");
}

#[test]
fn dynamic_direct_eval_arrow_body_var_is_not_predeclared_for_normal_code() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-arrow-body-var-normal-code-unsupported.ts";
    assert_build_fails_with(fixture, "UnresolvedName", "unresolved name: `hidden`");
}

#[test]
fn dynamic_direct_eval_object_method_var_is_not_predeclared_for_normal_code() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-object-method-var-normal-code-unsupported.ts";
    assert_build_fails_with(fixture, "UnresolvedName", "unresolved name: `hidden`");
}

#[test]
fn static_function_constructor_rejects_parameter_wrapper_injection() {
    let fixture = "fixtures/core-semantics/function-constructor-parameter-injection-unsupported.ts";
    assert_build_fails_with(
        fixture,
        "UnsupportedSyntax",
        "Function constructor parameters must parse as a single FormalParameters list",
    );
}

#[test]
fn static_function_constructor_unary_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-unary-source.ts";
    assert_node_shim_stdout(
        fixture,
        "undefined\nundefined\nfunction anonymous(\n) {\n-1\n}\nundefined\nfunction anonymous(\n) {\nundefined\n}\nundefined\nfunction anonymous(\n) {\ntrue\n}\n",
    );
}

#[test]
fn static_function_constructor_decimal_unary_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-decimal-unary-source.ts";
    assert_node_shim_stdout(
        fixture,
        "undefined\nundefined\nfunction anonymous(\n) {\n-1.5\n}\nfunction anonymous(\n) {\n2.5\n}\n",
    );
}

#[test]
fn static_function_constructor_decimal_expression_source_uses_aot_lane() {
    let fixture =
        "fixtures/core-semantics/function-constructor-static-decimal-expression-source.ts";
    assert_node_shim_stdout(
        fixture,
        "undefined\n4\nfunction anonymous(\n) {\n4\n}\nfunction anonymous(\n) {\nreturn 4\n}\n",
    );
}

#[test]
fn static_function_constructor_comparison_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-comparison-source.ts";
    assert_node_shim_stdout(
        fixture,
        "undefined\nundefined\nundefined\nundefined\nundefined\nundefined\nfunction anonymous(\n) {\ntrue\n}\nfunction anonymous(\n) {\nfalse\n}\nfunction anonymous(\n) {\ntrue\n}\nfunction anonymous(\n) {\ntrue\n}\nfunction anonymous(\n) {\nfalse\n}\nfunction anonymous(\n) {\ntrue\n}\n",
    );
}

#[test]
fn static_function_constructor_bitwise_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-bitwise-source.ts";
    assert_node_shim_stdout(
        fixture,
        "undefined\nundefined\nundefined\nundefined\nundefined\nundefined\nundefined\nundefined\nfunction anonymous(\n) {\n1\n}\nfunction anonymous(\n) {\n7\n}\nfunction anonymous(\n) {\n4\n}\nfunction anonymous(\n) {\n8\n}\nfunction anonymous(\n) {\n4\n}\nfunction anonymous(\n) {\n2147483647\n}\nfunction anonymous(\n) {\n-6\n}\nfunction anonymous(\n) {\n1\n}\n",
    );
}

#[test]
fn static_function_constructor_typeof_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-typeof-source.ts";
    assert_node_shim_stdout(
        fixture,
        "number\nstring\nboolean\nundefined\nobject\nobject\nbigint\nfunction anonymous(\n) {\nreturn 'number'\n}\nfunction anonymous(\n) {\nreturn 'string'\n}\nfunction anonymous(\n) {\nreturn 'boolean'\n}\nfunction anonymous(\n) {\nreturn 'undefined'\n}\nfunction anonymous(\n) {\nreturn 'object'\n}\nfunction anonymous(\n) {\nreturn 'object'\n}\nfunction anonymous(\n) {\nreturn 'bigint'\n}\n",
    );
}

#[test]
fn static_function_constructor_sequence_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-sequence-source.ts";
    assert_node_shim_stdout(
        fixture,
        "9\n7\nundefined\nfunction anonymous(\n) {\nreturn 9\n}\nfunction anonymous(value\n) {\nreturn value + 2\n}\nfunction anonymous(\n) {\n3\n}\n",
    );
}

#[test]
fn static_function_constructor_numeric_binary_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-numeric-binary-source.ts";
    assert_node_shim_stdout(
        fixture,
        "undefined\nundefined\nundefined\nundefined\nundefined\nundefined\nundefined\nfunction anonymous(\n) {\n6\n}\nfunction anonymous(\n) {\n3\n}\nfunction anonymous(\n) {\n2.5\n}\nfunction anonymous(\n) {\n3\n}\nfunction anonymous(\n) {\n8\n}\nfunction anonymous(\n) {\n3\n}\nfunction anonymous(\n) {\n0\n}\n",
    );
}

#[test]
fn static_function_constructor_string_unary_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-string-unary-source.ts";
    assert_node_shim_stdout(
        fixture,
        "undefined\nundefined\nundefined\nfunction anonymous(\n) {\n2.5\n}\nfunction anonymous(\n) {\n-1.5\n}\nfunction anonymous(\n) {\n0\n}\n",
    );
}

#[test]
fn static_function_constructor_expression_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-expression-source.ts";
    assert_node_shim_stdout(fixture, "1\n2\nundefined\nfunction anonymous(\n) {\n3\n}\n");
}

#[test]
fn static_function_constructor_array_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-array-source.ts";
    assert_node_shim_stdout(
        fixture,
        "7\nx,y\nundefined\nfunction anonymous(\n) {\nconsole.log(\"x,y\")\n}\nundefined\nfunction anonymous(\n) {\n\n}\n",
    );
}

#[test]
fn static_function_constructor_spread_array_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-spread-array-source.ts";
    assert_node_shim_stdout(
        fixture,
        "11\n13\n5\nfunction anonymous(value\n) {\nreturn value + 1\n}\n",
    );
}

#[test]
fn static_function_constructor_ternary_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-ternary-source.ts";
    assert_node_shim_stdout(fixture, "1\n2\nundefined\nfunction anonymous(\n) {\n3\n}\n");
}

#[test]
fn static_function_constructor_logical_source_uses_aot_lane() {
    let fixture = "fixtures/core-semantics/function-constructor-static-logical-source.ts";
    assert_node_shim_stdout(
        fixture,
        "2\n3\n4\n5\nundefined\nfunction anonymous(\n) {\nfalse\n}\n",
    );
}

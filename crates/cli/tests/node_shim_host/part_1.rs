use super::*;

#[test]
fn dynamic_function_handles_execute_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n");
}

#[test]
fn dynamic_function_handle_returns_string_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-string-node-shim.ts";
    assert_node_shim_stdout(fixture, "dynamic-string\n");
}

#[test]
fn dynamic_function_handle_returns_object_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-object-node-shim.ts";
    assert_node_shim_stdout(fixture, "[object Object]\n");
}

#[test]
fn dynamic_function_handle_preserves_object_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-object-properties-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_function_handle_bridges_function_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-function-property-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\nhostCallback\n[object Object]\nundefined\n");
}

#[test]
fn dynamic_function_handle_calls_function_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-function-property-call-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_function_handle_binds_this_for_function_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-function-property-this-node-shim.ts";
    assert_node_shim_stdout(fixture, "9\n");
}

#[test]
fn dynamic_function_handle_calls_computed_function_properties_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-computed-function-property-call-node-shim.ts";
    assert_node_shim_stdout(fixture, "10\n");
}

#[test]
fn dynamic_function_handle_bridges_thrown_errors_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-throw-catch-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\nfunction boom\n");
}

#[test]
fn dynamic_function_handle_bridges_thrown_object_methods_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-throw-object-method-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\n7\n");
}

#[test]
fn dynamic_function_compile_bridges_syntax_errors_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-syntax-error-catch-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\n");
}

#[test]
fn dynamic_function_compile_flattens_spread_array_args_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-spread-array-node-shim.ts";
    assert_node_shim_stdout(fixture, "12\n");
}

#[test]
fn dynamic_json_parse_and_stringify_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/json-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "1\nok\n{\"n\":1,\"label\":\"ok\"}\n");
}

#[test]
fn iterator_helpers_execute_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/iterator-helpers-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n0\n");
}

#[test]
fn iterator_helper_callbacks_execute_with_native_lowering() {
    let fixture = "fixtures/core-semantics/iterator-helpers-callback-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n6\n8\n6\n2\n6\n8\n20\n");
}

#[test]
fn dynamic_function_sequence_prefix_preserves_side_effect_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-sequence-prefix-node-shim.ts";
    assert_node_shim_stdout(fixture, "1\n7\n");
}

#[test]
fn dynamic_function_handle_preserves_object_identity_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-object-identity-node-shim.ts";
    assert_node_shim_stdout(fixture, "true\n7\n");
}

#[test]
fn dynamic_function_handle_refreshes_object_properties_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-object-mutation-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\ntrue\n3\n3\n");
}

#[test]
fn dynamic_function_handle_tracks_object_shape_changes_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-object-shape-node-shim.ts";
    assert_node_shim_stdout(fixture, "1\ntrue\n2\nok\nundefined\n");
}

#[test]
fn dynamic_function_handle_grows_object_shape_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-object-growth-node-shim.ts";
    assert_node_shim_stdout(fixture, "1\n5\nundefined\n");
}

#[test]
fn dynamic_function_handle_grows_existing_object_references_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-object-existing-ref-growth-node-shim.ts";
    assert_node_shim_stdout(fixture, "1\n5\nundefined\n");
}

#[test]
fn dynamic_function_handle_bridges_nested_arrays_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-nested-array-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n7\n8\nundefined\n");
}

#[test]
fn dynamic_function_handle_calls_array_function_elements_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-array-function-element-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n12\n");
}

#[test]
fn dynamic_function_handle_bridges_nested_objects_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-nested-object-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_function_handle_preserves_optional_nested_object_method_through_node_shim_host_imports()
{
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-optional-nested-method-node-shim.ts";
    assert_node_shim_stdout(fixture, "10\n");
}

#[test]
fn dynamic_function_optional_computed_nested_method_uses_host_call_method() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-optional-computed-nested-method-node-shim.ts";
    assert_node_shim_stdout(fixture, "11\n");
}

#[test]
fn dynamic_function_handle_grows_nested_arrays_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-array-growth-node-shim.ts";
    assert_node_shim_stdout(fixture, "4\n5\n5\nundefined\n");
}

#[test]
fn dynamic_function_handle_grows_existing_array_references_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-array-existing-ref-growth-node-shim.ts";
    assert_node_shim_stdout(fixture, "5\n5\nundefined\n");
}

#[test]
fn dynamic_function_handle_exposes_metadata_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-metadata-node-shim.ts";
    assert_node_shim_stdout(
        fixture,
        "2\nanonymous\n[object Object]\nfunction anonymous(a,b\n) {\nreturn a + b\n}\n7\n",
    );
}

#[test]
fn dynamic_function_handle_calls_computed_tostring_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-computed-tostring-node-shim.ts";
    assert_node_shim_stdout(fixture, "function anonymous(a,b\n) {\nreturn a + b\n}\n");
}

#[test]
fn dynamic_function_handle_preserves_prototype_constructor_identity_through_node_shim_host_imports()
{
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-prototype-identity-node-shim.ts";
    assert_node_shim_stdout(fixture, "true\nanonymous\n");
}

#[test]
fn dynamic_function_handle_calls_prototype_constructor_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-prototype-constructor-call-node-shim.ts";
    assert_node_shim_stdout(fixture, "anonymous:2\n");
}

#[test]
fn dynamic_function_construct_returns_object_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-construct-object-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_indirect_eval_executes_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n");
}

#[test]
fn dynamic_optional_eval_executes_as_indirect_eval_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/optional-eval-dynamic-node-shim.ts";
    assert_node_shim_stdout(fixture, "optional-eval\n");
}

#[test]
fn test262_eval_script_routes_to_indirect_eval_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/test262-eval-script-node-shim.ts";
    assert_node_shim_stdout(fixture, "3\n");
}

#[test]
fn test262_eval_script_bridges_thrown_object_methods_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/test262-eval-script-throw-object-method-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\n7\n");
}

#[test]
fn new_eval_throws_type_error() {
    let fixture = "fixtures/core-semantics/new-eval-type-error.ts";
    assert_node_shim_stdout(fixture, "TypeError\n");
}

#[test]
fn dynamic_indirect_eval_preserves_object_properties_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-object-properties-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_indirect_eval_bridges_thrown_errors_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-throw-catch-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\nindirect boom\n");
}

#[test]
fn dynamic_indirect_eval_bridges_thrown_object_methods_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/indirect-eval-dynamic-throw-object-method-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\n7\n");
}

#[test]
fn dynamic_direct_eval_executes_through_node_shim_host_import() {
    let fixture = "fixtures/builtins-and-io/dynamic-eval-host-path.ts";
    assert_node_shim_stdout(fixture, "3\n");
}

#[test]
fn dynamic_direct_eval_runtime_source_keeps_following_statements() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-host-path.ts";
    assert_node_shim_stdout(fixture, "unreachable\n");
}

#[test]
fn static_direct_eval_lands_var_and_function_declarations() {
    let fixture = "fixtures/core-semantics/direct-eval-var-function-declaration.ts";
    assert_node_shim_stdout(fixture, "1\n");
}

#[test]
fn static_direct_eval_function_var_is_visible_after_eval() {
    let fixture = "fixtures/core-semantics/direct-eval-function-var-visible-after-eval.ts";
    assert_node_shim_stdout(fixture, "2\n2\n");
}

#[test]
fn static_direct_eval_block_var_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-block-var-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "2\n2\n");
}

#[test]
fn static_direct_eval_if_var_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-if-var-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "2\n2\n");
}

#[test]
fn static_direct_eval_if_var_is_hoisted_as_undefined() {
    let fixture = "fixtures/core-semantics/direct-eval-if-var-hoisted-undefined.ts";
    assert_node_shim_stdout(fixture, "undefined\nundefined\n");
}

#[test]
fn static_direct_eval_while_var_is_hoisted_as_undefined() {
    let fixture = "fixtures/core-semantics/direct-eval-while-var-hoisted-undefined.ts";
    assert_node_shim_stdout(fixture, "undefined\nundefined\n");
}

#[test]
fn static_direct_eval_while_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-while-completion.ts";
    assert_node_shim_stdout(fixture, "3\n3\n");
}

#[test]
fn static_direct_eval_do_while_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-do-while-completion.ts";
    assert_node_shim_stdout(fixture, "3\n3\n");
}

#[test]
fn static_direct_eval_for_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-for-completion.ts";
    assert_node_shim_stdout(fixture, "2\n3\n");
}

#[test]
fn static_direct_eval_var_destructuring_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-var-destructuring-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "6:7:8:1\n6\n7\n8\n1\n");
}

#[test]
fn static_direct_eval_var_destructuring_computed_rest_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-var-destructuring-computed-rest-caller.ts";
    assert_node_shim_stdout(fixture, "1:ok:undefined\n1\nok\nundefined\n");
}

#[test]
fn static_direct_eval_var_destructuring_hoists_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-var-destructuring-hoist-caller.ts";
    assert_node_shim_stdout(fixture, "undefined\nundefined\n");
}

#[test]
fn static_direct_eval_for_head_var_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-for-head-var-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "alpha:4\nalpha\n4\n");
}

#[test]
fn static_direct_eval_for_head_var_destructuring_lands_in_caller_scope() {
    let fixture =
        "fixtures/core-semantics/direct-eval-for-head-var-destructuring-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "6:8:1\n6\n8\n1\n");
}

#[test]
fn static_direct_eval_for_head_var_destructuring_default_reads_caller_scope() {
    let fixture =
        "fixtures/core-semantics/direct-eval-for-head-var-destructuring-default-caller.ts";
    assert_node_shim_stdout(fixture, "fallback\nnext\n");
}

#[test]
fn static_direct_eval_for_head_var_destructuring_computed_key() {
    let fixture = "fixtures/core-semantics/direct-eval-for-head-var-destructuring-computed-key.ts";
    assert_node_shim_stdout(fixture, "ok\n");
}

#[test]
fn static_direct_eval_for_head_var_object_rest_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-for-head-var-object-rest-caller.ts";
    assert_node_shim_stdout(fixture, "ok:2:1\nok\n2\n1\n");
}

#[test]
fn static_direct_eval_for_head_var_object_rest_computed_key() {
    let fixture = "fixtures/core-semantics/direct-eval-for-head-var-object-rest-computed-key.ts";
    assert_node_shim_stdout(fixture, "1\nok\nundefined\n");
}

#[test]
fn static_direct_eval_for_init_var_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-for-init-var-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "1\n1\n");
}

#[test]
fn static_direct_eval_for_init_var_destructuring_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-for-init-var-destructuring-caller.ts";
    assert_node_shim_stdout(fixture, "6:8:1\n6\n8\n1\n");
}

#[test]
fn static_direct_eval_for_init_var_destructuring_computed_rest_lands_in_caller_scope() {
    let fixture =
        "fixtures/core-semantics/direct-eval-for-init-var-destructuring-computed-rest-caller.ts";
    assert_node_shim_stdout(fixture, "1:ok:undefined\n1\nok\nundefined\n");
}

#[test]
fn static_direct_eval_for_in_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-for-in-completion.ts";
    assert_node_shim_stdout(fixture, "ab\nab\n");
}

#[test]
fn static_direct_eval_for_of_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-for-of-completion.ts";
    assert_node_shim_stdout(fixture, "6\n6\n");
}

#[test]
fn static_indirect_eval_for_head_var_lands_on_global_object() {
    let fixture = "fixtures/core-semantics/indirect-eval-static-for-head-var-global.ts";
    assert_node_shim_stdout(fixture, "alpha:4\ncaller\ncaller\nalpha\n4\n");
}

#[test]
fn static_indirect_eval_var_destructuring_lands_on_global_object() {
    let fixture = "fixtures/core-semantics/indirect-eval-static-var-destructuring-global.ts";
    assert_node_shim_stdout(fixture, "6:7:8:1\ncaller\ncaller\ncaller\n6\n7\n8\n1\n");
}

#[test]
fn static_indirect_eval_var_destructuring_computed_rest_lands_on_global_object() {
    let fixture =
        "fixtures/core-semantics/indirect-eval-static-var-destructuring-computed-rest-global.ts";
    assert_node_shim_stdout(
        fixture,
        "1:ok:undefined\ncaller\ncaller\n1\nok\nundefined\n",
    );
}

#[test]
fn static_indirect_eval_var_destructuring_hoists_on_global_object() {
    let fixture = "fixtures/core-semantics/indirect-eval-static-var-destructuring-hoist-global.ts";
    assert_node_shim_stdout(fixture, "undefined\ncaller\nundefined\n");
}

#[test]
fn static_indirect_eval_for_head_var_destructuring_lands_on_global_object() {
    let fixture =
        "fixtures/core-semantics/indirect-eval-static-for-head-var-destructuring-global.ts";
    assert_node_shim_stdout(fixture, "6:8:1\ncaller\ncaller\ncaller\n6\n8\n1\n");
}

#[test]
fn static_indirect_eval_for_head_var_computed_lands_on_global_object() {
    let fixture = "fixtures/core-semantics/indirect-eval-static-for-head-var-computed-global.ts";
    assert_node_shim_stdout(fixture, "ok\ncaller\ncaller\nok\n");
}

#[test]
fn static_indirect_eval_for_head_var_object_rest_lands_on_global_object() {
    let fixture = "fixtures/core-semantics/indirect-eval-static-for-head-var-object-rest-global.ts";
    assert_node_shim_stdout(fixture, "ok:2:1\ncaller\ncaller\n1\nok\n2\n");
}

#[test]
fn static_indirect_eval_for_head_var_object_rest_computed_lands_on_global_object() {
    let fixture =
        "fixtures/core-semantics/indirect-eval-static-for-head-var-object-rest-computed-global.ts";
    assert_node_shim_stdout(
        fixture,
        "1:ok:undefined\ncaller\ncaller\ncaller\n1\nok\nundefined\n",
    );
}

#[test]
fn static_indirect_eval_for_init_var_lands_on_global_object() {
    let fixture = "fixtures/core-semantics/indirect-eval-static-for-init-var-global.ts";
    assert_node_shim_stdout(fixture, "1\ncaller\n1\n");
}

#[test]
fn static_indirect_eval_for_init_var_destructuring_lands_on_global_object() {
    let fixture =
        "fixtures/core-semantics/indirect-eval-static-for-init-var-destructuring-global.ts";
    assert_node_shim_stdout(fixture, "6:8:1\ncaller\ncaller\n6\n8\n1\n");
}

#[test]
fn static_indirect_eval_for_init_var_destructuring_computed_rest_lands_on_global_object() {
    let fixture = "fixtures/core-semantics/indirect-eval-static-for-init-var-destructuring-computed-rest-global.ts";
    assert_node_shim_stdout(
        fixture,
        "1:ok:undefined\ncaller\ncaller\n1\nok\nundefined\n",
    );
}

#[test]
fn static_direct_eval_switch_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-switch-completion.ts";
    assert_node_shim_stdout(fixture, "20\n20\n");
}

#[test]
fn static_direct_eval_try_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-try-completion.ts";
    assert_node_shim_stdout(fixture, "1\n2\n7\n8\n");
}

#[test]
fn static_direct_eval_labeled_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-labeled-completion.ts";
    assert_node_shim_stdout(fixture, "1\n1\n");
}

#[test]
fn static_direct_eval_expression_reads_caller_local() {
    let fixture = "fixtures/core-semantics/direct-eval-expression-caller-local.ts";
    assert_node_shim_stdout(fixture, "3\n");
}

#[test]
fn static_direct_eval_expression_preserves_side_effect_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-expression-side-effect.ts";
    assert_node_shim_stdout(fixture, "after\nafter\n");
}

#[test]
fn static_direct_eval_reads_caller_arguments_object() {
    let fixture = "fixtures/core-semantics/direct-eval-arguments.ts";
    assert_node_shim_stdout(fixture, "7:3\n");
}

#[test]
fn static_direct_eval_declaration_only_completion_is_undefined() {
    let fixture = "fixtures/core-semantics/direct-eval-declaration-empty-completion.ts";
    assert_node_shim_stdout(fixture, "undefined\n");
}

#[test]
fn static_direct_eval_declaration_preserves_previous_completion() {
    let fixture =
        "fixtures/core-semantics/direct-eval-declaration-preserves-previous-completion.ts";
    assert_node_shim_stdout(fixture, "1\n");
}

#[test]
fn static_direct_eval_destructuring_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-destructuring-completion.ts";
    assert_node_shim_stdout(fixture, "7\n");
}

#[test]
fn static_direct_eval_class_declaration_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-class-declaration-completion.ts";
    assert_node_shim_stdout(fixture, "function:5\nundefined\n");
}

#[test]
fn static_direct_eval_class_private_field_preserves_completion() {
    let fixture = "fixtures/core-semantics/direct-eval-class-private-field.ts";
    assert_node_shim_stdout(fixture, "8\nundefined\n");
}

#[test]
fn static_direct_eval_class_method_preserves_this_receiver() {
    let fixture = "fixtures/core-semantics/direct-eval-class-method-this.ts";
    assert_node_shim_stdout(fixture, "5\n");
}

#[test]
fn static_direct_eval_class_constructor_mutates_this_receiver() {
    let fixture = "fixtures/core-semantics/direct-eval-class-constructor-this.ts";
    assert_node_shim_stdout(fixture, "7\n");
}

#[test]
fn static_direct_eval_class_method_preserves_arguments_object() {
    let fixture = "fixtures/core-semantics/direct-eval-class-method-arguments.ts";
    assert_node_shim_stdout(fixture, "9\n");
}

#[test]
fn static_direct_eval_class_constructor_preserves_new_target() {
    let fixture = "fixtures/core-semantics/direct-eval-class-new-target.ts";
    assert_node_shim_stdout(fixture, "true\n");
}

#[test]
fn static_direct_eval_class_static_block_is_class_context() {
    let fixture = "fixtures/core-semantics/direct-eval-class-static-block.ts";
    assert_node_shim_stdout(fixture, "true\n");
}

#[test]
fn static_direct_eval_class_static_block_binds_this() {
    let fixture = "fixtures/core-semantics/direct-eval-class-static-block-this.ts";
    assert_node_shim_stdout(fixture, "7\nundefined\n");
}

#[test]
fn static_direct_eval_recursively_expands_nested_static_eval() {
    let fixture = "fixtures/core-semantics/direct-eval-nested-static.ts";
    assert_node_shim_stdout(fixture, "3\n");
}

#[test]
fn static_direct_eval_function_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-function-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "4\n4\n");
}

#[test]
fn static_direct_eval_function_is_hoisted_before_use() {
    let fixture = "fixtures/core-semantics/direct-eval-function-hoisted-before-use.ts";
    assert_node_shim_stdout(fixture, "4\n4\n");
}

#[test]
fn static_direct_eval_var_lands_in_caller_scope() {
    let fixture = "fixtures/core-semantics/direct-eval-var-lands-in-caller.ts";
    assert_node_shim_stdout(fixture, "2\n2\n");
}

#[test]
fn static_direct_eval_strict_lexical_local_shadows_caller_local() {
    let fixture = "fixtures/core-semantics/direct-eval-strict-lexical-local.ts";
    assert_node_shim_stdout(fixture, "2\n1\n");
}

#[test]
fn static_direct_eval_strict_caller_var_stays_eval_local() {
    let fixture = "fixtures/core-semantics/direct-eval-strict-caller-var-local.ts";
    assert_node_shim_stdout(fixture, "2\nundefined\n");
}

#[test]
fn static_direct_eval_strict_lexical_assignment_stays_eval_local() {
    let fixture = "fixtures/core-semantics/direct-eval-strict-lexical-assignment.ts";
    assert_node_shim_stdout(fixture, "3\n1\n");
}

#[test]
fn dynamic_direct_eval_writes_back_local_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-local-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_writes_back_parameter_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-param-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_writes_back_shadowed_block_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-block-shadow-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n1\n");
}

#[test]
fn dynamic_direct_eval_writes_back_catch_binding_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-catch-binding-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_function_declaration_writes_back_var_binding_through_node_shim_host_import()
{
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-function-declaration-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_function_expression_name_does_not_create_eval_binding_node_shim_host_import()
{
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-function-expression-name-not-binding-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n2\n");
}

#[test]
fn dynamic_direct_eval_async_and_generator_expression_names_do_not_create_eval_bindings_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-async-generator-expression-name-not-binding-node-shim.ts";
    assert_node_shim_stdout(fixture, "3\n3\n4\n4\n");
}

#[test]
fn dynamic_direct_eval_calls_function_properties_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-function-property-call-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n9\n");
}

#[test]
fn dynamic_direct_eval_calls_computed_function_properties_through_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-computed-function-property-call-node-shim.ts";
    assert_node_shim_stdout(fixture, "15\n");
}

#[test]
fn dynamic_direct_eval_var_declaration_writes_back_var_binding_through_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-var-declaration-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_new_var_declaration_is_visible_to_later_eval_through_node_shim_host_import()
{
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-new-var-declaration-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_new_var_declaration_is_visible_to_normal_code_through_node_shim_host_import()
{
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-new-var-normal-code-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_new_var_destructuring_is_visible_to_later_eval_through_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-new-var-destructuring-node-shim.ts";
    assert_node_shim_stdout(fixture, "9\n9\n");
}

#[test]
fn dynamic_direct_eval_new_var_destructuring_is_visible_to_normal_code_through_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-new-var-destructuring-normal-code-node-shim.ts";
    assert_node_shim_stdout(fixture, "9\n9\n");
}

#[test]
fn dynamic_direct_eval_new_var_array_destructuring_is_visible_to_later_eval_through_node_shim_host_import()
 {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-new-var-array-destructuring-node-shim.ts";
    assert_node_shim_stdout(fixture, "4\n4\n");
}

#[test]
fn dynamic_direct_eval_new_var_array_destructuring_is_visible_to_normal_code_through_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-new-var-array-destructuring-normal-code-node-shim.ts";
    assert_node_shim_stdout(fixture, "4\n4\n");
}

#[test]
fn dynamic_direct_eval_for_head_var_is_visible_to_normal_code_through_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-for-head-var-normal-code-node-shim.ts";
    assert_node_shim_stdout(fixture, "alpha\n4\n");
}

#[test]
fn dynamic_direct_eval_for_head_var_destructuring_is_visible_to_normal_code_through_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-for-head-var-destructuring-normal-code-node-shim.ts";
    assert_node_shim_stdout(fixture, "6\n8\n9\n");
}

#[test]
fn dynamic_direct_eval_new_function_declaration_is_visible_to_later_eval_through_node_shim_host_import()
 {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-new-function-declaration-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_new_function_declaration_is_visible_to_normal_code_through_node_shim_host_import()
 {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-new-function-normal-code-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_new_function_computed_tostring_through_node_shim_host_import() {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-new-function-computed-tostring-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nfunction created() { return 7; }\n");
}

#[test]
fn dynamic_direct_eval_new_generator_function_declaration_is_visible_to_later_eval_through_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-new-generator-function-declaration-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
}

#[test]
fn dynamic_direct_eval_new_async_function_declaration_is_visible_to_later_eval_through_node_shim_host_import()
 {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-new-async-function-declaration-node-shim.ts";
    assert_node_shim_stdout(fixture, "AsyncFunction\nAsyncFunction\n");
}

#[test]
fn dynamic_direct_eval_class_method_reads_arguments_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-class-method-arguments-node-shim.ts";
    assert_node_shim_stdout(fixture, "9:1\n");
}

#[test]
fn dynamic_direct_eval_class_method_reads_this_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-class-method-this-node-shim.ts";
    assert_node_shim_stdout(fixture, "7:5\n");
}

#[test]
fn dynamic_direct_eval_class_constructor_reads_this_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-class-constructor-this-node-shim.ts";
    assert_node_shim_stdout(fixture, "7:7\n");
}

#[test]
fn dynamic_direct_eval_object_method_reads_this_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-object-method-this-node-shim.ts";
    assert_node_shim_stdout(fixture, "7:5\n");
}

#[test]
fn dynamic_direct_eval_arrow_reads_lexical_this_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-arrow-lexical-this-node-shim.ts";
    assert_node_shim_stdout(fixture, "7:5\n");
}

#[test]
fn dynamic_direct_eval_arrow_writes_back_lexical_local_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-arrow-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "6\n6\n");
}

#[test]
fn dynamic_direct_eval_writes_back_string_env_cell_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-string-writeback-node-shim.ts";
    assert_node_shim_stdout(fixture, "after\nafter\n");
}

#[test]
fn dynamic_direct_eval_returns_object_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-object-result-node-shim.ts";
    assert_node_shim_stdout(fixture, "[object Object]\n");
}

#[test]
fn dynamic_direct_eval_preserves_object_properties_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-object-properties-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_direct_eval_preserves_object_identity_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-object-identity-node-shim.ts";
    assert_node_shim_stdout(fixture, "true\n7\n");
}

#[test]
fn dynamic_direct_eval_bridges_nested_arrays_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-nested-array-node-shim.ts";
    assert_node_shim_stdout(fixture, "2\n7\n8\nundefined\n");
}

#[test]
fn dynamic_direct_eval_calls_array_function_elements_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-array-function-element-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n12\n");
}

#[test]
fn dynamic_direct_eval_let_initializer_ignores_unreferenced_later_bindings() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-let-initializer-node-shim.ts";
    assert_node_shim_stdout(fixture, "ok\nafter\n");
}

#[test]
fn dynamic_direct_eval_ignores_later_binding_name_inside_source_string_literal() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-name-in-string-node-shim.ts";
    assert_node_shim_stdout(fixture, "later\nafter\n");
}

#[test]
fn dynamic_direct_eval_ignores_later_binding_name_inside_source_template_raw_text() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-name-in-template-node-shim.ts";
    assert_node_shim_stdout(fixture, "later\nafter\n");
}

#[test]
fn dynamic_direct_eval_ignores_later_binding_name_inside_source_regexp_literal() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-name-in-regexp-node-shim.ts";
    assert_node_shim_stdout(fixture, "true\nafter\n");
}

#[test]
fn dynamic_direct_eval_bridges_nested_objects_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-nested-object-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
}

#[test]
fn dynamic_direct_eval_bridges_thrown_errors_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-throw-catch-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\ndirect boom\n");
}

#[test]
fn dynamic_direct_eval_bridges_thrown_object_methods_through_node_shim_host_import() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-throw-object-method-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\n7\n");
}

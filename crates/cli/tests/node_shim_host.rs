use std::{fs, process::Command};

#[path = "common/capability.rs"]
mod capability;

use capability::node_command;
use ts2wasm_shared::test_helpers::{fixture_path, temp_wasm_path, unique_temp_dir};

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
fn dynamic_function_handle_bridges_thrown_errors_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-throw-catch-node-shim.ts";
    assert_node_shim_stdout(fixture, "Error\nfunction boom\n");
}

#[test]
fn dynamic_function_compile_bridges_syntax_errors_through_node_shim_host_imports() {
    let fixture =
        "fixtures/core-semantics/function-constructor-dynamic-syntax-error-catch-node-shim.ts";
    assert_node_shim_stdout(fixture, "SyntaxError\n");
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
fn dynamic_function_handle_bridges_nested_objects_through_node_shim_host_imports() {
    let fixture = "fixtures/core-semantics/function-constructor-dynamic-nested-object-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\nok\nundefined\n");
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
    assert_node_shim_stdout(fixture, "2\nanonymous\n[object Object]\n7\n");
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
fn dynamic_direct_eval_new_var_destructuring_is_visible_to_later_eval_through_node_shim_host_import()
 {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-new-var-destructuring-node-shim.ts";
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
fn dynamic_direct_eval_new_function_declaration_is_visible_to_later_eval_through_node_shim_host_import()
 {
    let fixture =
        "fixtures/core-semantics/direct-eval-dynamic-new-function-declaration-node-shim.ts";
    assert_node_shim_stdout(fixture, "7\n7\n");
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
fn dynamic_direct_eval_rejects_tdz_env_descriptor_conflict() {
    let fixture = "fixtures/core-semantics/direct-eval-dynamic-tdz-conflict-unsupported.ts";
    assert_build_fails_with(fixture, "UnsupportedEval", "TDZ-aware env descriptors");
}

#[test]
fn static_direct_eval_rejects_return_statement() {
    let fixture = "fixtures/core-semantics/direct-eval-return-unsupported.ts";
    assert_build_fails_with(
        fixture,
        "UnsupportedEval",
        "return statement is not valid in eval source",
    );
}

fn assert_build_fails_with(fixture: &str, expected_code: &str, expected_message: &str) {
    let fixture_path = fixture_path(fixture);
    let output_wasm = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("failed to execute ts2wasm build");

    assert!(
        !build.status.success(),
        "{fixture} should fail to build but succeeded\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let stderr = String::from_utf8_lossy(&build.stderr);
    assert!(
        stderr.contains(expected_code),
        "expected {expected_code} diagnostic for {fixture}, got:\n{stderr}"
    );
    assert!(
        stderr.contains(expected_message),
        "expected diagnostic message containing {expected_message:?} for {fixture}, got:\n{stderr}"
    );
}

fn assert_node_shim_stdout(fixture: &str, expected_stdout: &str) {
    let fixture_path = fixture_path(fixture);
    let output_wasm = temp_wasm_path(fixture);

    let build = Command::new(env!("CARGO_BIN_EXE_ts2wasm"))
        .arg("build")
        .arg(&fixture_path)
        .arg("-o")
        .arg(&output_wasm)
        .output()
        .expect("failed to execute ts2wasm build");

    assert!(
        build.status.success(),
        "{fixture} should build for node-shim execution\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let runner_dir = unique_temp_dir("node-shim-host");
    let runner = runner_dir.join("runner.mjs");
    fs::write(&runner, NODE_SHIM_RUNNER).expect("failed to write node shim runner");

    let node = node_command()
        .arg(&runner)
        .arg(&output_wasm)
        .output()
        .expect("failed to execute node shim runner");

    assert!(
        node.status.success(),
        "node shim runner should execute {fixture}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&node.stdout),
        String::from_utf8_lossy(&node.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&node.stdout), expected_stdout);
}

const NODE_SHIM_RUNNER: &str = r#"
import fs from 'node:fs';

const wasmPath = process.argv[process.argv.length - 1];
const wasmBytes = fs.readFileSync(wasmPath);

const TAG_UNDEFINED = 0;
const TAG_NULL = 1;
const TAG_FALSE = 2;
const TAG_TRUE = 3;
const TAG_NUMBER = 4;
const TAG_ARRAY = 5;
const TAG_STRING = 6;
const TAG_OBJECT = 7;
const TAG_MASK = 7;
const HEAP_MASK = -8;
const ARRAY_HEADER_SIZE = 20;
const ARRAY_PRESENCE_WORDS_OFFSET = 16;
const HOST_EXCEPTION_ARRAY_CAPACITY = -2;
const OBJECT_HEADER_SIZE = 12;
const OBJECT_ENTRIES_OFFSET = 12;
const OBJECT_ENTRY_SIZE = 8;
const GC_HEADER_SIZE = 16;
const GC_FLAGS_AND_TYPE_OFFSET = 0;
const GC_BODY_SIZE_OFFSET = 4;
const GC_KIND_ARRAY = 8;
const GC_KIND_OBJECT = 12;

let memory;
const hostFunctions = [];
const hostFunctionHandles = new Map();
const hostFunctionHandleValues = new WeakMap();
const hostArrayHandles = new WeakMap();
const hostObjectHandles = new WeakMap();
const hostObjectValues = new Map();
const directEvalExtraBindings = new Map();
const EVAL_DESCRIPTOR_CALLER_STRICT = '__ts2wasm_eval_caller_strict';
const decoder = new TextDecoder();
const encoder = new TextEncoder();
let stdout = '';
let hostHeapCursor = 0;

function view() {
  return new DataView(memory.buffer);
}

function bytes() {
  return new Uint8Array(memory.buffer);
}

function rawTag(raw) {
  return raw & TAG_MASK;
}

function rawPtr(raw) {
  return raw & HEAP_MASK;
}

function decodeString(raw) {
  if (rawTag(raw) !== TAG_STRING) {
    throw new TypeError(`expected string RawValue, got ${raw}`);
  }
  const base = rawPtr(raw);
  const len = view().getInt32(base, true);
  return decoder.decode(bytes().subarray(base + 4, base + 4 + len));
}

function decodeArray(raw) {
  if (rawTag(raw) !== TAG_ARRAY) {
    throw new TypeError(`expected array RawValue, got ${raw}`);
  }
  const base = rawPtr(resolveHostArrayRaw(raw));
  const len = view().getInt32(base, true);
  const presenceWords = view().getInt32(base + 8, true);
  const elementsOffset = view().getInt32(base + 12, true);
  const values = [];
  for (let i = 0; i < len; i += 1) {
    const wordIndex = i >> 5;
    const bitIndex = i & 31;
    const present =
      wordIndex < presenceWords &&
      (view().getUint32(base + ARRAY_PRESENCE_WORDS_OFFSET + wordIndex * 4, true) &
        (1 << bitIndex)) !==
        0;
    values.push(present ? view().getInt32(base + elementsOffset + i * 4, true) : TAG_UNDEFINED);
  }
  return values;
}

function decodeObject(raw) {
  if (rawTag(raw) !== TAG_OBJECT) {
    throw new TypeError(`expected object RawValue, got ${raw}`);
  }
  const ptr = rawPtr(raw);
  if (hostObjectValues.has(ptr)) return hostObjectValues.get(ptr);
  const forwarded = view().getInt32(ptr + 8, true);
  if (forwarded !== 0 && view().getInt32(ptr, true) === 0 && view().getInt32(ptr + 4, true) === 0) {
    return decodeObject(forwarded | TAG_OBJECT);
  }

  const len = view().getInt32(ptr, true);
  const object = {};
  for (let i = 0; i < len; i += 1) {
    const entry = ptr + OBJECT_ENTRIES_OFFSET + i * OBJECT_ENTRY_SIZE;
    object[decodeString(view().getInt32(entry, true))] = decodeValue(
      view().getInt32(entry + 4, true),
    );
  }
  return object;
}

function decodeValue(raw) {
  switch (rawTag(raw)) {
    case TAG_UNDEFINED:
      return undefined;
    case TAG_NULL:
      return null;
    case TAG_FALSE:
      return false;
    case TAG_TRUE:
      return true;
    case TAG_NUMBER:
      return raw >> 3;
    case TAG_STRING:
      return decodeString(raw);
    case TAG_ARRAY:
      return decodeArray(raw).map(decodeValue);
    case TAG_OBJECT: {
      const ptr = rawPtr(raw);
      if (hostFunctionHandles.has(ptr)) {
        return hostFunctions[hostFunctionHandles.get(ptr)];
      }
      return decodeObject(raw);
    }
    default:
      throw new TypeError(`unsupported RawValue for this host-shim test: ${raw}`);
  }
}

function readEnvCellRaw(cellRaw) {
  if (rawTag(cellRaw) !== TAG_ARRAY) {
    throw new TypeError(`expected env cell array RawValue, got ${cellRaw}`);
  }
  const base = rawPtr(cellRaw);
  return view().getInt32(base + ARRAY_HEADER_SIZE, true);
}

function writeEnvCellRaw(cellRaw, valueRaw) {
  if (rawTag(cellRaw) !== TAG_ARRAY) {
    throw new TypeError(`expected env cell array RawValue, got ${cellRaw}`);
  }
  const base = rawPtr(cellRaw);
  view().setInt32(base + ARRAY_HEADER_SIZE, valueRaw, true);
}

function alignHostPtr(ptr) {
  return Math.ceil(ptr / 8) * 8;
}

function hostAlloc(size) {
  if (hostHeapCursor === 0) {
    hostHeapCursor = bytes().byteLength;
  }
  const ptr = alignHostPtr(hostHeapCursor);
  const end = ptr + size;
  while (end > bytes().byteLength) {
    memory.grow(1);
  }
  hostHeapCursor = end;
  return ptr;
}

function encodeString(value) {
  const data = encoder.encode(value);
  const ptr = hostAlloc(4 + data.length);
  view().setInt32(ptr, data.length, true);
  bytes().set(data, ptr + 4);
  return ptr | TAG_STRING;
}

function refreshHostArrayEntries(value, record) {
  if (value.length > record.capacity) {
    throw new TypeError('internal host array capacity mismatch for this test');
  }
  const ptr = rawPtr(record.raw);
  view().setInt32(ptr, value.length, true);
  for (let word = 0; word < record.presenceWords; word += 1) {
    view().setUint32(ptr + ARRAY_PRESENCE_WORDS_OFFSET + word * 4, 0, true);
  }
  for (let i = 0; i < value.length; i += 1) {
    const entry = ptr + record.elementsOffset + i * 4;
    if (Object.prototype.hasOwnProperty.call(value, i)) {
      const word = i >> 5;
      const bit = i & 31;
      const maskOffset = ptr + ARRAY_PRESENCE_WORDS_OFFSET + word * 4;
      const mask = view().getUint32(maskOffset, true) | (1 << bit);
      view().setUint32(maskOffset, mask, true);
      view().setInt32(entry, encodeHostValue(value[i]), true);
    } else {
      view().setInt32(entry, TAG_UNDEFINED, true);
    }
  }
}

function resolveHostArrayRaw(raw) {
  let current = raw;
  for (let steps = 0; steps < 64; steps += 1) {
    const ptr = rawPtr(current);
    if (view().getInt32(ptr + 4, true) !== -1) {
      return current;
    }
    current = view().getInt32(ptr + ARRAY_HEADER_SIZE, true);
  }
  throw new TypeError('host array forwarding cycle');
}

function allocateHostArrayRecord(value, requestedCapacity) {
  const capacity = Math.max(value.length, requestedCapacity, 4);
  const presenceWords = Math.max(1, Math.ceil(capacity / 32));
  const elementsOffset = ARRAY_PRESENCE_WORDS_OFFSET + presenceWords * 4;
  const size = elementsOffset + capacity * 4;
  const base = hostAlloc(GC_HEADER_SIZE + size);
  view().setInt32(base + GC_FLAGS_AND_TYPE_OFFSET, GC_KIND_ARRAY, true);
  view().setInt32(base + GC_BODY_SIZE_OFFSET, size, true);
  const ptr = base + GC_HEADER_SIZE;
  view().setInt32(ptr + 4, capacity, true);
  view().setInt32(ptr + 8, presenceWords, true);
  view().setInt32(ptr + 12, elementsOffset, true);
  const raw = ptr | TAG_ARRAY;
  const record = { raw, capacity, presenceWords, elementsOffset };
  hostArrayHandles.set(value, record);
  return record;
}

function forwardHostArrayRecord(from, to) {
  const fromPtr = rawPtr(from.raw);
  view().setInt32(fromPtr, 1, true);
  view().setInt32(fromPtr + 4, -1, true);
  view().setInt32(fromPtr + 8, 1, true);
  view().setInt32(fromPtr + 12, ARRAY_HEADER_SIZE, true);
  view().setUint32(fromPtr + ARRAY_PRESENCE_WORDS_OFFSET, 1, true);
  view().setInt32(fromPtr + ARRAY_HEADER_SIZE, to.raw, true);
}

function encodeHostArray(value) {
  let record = hostArrayHandles.get(value);
  if (record === undefined) {
    record = allocateHostArrayRecord(value, value.length);
  } else if (value.length > record.capacity) {
    const previous = record;
    record = allocateHostArrayRecord(value, record.capacity * 2);
    forwardHostArrayRecord(previous, record);
  }
  refreshHostArrayEntries(value, record);
  return record.raw;
}

function refreshHostObjectEntries(value, record) {
  const keys = Object.keys(value);
  if (keys.length > record.capacity) {
    throw new TypeError('internal host object capacity mismatch for this test');
  }
  const ptr = rawPtr(record.raw);
  view().setInt32(ptr, keys.length, true);
  for (let i = 0; i < keys.length; i += 1) {
    const entry = ptr + OBJECT_ENTRIES_OFFSET + i * OBJECT_ENTRY_SIZE;
    view().setInt32(entry, encodeString(keys[i]), true);
    view().setInt32(entry + 4, encodeHostValue(value[keys[i]]), true);
  }
  record.keys = keys;
}

function allocateHostObjectRecord(value, requestedCapacity) {
  const capacity = Math.max(Object.keys(value).length, requestedCapacity, 4);
  const size = OBJECT_HEADER_SIZE + capacity * OBJECT_ENTRY_SIZE;
  const base = hostAlloc(GC_HEADER_SIZE + size);
  view().setInt32(base + GC_FLAGS_AND_TYPE_OFFSET, GC_KIND_OBJECT, true);
  view().setInt32(base + GC_BODY_SIZE_OFFSET, size, true);
  const ptr = base + GC_HEADER_SIZE;
  view().setInt32(ptr, 0, true);
  view().setInt32(ptr + 4, 0, true);
  view().setInt32(ptr + 8, 0, true);
  const raw = ptr | TAG_OBJECT;
  const record = { raw, keys: [], capacity };
  hostObjectHandles.set(value, record);
  hostObjectValues.set(ptr, value);
  return record;
}

function forwardHostObjectRecord(from, to) {
  const fromPtr = rawPtr(from.raw);
  const toPtr = rawPtr(to.raw);
  view().setInt32(fromPtr, 0, true);
  view().setInt32(fromPtr + 4, 0, true);
  view().setInt32(fromPtr + 8, toPtr, true);
}

function encodeHostObject(value) {
  const keys = Object.keys(value);
  let record = hostObjectHandles.get(value);
  if (record === undefined) {
    record = allocateHostObjectRecord(value, keys.length);
  } else if (keys.length > record.capacity) {
    const previous = record;
    record = allocateHostObjectRecord(value, record.capacity * 2);
    forwardHostObjectRecord(previous, record);
  }
  refreshHostObjectEntries(value, record);
  return record.raw;
}

function encodeHostFunctionHandle(fn, index) {
  const raw = encodeHostObject({
    length: fn.length,
    name: fn.name,
    prototype: {},
  });
  hostFunctionHandles.set(rawPtr(raw), index);
  return raw;
}

function encodeHostFunctionValue(fn) {
  const existing = hostFunctionHandleValues.get(fn);
  if (existing !== undefined) return existing;
  hostFunctions.push(fn);
  const raw = encodeHostFunctionHandle(fn, hostFunctions.length - 1);
  hostFunctionHandleValues.set(fn, raw);
  return raw;
}

function decodeHostFunctionHandle(raw) {
  if (rawTag(raw) !== TAG_OBJECT) {
    throw new TypeError(`expected host function handle object RawValue, got ${raw}`);
  }
  const ptr = rawPtr(raw);
  if (!hostFunctionHandles.has(ptr)) {
    throw new TypeError(`unknown host function handle object: ${raw}`);
  }
  return hostFunctionHandles.get(ptr);
}

function decodeHostReceiver(raw) {
  if (rawTag(raw) === TAG_OBJECT) {
    const ptr = rawPtr(raw);
    if (hostObjectValues.has(ptr)) return hostObjectValues.get(ptr);
  }
  return decodeValue(raw);
}

function encodeHostValue(value) {
  if (value === undefined) return TAG_UNDEFINED;
  if (value === null) return TAG_NULL;
  if (value === false) return TAG_FALSE;
  if (value === true) return TAG_TRUE;
  if (Number.isInteger(value)) return (value << 3) | TAG_NUMBER;
  if (typeof value === 'string') return encodeString(value);
  if (Array.isArray(value)) return encodeHostArray(value);
  if (typeof value === 'object') return encodeHostObject(value);
  if (typeof value === 'function') return encodeHostFunctionValue(value);
  throw new TypeError(`unsupported host return value for this test: ${String(value)}`);
}

function encodeHostException(error) {
  const name = error && typeof error.name === 'string' ? error.name : 'Error';
  const message = error && typeof error.message === 'string' ? error.message : String(error);
  const errorRaw = encodeHostObject({ name, message });
  const base = hostAlloc(GC_HEADER_SIZE + ARRAY_HEADER_SIZE + 4);
  view().setInt32(base + GC_FLAGS_AND_TYPE_OFFSET, GC_KIND_ARRAY, true);
  view().setInt32(base + GC_BODY_SIZE_OFFSET, ARRAY_HEADER_SIZE + 4, true);
  const ptr = base + GC_HEADER_SIZE;
  view().setInt32(ptr, 1, true);
  view().setInt32(ptr + 4, HOST_EXCEPTION_ARRAY_CAPACITY, true);
  view().setInt32(ptr + 8, 1, true);
  view().setInt32(ptr + 12, ARRAY_HEADER_SIZE, true);
  view().setUint32(ptr + ARRAY_PRESENCE_WORDS_OFFSET, 1, true);
  view().setInt32(ptr + ARRAY_HEADER_SIZE, errorRaw, true);
  return ptr | TAG_ARRAY;
}

function uniqueInternalName(base, names) {
  let name = base;
  while (names.includes(name)) {
    name = `_${name}`;
  }
  return name;
}

function isIdentifierStart(ch) {
  return /[A-Za-z_$]/.test(ch);
}

function isIdentifierPart(ch) {
  return /[0-9A-Za-z_$]/.test(ch);
}

function skipWhitespace(text, index) {
  let i = index;
  while (i < text.length && /\s/.test(text[i])) i += 1;
  return i;
}

function splitTopLevelComma(text) {
  const parts = [];
  let start = 0;
  let depth = 0;
  let quote = null;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      depth -= 1;
    } else if (ch === ',' && depth === 0) {
      parts.push(text.slice(start, i));
      start = i + 1;
    }
  }
  parts.push(text.slice(start));
  return parts;
}

function topLevelEqualsIndex(text) {
  let depth = 0;
  let quote = null;
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      depth -= 1;
    } else if (ch === '=' && depth === 0) {
      return i;
    }
  }
  return -1;
}

function skipBindingInitializer(text, index) {
  let i = index;
  let depth = 0;
  let quote = null;
  while (i < text.length) {
    const ch = text[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      i += 1;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      if (depth === 0) return i;
      depth -= 1;
    } else if (ch === ',' && depth === 0) {
      return i;
    }
    i += 1;
  }
  return i;
}

function addBindingNamesFromPattern(pattern, addName) {
  for (let i = 0; i < pattern.length; ) {
    const ch = pattern[i];
    if (ch === '=') {
      i = skipBindingInitializer(pattern, i + 1);
      continue;
    }
    if (!isIdentifierStart(ch)) {
      i += 1;
      continue;
    }
    let end = i + 1;
    while (end < pattern.length && isIdentifierPart(pattern[end])) end += 1;
    const name = pattern.slice(i, end);
    const next = skipWhitespace(pattern, end);
    if (pattern[next] === ':') {
      i = next + 1;
      continue;
    }
    addName(name);
    i = end;
  }
}

function readVarDeclarationText(source, index) {
  let depth = 0;
  let quote = null;
  for (let i = index; i < source.length; i += 1) {
    const ch = source[i];
    if (quote !== null) {
      if (ch === '\\') i += 1;
      else if (ch === quote) quote = null;
      continue;
    }
    if (ch === '"' || ch === "'" || ch === '`') {
      quote = ch;
    } else if (ch === '{' || ch === '[' || ch === '(') {
      depth += 1;
    } else if (ch === '}' || ch === ']' || ch === ')') {
      depth -= 1;
    } else if (ch === ';' && depth === 0) {
      return source.slice(index, i);
    }
  }
  return source.slice(index);
}

function directEvalEnvKey(bindings) {
  return bindings
    .map((binding) => `${binding.name}:${binding.cellRaw}`)
    .sort()
    .join('|');
}

function collectVariableDeclarationBindingNames(source, keyword) {
  const names = [];
  const addName = (name) => {
    if (!names.includes(name)) names.push(name);
  };
  const keywordPattern = new RegExp(`\\b${keyword}\\b`, 'g');
  for (const match of source.matchAll(keywordPattern)) {
    const declarationText = readVarDeclarationText(source, match.index + match[0].length);
    for (const declarator of splitTopLevelComma(declarationText)) {
      const equalsIndex = topLevelEqualsIndex(declarator);
      const pattern = (equalsIndex === -1 ? declarator : declarator.slice(0, equalsIndex)).trim();
      addBindingNamesFromPattern(pattern, addName);
    }
  }
  return names;
}

function collectEvalDeclarationNames(source) {
  const names = collectVariableDeclarationBindingNames(source, 'var');
  const addName = (name) => {
    if (!names.includes(name)) names.push(name);
  };
  for (const match of source.matchAll(/\b(?:async\s+)?function\s*\*?\s+([A-Za-z_$][0-9A-Za-z_$]*)\s*\(/g)) {
    let prior = match.index - 1;
    while (prior >= 0 && /\s/.test(source[prior])) prior -= 1;
    if (prior >= 0 && !';{}'.includes(source[prior])) {
      continue;
    }
    addName(match[1]);
  }
  return names;
}

function strictEvalHasDeleteIdentifier(source) {
  return /\bdelete\s+[A-Za-z_$][0-9A-Za-z_$]*\b(?!\s*[.[(])/.test(source);
}

function strictEvalHasRestrictedVariableBinding(source) {
  for (const keyword of ['var', 'let', 'const']) {
    const names = collectVariableDeclarationBindingNames(source, keyword);
    if (names.includes('arguments') || names.includes('eval')) {
      return true;
    }
  }
  return false;
}

function strictEvalHasRestrictedBinding(source) {
  return (
    strictEvalHasRestrictedVariableBinding(source) ||
    /\b(?:async\s+)?function\s*\*?\s+(?:arguments|eval)\b/.test(source)
  );
}

function evalWithEnvDescriptor(source, envRaw) {
  if (envRaw === TAG_UNDEFINED) {
    return eval(source);
  }

  const pairs = decodeArray(envRaw);
  let callerIsStrict = false;
  let pairOffset = 0;
  if (
    pairs.length >= 2 &&
    rawTag(pairs[0]) === TAG_STRING &&
    decodeString(pairs[0]) === EVAL_DESCRIPTOR_CALLER_STRICT
  ) {
    callerIsStrict = decodeValue(pairs[1]) === true;
    pairOffset = 2;
  }
  if ((pairs.length - pairOffset) % 2 !== 0) {
    throw new TypeError('invalid direct eval env descriptor');
  }
  if (callerIsStrict && strictEvalHasDeleteIdentifier(source)) {
    throw new SyntaxError('Delete of an unqualified identifier in strict mode.');
  }
  if (callerIsStrict && strictEvalHasRestrictedBinding(source)) {
    throw new SyntaxError('Unexpected eval or arguments in strict mode.');
  }

  const bindings = [];
  for (let i = pairOffset; i < pairs.length; i += 2) {
    const name = decodeString(pairs[i]);
    const cellRaw = pairs[i + 1];
    const raw = readEnvCellRaw(cellRaw);
    bindings.push({ name, cellRaw, raw, value: decodeValue(raw) });
  }

  const names = bindings.map((binding) => binding.name);
  const thisBinding = bindings.find((binding) => binding.name === 'this');
  const envKey = directEvalEnvKey(bindings);
  const extraMap = directEvalExtraBindings.get(envKey) ?? new Map();
  const extraBindings = [];
  for (const [name, value] of extraMap.entries()) {
    if (!names.includes(name)) {
      extraBindings.push({ name, value });
      names.push(name);
    }
  }
  if (!callerIsStrict) {
    for (const name of collectEvalDeclarationNames(source)) {
      if (!names.includes(name)) {
        extraBindings.push({ name, value: undefined });
        names.push(name);
      }
    }
  }
  const formalBindings = bindings.filter((binding) => binding.name !== 'this');
  const allFormalBindings = formalBindings.concat(extraBindings);
  const sourceReferencesStrictReservedBinding = /\b(?:arguments|eval)\b/.test(source);
  const useStrictWrapper = callerIsStrict && !sourceReferencesStrictReservedBinding;
  const wrapperBindings = useStrictWrapper
    ? allFormalBindings.filter((binding) => binding.name !== 'arguments' && binding.name !== 'eval')
    : allFormalBindings;
  const formalNames = wrapperBindings.map((binding) => binding.name);
  const sourceName = uniqueInternalName('__ts2wasm_eval_source', names);
  const resultName = uniqueInternalName('__ts2wasm_eval_result', [...names, sourceName]);
  const strictPrefix = useStrictWrapper ? '"use strict"; ' : '';
  const wrapper = Function(
    sourceName,
    ...formalNames,
    `${strictPrefix}let ${resultName} = eval(${sourceName}); return [${resultName}, ${formalNames.join(', ')}];`,
  );
  const values = wrapperBindings.map((binding) => binding.value);
  const thisValue = thisBinding === undefined ? undefined : thisBinding.value;
  const [result, ...updatedValues] = wrapper.call(thisValue, source, ...values);

  for (let i = 0; i < wrapperBindings.length; i += 1) {
    if (!Object.is(wrapperBindings[i].value, updatedValues[i])) {
      if (wrapperBindings[i].cellRaw !== undefined) {
        writeEnvCellRaw(wrapperBindings[i].cellRaw, encodeHostValue(updatedValues[i]));
      } else {
        extraMap.set(wrapperBindings[i].name, updatedValues[i]);
      }
    }
  }
  if (extraMap.size > 0) directEvalExtraBindings.set(envKey, extraMap);

  return result;
}

function decodeArgs(raw) {
  return decodeArray(raw).map(decodeValue);
}

const imports = {
  wasi_snapshot_preview1: {
    fd_write(fd, iovs, iovsLen, nwritten) {
      if (fd !== 1) return 8;
      let written = 0;
      for (let i = 0; i < iovsLen; i += 1) {
        const iov = iovs + i * 8;
        const ptr = view().getInt32(iov, true);
        const len = view().getInt32(iov + 4, true);
        stdout += decoder.decode(bytes().subarray(ptr, ptr + len));
        written += len;
      }
      view().setInt32(nwritten, written, true);
      return 0;
    },
    proc_exit(code) {
      throw Object.assign(new Error(`proc_exit(${code})`), { code });
    },
  },
  host: {
    'eval.direct'(sourceRaw, envRaw) {
      try {
        const result = evalWithEnvDescriptor(decodeString(sourceRaw), envRaw);
        return encodeHostValue(result);
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'eval.indirect'(sourceRaw, _envRaw) {
      try {
        const result = globalThis.eval(decodeString(sourceRaw));
        return encodeHostValue(result);
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.compile'(argsRaw) {
      try {
        const args = decodeArgs(argsRaw);
        const fn = Function(...args);
        return encodeHostFunctionValue(fn);
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.call'(handleRaw, argsRaw) {
      try {
        const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
        if (typeof fn !== 'function') {
          throw new TypeError(`unknown host function handle: ${handleRaw}`);
        }
        return encodeHostValue(fn(...decodeArgs(argsRaw)));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.callMethod'(handleRaw, receiverRaw, argsRaw) {
      try {
        const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
        if (typeof fn !== 'function') {
          throw new TypeError(`unknown host function handle: ${handleRaw}`);
        }
        return encodeHostValue(fn.apply(decodeHostReceiver(receiverRaw), decodeArgs(argsRaw)));
      } catch (error) {
        return encodeHostException(error);
      }
    },
    'function.construct'(handleRaw, argsRaw) {
      try {
        const fn = hostFunctions[decodeHostFunctionHandle(handleRaw)];
        if (typeof fn !== 'function') {
          throw new TypeError(`unknown host function handle: ${handleRaw}`);
        }
        return encodeHostValue(Reflect.construct(fn, decodeArgs(argsRaw)));
      } catch (error) {
        return encodeHostException(error);
      }
    },
  },
};

try {
  const { instance } = await WebAssembly.instantiate(wasmBytes, imports);
  memory = instance.exports.memory;
  instance.exports._start();
} catch (error) {
  if (error && error.code === 0) {
    process.stdout.write(stdout);
    process.exit(0);
  }
  throw error;
}

process.stdout.write(stdout);
"#;

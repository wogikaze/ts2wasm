pub(super) mod array;
pub(super) mod class;
pub(super) mod number;
pub(super) mod object;
pub(super) mod string;

// Re-export submodule items for use from parent (builtin_resolver)
pub(crate) use array::{resolve_array_literal, try_resolve_array_call};
pub(crate) use class::{class_method_kind, resolve_class_expr};
pub(crate) use number::resolve_global_identifier_call;
pub(crate) use object::resolve_object_literal;
// string: no exports yet

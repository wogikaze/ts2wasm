mod emitter;
mod expr_emit;
mod runtime_builder;
mod runtime_fn;
mod runtime_link_plan;
mod stmt_emit;
mod string_intern;

pub(crate) use emitter::emit_wat;

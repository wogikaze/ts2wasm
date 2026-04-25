mod capability_manifest;
mod emitter;
mod expr_emit;
mod runtime_builder;
mod runtime_fn;
mod runtime_link_plan;
mod stmt_emit;
mod string_intern;

use crate::ir::lowered::LoweredProgram;

pub(crate) use capability_manifest::emit_manifest_v1_json;
pub(crate) use emitter::emit_wat;
pub(crate) use runtime_fn::RuntimeFn;

pub(crate) fn program_requires_read_stdin_utf8_runtime(program: &LoweredProgram) -> bool {
    runtime_link_plan::RuntimeLinkPlan::from_program(program)
        .required_runtime_functions()
        .contains(&runtime_fn::RuntimeFn::ReadStdinUtf8)
}

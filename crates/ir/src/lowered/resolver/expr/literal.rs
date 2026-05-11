use super::super::{
    is_private_field_storage_key, is_set_prototype_property, is_set_prototype_property_expr,
    private_storage_observable_access_diagnostic,
};
use crate::builtin::BuiltinId;
use crate::builtin_resolved::ResolvedExpr;
use crate::lowered::*;
use ts2wasm_shared::{Span, Diagnostic};

impl super::super::Resolver {
    pub(super) fn lower_builtin_call_expr(
        &mut self,
        builtin: BuiltinId,
        args: &[ResolvedExpr],
    ) -> Result<LoweredExpr, Diagnostic> {
        let mut lowered_args = args
            .iter()
            .map(|arg| self.lower_expr(arg))
            .collect::<Result<Vec<_>, _>>()?;
        if builtin == BuiltinId::ParseInt && lowered_args.len() == 1 {
            lowered_args.push(LoweredExpr::Number(0, Span::generated("num")));
        }
        Ok(LoweredExpr::Call {
            kind: FunctionCallKind::Builtin(builtin),
            args: lowered_args,
            span: Span::generated("call"),
        })
    }
}

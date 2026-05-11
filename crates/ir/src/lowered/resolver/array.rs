use crate::builtin_resolved::ResolvedArrayElement;
use crate::lowered::*;
use ts2wasm_shared::Diagnostic;

impl<'a> super::Resolver<'a> {
    pub(super) fn lower_array_expr(
        &mut self,
        elements: &[ResolvedArrayElement],
    ) -> Result<LoweredExpr, Diagnostic> {
        self.lower_array_literal(elements)
    }
}

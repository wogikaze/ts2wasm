pub use ts2wasm_shared::diagnostic::{
    DiagCode, Diagnostic, InternalDiagnostic, SourceDiagnostic, Span,
};

#[cfg(test)]
mod tests {
    use super::{DiagCode, Diagnostic, Span};

    fn unsupported(message: &str) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: message.to_owned(),
            span: Some(Span { start: 1, end: 2 }),
            phase: None,
        }
    }

    #[test]
    fn display_code_splits_common_unsupported_categories() {
        let cases = [
            (
                "issue-055: unsupported named export; module resolution and loading are not implemented",
                DiagCode::UnsupportedModule,
                "[UnsupportedModule]",
            ),
            (
                "issue-050: Date.prototype.toString() requires timezone/host formatting policy",
                DiagCode::UnsupportedDate,
                "[UnsupportedDate]",
            ),
            (
                "issue-246: optional chaining cannot be used as an assignment or update target",
                DiagCode::UnsupportedSyntax,
                "[UnsupportedSyntax]",
            ),
            (
                "issue-051: RegExp.prototype.compile is not supported in this subset",
                DiagCode::UnsupportedRegExp,
                "[UnsupportedRegExp]",
            ),
            (
                "issue-347: indirect eval calls are not supported",
                DiagCode::UnsupportedEval,
                "[UnsupportedEval]",
            ),
            (
                "type annotation is not supported in this parser slice",
                DiagCode::UnsupportedTypeScriptSyntax,
                "[UnsupportedTypeScriptSyntax]",
            ),
            (
                "issue-255: private method `#m` is not declared in this class",
                DiagCode::UnsupportedSyntax,
                "[UnsupportedSyntax]",
            ),
            (
                "Array.prototype.map generic calls are not supported",
                DiagCode::UnsupportedBuiltin,
                "[UnsupportedBuiltin]",
            ),
            (
                "issue-280: BigInt.asIntN/asUintN currently require a supported BigInt value input",
                DiagCode::UnsupportedBuiltin,
                "[UnsupportedBuiltin]",
            ),
            (
                "issue-370: mixed Number/BigInt arithmetic TypeError parity is not implemented in the dynamic BigInt runtime slice",
                DiagCode::UnsupportedRuntimeSubset,
                "[UnsupportedRuntimeSubset]",
            ),
            (
                "issue-378: BigInt shift operators and unsigned right shift TypeError policy are not implemented",
                DiagCode::UnsupportedRuntimeSubset,
                "[UnsupportedRuntimeSubset]",
            ),
        ];

        for (message, expected, rendered_code) in cases {
            let diagnostic = unsupported(message);
            assert_eq!(diagnostic.display_code(), expected);
            assert!(diagnostic.to_string().contains(rendered_code));
        }
    }
}

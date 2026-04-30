/// Structured diagnostic emitted by compiler phases.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: DiagCode,
    pub message: String,
    pub span: Option<Span>,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.display_code();
        match &self.span {
            Some(span) => write!(
                f,
                "[{:?}] {} at {}..{}",
                code, self.message, span.start, span.end
            ),
            None => write!(f, "[{:?}] {}", code, self.message),
        }
    }
}

impl Diagnostic {
    pub fn display_code(&self) -> DiagCode {
        if self.code != DiagCode::UnsupportedSyntax {
            return self.code;
        }

        let message = self.message.to_ascii_lowercase();

        if message.contains("issue-055")
            || message.contains("module resolution")
            || message.contains("module loading")
            || message.contains(" import")
            || message.contains(" export")
            || message.contains("require(")
        {
            return DiagCode::UnsupportedModule;
        }

        if message.contains("issue-050") || contains_ascii_word(&message, "date") {
            return DiagCode::UnsupportedDate;
        }

        if message.contains("regexp")
            || message.contains("regular expression")
            || message.contains("issue-051")
        {
            return DiagCode::UnsupportedRegExp;
        }

        if message.contains("eval")
            || message.contains("issue-302")
            || message.contains("issue-347")
        {
            return DiagCode::UnsupportedEval;
        }

        if message.contains("typescript")
            || message.contains("type annotation")
            || message.contains("type directive")
            || message.contains("reference types")
            || message.contains("interface")
            || message.contains(" enum")
            || message.contains("declare")
            || message.contains("ambient")
            || message.contains("parameter propert")
            || message.contains("type alias")
        {
            return DiagCode::UnsupportedTypeScriptSyntax;
        }

        if message.contains("array.prototype")
            || message.contains("object.")
            || message.contains("object prototype")
            || message.contains("string.prototype")
            || message.contains("function.prototype")
            || message.contains("json.")
            || message.contains("math.")
            || message.contains("number.")
            || message.contains("boolean.")
            || message.contains("console.")
            || message.contains("process.")
            || message.contains("globalthis")
            || message.contains("builtin")
        {
            return DiagCode::UnsupportedBuiltin;
        }

        if message.contains("runtime slice")
            || message.contains("runtime subset")
            || message.contains("runtime semantics")
            || message.contains("literal runtime slice")
            || message.contains("literal-folding slice")
            || message.contains("coercion slice")
            || message.contains("in this subset")
            || message.contains("in this slice")
            || message.contains("not implemented in")
            || message.contains("is not implemented")
            || message.contains("are not implemented")
        {
            return DiagCode::UnsupportedRuntimeSubset;
        }

        DiagCode::UnsupportedSyntax
    }
}

fn contains_ascii_word(haystack: &str, needle: &str) -> bool {
    haystack.match_indices(needle).any(|(start, _)| {
        let before_is_boundary = start == 0
            || !haystack.as_bytes()[start - 1].is_ascii_alphanumeric()
                && haystack.as_bytes()[start - 1] != b'_';
        let end = start + needle.len();
        let after_is_boundary = end == haystack.len()
            || !haystack.as_bytes()[end].is_ascii_alphanumeric()
                && haystack.as_bytes()[end] != b'_';
        before_is_boundary && after_is_boundary
    })
}

/// Error codes for compiler diagnostics. See `docs/12-coding-standard.md` §2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagCode {
    /// A name referenced in source was not declared in any enclosing scope.
    UnresolvedName,
    /// A function called in source was not declared in the program.
    UnresolvedFunction,
    /// Two functions share the same name in the same program.
    DuplicateFunction,
    /// Two local bindings share the same name in the same lexical scope.
    DuplicateLocal,
    /// Two parameters share the same name in the same function parameter list.
    DuplicateParameter,
    /// A number literal is outside small-int tagged range.
    NumberOutOfRange,
    /// A function call passes the wrong number of arguments.
    ArityMismatch,
    /// `return` is used in top-level script scope.
    InvalidTopLevelReturn,
    /// A lowered IR node violates a structural invariant — this is a compiler bug.
    InvariantViolation,
    /// Source uses syntax that is not supported in the current milestone.
    UnsupportedSyntax,
    /// Source references a builtin API outside the current supported subset.
    UnsupportedBuiltin,
    /// Source uses Date behavior outside the current deterministic Date subset.
    UnsupportedDate,
    /// Source uses RegExp behavior outside the current supported RegExp subset.
    UnsupportedRegExp,
    /// Source uses module loading/resolution behavior outside the current subset.
    UnsupportedModule,
    /// Source uses eval behavior outside the current direct-eval subset.
    UnsupportedEval,
    /// Source uses TypeScript syntax that is not yet parsed or erased.
    UnsupportedTypeScriptSyntax,
    /// Source reaches a runtime/lowering subset boundary rather than parser syntax.
    UnsupportedRuntimeSubset,
    /// I/O or command execution failure at the backend boundary.
    BackendIo,
    /// TypeScript compiler API reported a type-checking diagnostic.
    TypeScriptTypeCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[cfg(test)]
mod tests {
    use super::{DiagCode, Diagnostic, Span};

    fn unsupported(message: &str) -> Diagnostic {
        Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: message.to_owned(),
            span: Some(Span { start: 1, end: 2 }),
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
                "Array.prototype.map generic calls are not supported",
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

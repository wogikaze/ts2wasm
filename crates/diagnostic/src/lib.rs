//! Diagnostic types for ts2wasm.
//!
//! Provides `DiagCode`, `Diagnostic`, `SourceDiagnostic`, and `InternalDiagnostic`
//! used throughout the compiler pipeline for error reporting.

use ts2wasm_source::Span;

/// Source-originating diagnostic with mandatory span.
/// Used for user-facing errors (unsupported syntax, unresolved names, etc.).
#[derive(Debug, Clone)]
pub struct SourceDiagnostic {
    pub span: Span,
    pub code: DiagCode,
    pub message: String,
}

impl SourceDiagnostic {
    pub fn new(span: Span, code: DiagCode, message: impl Into<String>) -> Self {
        Self {
            span,
            code,
            message: message.into(),
        }
    }
}

/// Internal diagnostic (compiler bug / invariant violation / backend I/O).
/// Span is optional because these errors are not always source-originating.
#[derive(Debug, Clone)]
pub struct InternalDiagnostic {
    pub span: Option<Span>,
    pub code: DiagCode,
    pub message: String,
}

impl InternalDiagnostic {
    pub fn new(code: DiagCode, message: impl Into<String>) -> Self {
        Self {
            span: None,
            code,
            message: message.into(),
        }
    }

    pub fn with_span(code: DiagCode, span: Span, message: impl Into<String>) -> Self {
        Self {
            span: Some(span),
            code,
            message: message.into(),
        }
    }
}

/// Structured diagnostic emitted by compiler phases.
/// This is the legacy common type; new code should prefer `SourceDiagnostic`
/// for user-facing errors and `InternalDiagnostic` for compiler-internal errors.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: DiagCode,
    pub message: String,
    pub span: Option<Span>,
    pub phase: Option<&'static str>,
}

impl Diagnostic {
    /// Create a source-originating diagnostic with a mandatory span.
    pub fn source(span: Span, code: DiagCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            span: Some(span),
            phase: None,
        }
    }

    /// Create an invariant violation diagnostic (compiler bug).
    pub fn invariant(message: impl Into<String>) -> Self {
        Self {
            code: DiagCode::InvariantViolation,
            message: message.into(),
            span: None,
            phase: None,
        }
    }

    /// Create a backend I/O diagnostic.
    pub fn backend_io(message: impl Into<String>) -> Self {
        Self {
            code: DiagCode::BackendIo,
            message: message.into(),
            span: None,
            phase: None,
        }
    }

    /// Create an unsupported-syntax diagnostic with eager code resolution.
    /// The code is determined by `resolve_diag_code()` based on the message.
    pub fn unsupported(message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            code: resolve_diag_code(&message),
            message,
            span: None,
            phase: None,
        }
    }

    /// Create an unsupported-syntax diagnostic with an optional source span.
    /// Accepts both `Span` (converted to `Some(span)`) and `Option<Span>`.
    /// Resolves the diagnostic code eagerly from the message content.
    pub fn unsupported_at(span: impl Into<Option<Span>>, message: impl Into<String>) -> Self {
        let message = message.into();
        let span: Option<Span> = span.into();
        Self {
            code: resolve_diag_code(&message),
            message,
            span,
            phase: None,
        }
    }

    /// Tag this diagnostic with a pipeline phase.
    /// Returns self for chaining: `diag.with_phase("parser")`.
    pub fn with_phase(mut self, phase: &'static str) -> Self {
        self.phase = Some(phase);
        self
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.display_code();
        match self.phase {
            Some(phase) => match &self.span {
                Some(span) => write!(
                    f,
                    "[{:?}/{}] {} at {}..{}",
                    code, phase, self.message, span.start, span.end
                ),
                None => write!(f, "[{:?}/{}] {}", code, phase, self.message),
            },
            None => match &self.span {
                Some(span) => write!(
                    f,
                    "[{:?}] {} at {}..{}",
                    code, self.message, span.start, span.end
                ),
                None => write!(f, "[{:?}] {}", code, self.message),
            },
        }
    }
}

/// Resolve the appropriate `DiagCode` from a diagnostic message at construction time.
///
/// This centralises the message-to-code mapping so that all emit sites (frontend,
/// IR, compiler, backend) set a precise code eagerly rather than relying on
/// `display_code()` string matching at render time.
pub fn resolve_diag_code(message: &str) -> DiagCode {
    let lower = message.to_ascii_lowercase();

    if lower.contains("issue-055")
        || lower.contains("module resolution")
        || lower.contains("module loading")
        || lower.contains(" import")
        || lower.contains(" export")
        || lower.contains("require(")
    {
        return DiagCode::UnsupportedModule;
    }

    if lower.contains("issue-050") || contains_ascii_word(&lower, "date") {
        return DiagCode::UnsupportedDate;
    }

    if lower.contains("regexp")
        || lower.contains("regular expression")
        || lower.contains("issue-051")
    {
        return DiagCode::UnsupportedRegExp;
    }

    if lower.contains("return statement is not valid in eval source") {
        return DiagCode::UnsupportedSyntax;
    }

    if lower.contains("eval") || lower.contains("issue-302") || lower.contains("issue-347") {
        return DiagCode::UnsupportedEval;
    }

    if lower.contains("typescript")
        || lower.contains("type annotation")
        || lower.contains("type directive")
        || lower.contains("reference types")
        || lower.contains("interface")
        || lower.contains(" enum")
        || contains_ascii_word(&lower, "declare")
        || lower.contains("ambient")
        || lower.contains("parameter propert")
        || lower.contains("type alias")
        || lower.contains("decorator")
    {
        return DiagCode::UnsupportedTypeScriptSyntax;
    }

    if lower.contains("array.prototype")
        || lower.contains("object.")
        || lower.contains("object prototype")
        || lower.contains("string.prototype")
        || lower.contains("function.prototype")
        || lower.contains("json.")
        || lower.contains("math.")
        || lower.contains("number.")
        || lower.contains("boolean.")
        || lower.contains("console.")
        || lower.contains("process.")
        || lower.contains("bigint.")
        || lower.contains("bigint(")
        || lower.contains("bigint is not a constructor")
        || lower.contains("globalthis")
        || lower.contains("builtin")
    {
        return DiagCode::UnsupportedBuiltin;
    }

    if lower.contains("runtime slice")
        || lower.contains("runtime subset")
        || lower.contains("runtime semantics")
        || lower.contains("literal runtime slice")
        || lower.contains("literal-folding slice")
        || lower.contains("coercion slice")
        || lower.contains("in this subset")
        || lower.contains("in this slice")
        || lower.contains("not implemented in")
        || lower.contains("is not implemented")
        || lower.contains("are not implemented")
    {
        return DiagCode::UnsupportedRuntimeSubset;
    }

    DiagCode::UnsupportedSyntax
}

impl Diagnostic {
    pub fn display_code(&self) -> DiagCode {
        if self.code != DiagCode::UnsupportedSyntax {
            return self.code;
        }
        resolve_diag_code(&self.message)
    }
}

pub(crate) fn contains_ascii_word(haystack: &str, needle: &str) -> bool {
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
    /// Two labels share the same name in the same label scope.
    DuplicateLabel,
    /// A label referenced in break/continue was not declared in any enclosing scope.
    UnresolvedLabel,
    /// A name binding is a class declaration and cannot be assigned to.
    ClassSideEffect,
    /// A number literal is outside small-int tagged range.
    NumberOutOfRange,
    /// A function call passes the wrong number of arguments.
    ArityMismatch,
    /// `return` is used in top-level script scope.
    InvalidTopLevelReturn,
    /// A lowered IR node violates a structural invariant — this is a compiler bug.
    InvariantViolation,
    /// Source code contains invalid ECMAScript syntax.
    SyntaxError,
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
    /// Target execution environment is not supported by the current backend.
    UnsupportedTarget,
}

#[cfg(test)]
mod tests {
    use super::{DiagCode, Diagnostic};
    use ts2wasm_source::Span;

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
                "return statement is not valid in eval source",
                DiagCode::UnsupportedSyntax,
                "[UnsupportedSyntax]",
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

    #[test]
    fn direct_code_variants_render_correctly() {
        let cases = [
            (
                "duplicate label `foo`",
                DiagCode::DuplicateLabel,
                "[DuplicateLabel]",
            ),
            (
                "undefined break label `foo`",
                DiagCode::UnresolvedLabel,
                "[UnresolvedLabel]",
            ),
            (
                "cannot assign to `Foo` because it is a class declaration",
                DiagCode::ClassSideEffect,
                "[ClassSideEffect]",
            ),
        ];

        for (message, code, rendered) in cases {
            let diagnostic = Diagnostic {
                code,
                message: message.to_owned(),
                span: Some(Span { start: 1, end: 2 }),
                phase: None,
            };
            assert_eq!(diagnostic.display_code(), code);
            assert!(diagnostic.to_string().contains(rendered));
        }
    }
}

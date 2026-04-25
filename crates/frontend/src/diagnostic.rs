/// Structured diagnostic emitted by compiler phases.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: DiagCode,
    pub message: String,
    pub span: Option<Span>,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.span {
            Some(span) => write!(
                f,
                "[{:?}] {} at {}..{}",
                self.code, self.message, span.start, span.end
            ),
            None => write!(f, "[{:?}] {}", self.code, self.message),
        }
    }
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
    /// I/O or command execution failure at the backend boundary.
    BackendIo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

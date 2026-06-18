use crate::expr::SemExpr;
use crate::reference::SemReference;
use crate::value::{EnvRef, LocalId, ValueRef};
use ts2wasm_source::Span;

#[derive(Debug, Clone)]
pub enum ContextKind {
    Script,
    Function { function_id: u32 },
    Eval { strict: bool },
    Module,
}

#[derive(Debug, Clone)]
pub enum SemStmt {
    // — Basic operations —
    Let { local: LocalId, init: Option<SemExpr>, span: Span },
    Assign { local: LocalId, value: SemExpr, span: Span },
    Expr(SemExpr, Span),

    // — Environment operations —
    CreateLexicalBinding { env: EnvRef, name: String, span: Span },
    InitializeBinding { env: EnvRef, name: String, value: ValueRef, span: Span },
    GetBindingValue { env: EnvRef, name: String, result: LocalId, span: Span },
    SetMutableBinding { env: EnvRef, name: String, value: ValueRef, span: Span },

    // — Reference operations —
    GetValue { reference: SemReference, result: LocalId, span: Span },
    PutValue { reference: SemReference, value: ValueRef, span: Span },
    ResolveBinding { name: String, env: EnvRef, result: LocalId, span: Span },
    MakeReference {
        base: ValueRef,
        name: SemReference,
        strict: bool,
        result: LocalId,
        span: Span,
    },

    // — Context operations —
    EnterContext { kind: ContextKind, span: Span },
    LeaveContext(Span),

    // — Iterator operations —
    IteratorNext { iterator: ValueRef, result: LocalId, span: Span },
    IteratorClose { iterator: ValueRef, completion: ValueRef, span: Span },
}

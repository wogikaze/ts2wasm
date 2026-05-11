use std::collections::HashMap;

use ts2wasm_syntax::{BinaryOp, UnaryOp};
use ts2wasm_diagnostic::{DiagCode, Diagnostic};
use ts2wasm_source::Span;

use crate::builtin::{BuiltinId, BuiltinPropertyId};
use crate::builtin_resolved::{ResolvedArrayElement, ResolvedExpr, ResolvedParam, ResolvedStmt};

// ---------------------------------------------------------------------------
// Completion Record types (ECMAScript [[Type]] / [[Value]] / [[Target]])
//
// See docs/22-completion-records.md for the full design.
// ---------------------------------------------------------------------------

/// Completion status codes corresponding to ECMAScript [[Type]].
///
/// These discriminants match the runtime convention used in WAT emission:
/// Normal=0, Return=1, Throw=2, Break=3, Continue=4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionStatus {
    Normal = 0,
    Return = 1,
    Throw = 2,
    Break = 3,
    Continue = 4,
}

/// Sentinel value meaning "no label target" for [[Target]].
pub const TARGET_EMPTY: i32 = 0;

/// A label identifier (1-based; 0 = TARGET_EMPTY).
pub type LabelId = i32;

/// Sentinel for the "empty" completion value (not the same as undefined).
///
/// The jsval i64 space is large enough to reserve one sentinel. This value
/// is never observable by user JavaScript code — it appears only during
/// intermediate completion propagation.
pub const JSVAL_EMPTY: i64 = i64::MIN;

/// An ECMAScript Completion Record.
///
/// Every statement lowering logically returns a `CompletionRecord`. The
/// three fields correspond to [[Type]], [[Value]], and [[Target]] in the
/// specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompletionRecord {
    pub status: CompletionStatus,
    pub value: i64,
    pub target: i32,
}

impl CompletionRecord {
    /// `NormalCompletion(value)` — normal completion with the given value.
    pub const fn normal(value: i64) -> Self {
        Self {
            status: CompletionStatus::Normal,
            value,
            target: TARGET_EMPTY,
        }
    }

    /// `ReturnCompletion(value)` — abrupt return completion.
    pub const fn return_completion(value: i64) -> Self {
        Self {
            status: CompletionStatus::Return,
            value,
            target: TARGET_EMPTY,
        }
    }

    /// `ThrowCompletion(value)` — abrupt throw completion.
    pub const fn throw_completion(value: i64) -> Self {
        Self {
            status: CompletionStatus::Throw,
            value,
            target: TARGET_EMPTY,
        }
    }

    /// `BreakCompletion(target)` — abrupt break completion.
    ///
    /// The value is always `JSVAL_EMPTY`; use `update_empty` to fill it.
    pub const fn break_completion(target: i32) -> Self {
        Self {
            status: CompletionStatus::Break,
            value: JSVAL_EMPTY,
            target,
        }
    }

    /// `ContinueCompletion(target)` — abrupt continue completion.
    ///
    /// The value is always `JSVAL_EMPTY`; use `update_empty` to fill it.
    pub const fn continue_completion(target: i32) -> Self {
        Self {
            status: CompletionStatus::Continue,
            value: JSVAL_EMPTY,
            target,
        }
    }

    /// `UpdateEmpty(cr, defaultValue)` — replace `JSVAL_EMPTY` with `defaultValue`.
    ///
    /// Returns `self` unchanged when the value is already non-empty.
    pub const fn update_empty(self, default_value: i64) -> Self {
        if self.value == JSVAL_EMPTY {
            Self {
                value: default_value,
                ..self
            }
        } else {
            self
        }
    }

    /// Returns `true` when this is an abrupt completion (status != Normal).
    pub const fn is_abrupt(self) -> bool {
        !matches!(self.status, CompletionStatus::Normal)
    }
}

// ---------------------------------------------------------------------------
// HIR types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirLocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirFunctionId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirProgram {
    pub body: Vec<HirStmt>,
    pub locals: Vec<HirLocalId>,
    pub functions: Vec<HirFunction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HirFunction {
    pub id: HirFunctionId,
    pub params: Vec<HirLocalId>,
    pub locals: Vec<HirLocalId>,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirStmt {
    Let {
        local: HirLocalId,
        init: HirExpr,
    },
    StoreLocal {
        local: HirLocalId,
        value: HirExpr,
    },
    Expr(HirExpr),
    BranchIfTruthy {
        condition: HirExpr,
        then_body: Vec<HirStmt>,
        else_body: Vec<HirStmt>,
    },
    LoopWhile {
        condition: HirExpr,
        body: Vec<HirStmt>,
    },
    Return(HirExpr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HirExpr {
    ConstUndefined,
    ConstNull,
    ConstBool(bool),
    ConstNumber(i32),
    ConstBigInt(String),
    ConstString(String),
    LoadLocal(HirLocalId),
    LoadBuiltin(String),
    ToBoolean(Box<HirExpr>),
    JsUnaryNot(Box<HirExpr>),
    JsAdd {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    JsStrictEqual {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    JsAbstractEqual {
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    JsRelational {
        op: HirRelationalOp,
        left: Box<HirExpr>,
        right: Box<HirExpr>,
    },
    GetProp {
        object: Box<HirExpr>,
        key: String,
    },
    GetIndex {
        object: Box<HirExpr>,
        index: Box<HirExpr>,
    },
    ArrayLength(Box<HirExpr>),
    CallBuiltin {
        builtin: BuiltinId,
        args: Vec<HirExpr>,
    },
    CallFunction {
        function: HirFunctionId,
        args: Vec<HirExpr>,
    },
    CallMethod {
        receiver: Box<HirExpr>,
        method: String,
        args: Vec<HirExpr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HirRelationalOp {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub fn lower_to_hir(program: &[ResolvedStmt]) -> Result<HirProgram, Diagnostic> {
    let function_ids = collect_function_ids(program)?;
    let mut lowerer = HirLowerer::new(&function_ids);
    let mut body = Vec::new();
    let mut functions = Vec::new();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function {
                name,
                params,
                body,
                is_generator,
                ..
            } => {
                // Skip bodyless TypeScript overload signatures, but not
                // generator functions (parser erases generator body but we
                // still need to register them for call-site resolution).
                if body.is_empty() && !is_generator {
                    continue;
                }
                let id = function_ids[name.as_str()];
                functions.push(lower_function(id, params, body, &function_ids)?);
            }
            _ => body.push(lowerer.lower_stmt(stmt)?),
        }
    }

    Ok(HirProgram {
        body,
        locals: lowerer.locals,
        functions,
    })
}

pub fn validate_hir(program: &HirProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    validate_function_ids(program, &mut errors);
    let top_context = ValidationContext {
        locals: &program.locals,
        functions_len: program.functions.len(),
        allow_return: false,
    };
    validate_stmts(&program.body, top_context, &mut errors);

    for (index, function) in program.functions.iter().enumerate() {
        if function.id.0 != index {
            errors.push(invariant(format!(
                "function id {:?} does not match function table index {index}",
                function.id
            )));
        }
        for param in &function.params {
            if !function.locals.contains(param) {
                errors.push(invariant(format!(
                    "function {:?} parameter {:?} is not present in locals",
                    function.id, param
                )));
            }
        }
        let context = ValidationContext {
            locals: &function.locals,
            functions_len: program.functions.len(),
            allow_return: true,
        };
        validate_stmts(&function.body, context, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TypeScriptFunctionArity {
    required: usize,
    max: Option<usize>,
}

/// Validate TypeScript-style direct calls to resolved user functions.
///
/// Runtime lowering still preserves JavaScript call behavior by padding missing
/// arguments and dropping extras where the current ABI requires it. This pass is
/// for `.ts` semantic checking before that runtime adaptation loses the source
/// call arity.
pub fn validate_typescript_call_arity(program: &[ResolvedStmt]) -> Result<(), Diagnostic> {
    TypeScriptCallArityValidator::default().validate_lexical_block(program)
}

#[derive(Default)]
struct TypeScriptCallArityValidator {
    scopes: Vec<HashMap<String, TypeScriptFunctionArity>>,
}

impl TypeScriptCallArityValidator {
    fn validate_lexical_block(&mut self, statements: &[ResolvedStmt]) -> Result<(), Diagnostic> {
        self.scopes.push(collect_function_arities(statements));
        let result = self.validate_stmts(statements);
        self.scopes.pop();
        result
    }

    fn validate_with_scope(
        &mut self,
        scope: HashMap<String, TypeScriptFunctionArity>,
        f: impl FnOnce(&mut Self) -> Result<(), Diagnostic>,
    ) -> Result<(), Diagnostic> {
        self.scopes.push(scope);
        let result = f(self);
        self.scopes.pop();
        result
    }

    fn validate_stmts(&mut self, statements: &[ResolvedStmt]) -> Result<(), Diagnostic> {
        for statement in statements {
            self.validate_stmt(statement)?;
        }
        Ok(())
    }

    fn validate_stmt(&mut self, statement: &ResolvedStmt) -> Result<(), Diagnostic> {
        match statement {
            ResolvedStmt::AmbientValue(_)
            | ResolvedStmt::Break { .. }
            | ResolvedStmt::Continue { .. } => {}
            ResolvedStmt::Let(_, expr)
            | ResolvedStmt::Assign(_, expr)
            | ResolvedStmt::Expr(expr)
            | ResolvedStmt::Return(expr)
            | ResolvedStmt::Throw(expr) => self.validate_expr(expr)?,
            ResolvedStmt::DestructureLet { expr, .. } => self.validate_expr(expr)?,
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => {
                self.validate_expr(condition)?;
                self.validate_lexical_block(then_body)?;
                self.validate_lexical_block(else_body)?;
            }
            ResolvedStmt::While { condition, body } => {
                self.validate_expr(condition)?;
                self.validate_lexical_block(body)?;
            }
            ResolvedStmt::Function { body, .. } => {
                self.validate_lexical_block(body)?;
            }
            ResolvedStmt::TryCatch {
                try_block,
                catch_block,
                finally_block,
                ..
            } => {
                self.validate_lexical_block(try_block)?;
                if let Some(catch_block) = catch_block {
                    self.validate_lexical_block(catch_block)?;
                }
                if let Some(finally_block) = finally_block {
                    self.validate_lexical_block(finally_block)?;
                }
            }
            ResolvedStmt::Switch { expr, cases } => {
                self.validate_expr(expr)?;
                for (case_expr, body) in cases {
                    if let Some(case_expr) = case_expr {
                        self.validate_expr(case_expr)?;
                    }
                    self.validate_lexical_block(body)?;
                }
            }
            ResolvedStmt::DoWhile { body, condition } => {
                self.validate_lexical_block(body)?;
                self.validate_expr(condition)?;
            }
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init) = init {
                    self.validate_stmt(init)?;
                }
                if let Some(condition) = condition {
                    self.validate_expr(condition)?;
                }
                if let Some(update) = update {
                    self.validate_expr(update)?;
                }
                self.validate_lexical_block(body)?;
            }
            ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
                self.validate_expr(iter)?;
                self.validate_lexical_block(body)?;
            }
            ResolvedStmt::Labeled { body, .. } => self.validate_stmt(body)?,
            ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
                self.validate_expr(expr)?;
            }
            ResolvedStmt::ClassDecl {
                constructor,
                methods,
                statics,
                static_blocks,
                ..
            } => {
                if let Some((_, body)) = constructor {
                    self.validate_lexical_block(body)?;
                }
                for method in methods {
                    self.validate_lexical_block(&method.body)?;
                }
                for (_, expr) in statics {
                    self.validate_expr(expr)?;
                }
                for (_, body) in static_blocks {
                    self.validate_lexical_block(body)?;
                }
            }
            ResolvedStmt::Block { statements } => self.validate_lexical_block(statements)?,
        }
        Ok(())
    }

    fn validate_expr(&mut self, expr: &ResolvedExpr) -> Result<(), Diagnostic> {
        match expr {
            ResolvedExpr::Number(_)
            | ResolvedExpr::BigIntLiteral { .. }
            | ResolvedExpr::String(_)
            | ResolvedExpr::Bool(_)
            | ResolvedExpr::Null
            | ResolvedExpr::Undefined
            | ResolvedExpr::This { .. }
            | ResolvedExpr::NewTarget { .. }
            | ResolvedExpr::Ident(_)
            | ResolvedExpr::ModuleLoad { .. } => {}
            ResolvedExpr::Await { expr } => {
                self.validate_expr(expr)?;
            }
            ResolvedExpr::Unary { expr, .. } | ResolvedExpr::Spread(expr) => {
                self.validate_expr(expr)?;
            }
            ResolvedExpr::Binary { left, right, .. }
            | ResolvedExpr::ComputedIndex {
                object: left,
                index: right,
            }
            | ResolvedExpr::OptionalComputedIndex {
                object: left,
                index: right,
                ..
            } => {
                self.validate_expr(left)?;
                self.validate_expr(right)?;
            }
            ResolvedExpr::Ternary {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.validate_expr(condition)?;
                self.validate_expr(then_expr)?;
                self.validate_expr(else_expr)?;
            }
            ResolvedExpr::Call { callee, args, span }
            | ResolvedExpr::OptionalCall { callee, args, span } => {
                self.validate_direct_call_arity(callee, args, *span)?;
                self.validate_expr(callee)?;
                self.validate_args(args)?;
            }
            ResolvedExpr::BuiltinCall { args, .. } | ResolvedExpr::New { args, .. } => {
                self.validate_args(args)?;
            }
            ResolvedExpr::Assign { expr, .. } | ResolvedExpr::LogicalAssign { expr, .. } => {
                self.validate_expr(expr)?;
            }
            ResolvedExpr::LogicalPropertyAssign { expr, .. } => {
                self.validate_expr(expr)?;
            }
            ResolvedExpr::LogicalComputedPropertyAssign { key, expr, .. } => {
                self.validate_expr(key)?;
                self.validate_expr(expr)?;
            }
            ResolvedExpr::LogicalComputedMemberAssign {
                object, key, expr, ..
            } => {
                self.validate_expr(object)?;
                self.validate_expr(key)?;
                self.validate_expr(expr)?;
            }
            ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
                self.validate_expr(object)?;
                self.validate_expr(expr)?;
            }
            ResolvedExpr::Array(elements) => {
                for element in elements {
                    if let ResolvedArrayElement::Present(expr) = element {
                        self.validate_expr(expr)?;
                    }
                }
            }
            ResolvedExpr::Object(props) => {
                for (_, value) in props {
                    self.validate_expr(value)?;
                }
            }
            ResolvedExpr::BuiltinProperty { object, .. }
            | ResolvedExpr::PropertyAccess { object, .. }
            | ResolvedExpr::OptionalPropertyAccess { object, .. } => {
                self.validate_expr(object)?;
            }
            ResolvedExpr::MethodCall { object, args, .. } => {
                self.validate_expr(object)?;
                self.validate_args(args)?;
            }
            ResolvedExpr::PropertyAssign { object, value, .. } => {
                self.validate_expr(object)?;
                self.validate_expr(value)?;
            }
            ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
                self.validate_expr(object)?;
                self.validate_expr(key)?;
                self.validate_expr(value)?;
            }
            ResolvedExpr::ArrowFn {
                body, body_stmts, ..
            } => {
                if body_stmts.is_empty() {
                    self.validate_expr(body)?;
                } else {
                    self.validate_lexical_block(body_stmts)?;
                }
            }
            ResolvedExpr::FunctionExpr { name, params, body } => {
                let mut scope = HashMap::new();
                if !name.is_empty() {
                    scope.insert(name.clone(), function_arity(params, body));
                }
                self.validate_with_scope(scope, |validator| {
                    validator.validate_lexical_block(body)
                })?;
            }
            ResolvedExpr::ClassExpr { body, .. } => {
                self.validate_lexical_block(body)?;
            }
        }
        Ok(())
    }

    fn validate_args(&mut self, args: &[ResolvedExpr]) -> Result<(), Diagnostic> {
        for arg in args {
            self.validate_expr(arg)?;
        }
        Ok(())
    }

    fn validate_direct_call_arity(
        &self,
        callee: &ResolvedExpr,
        args: &[ResolvedExpr],
        span: Span,
    ) -> Result<(), Diagnostic> {
        if args
            .iter()
            .any(|arg| matches!(arg, ResolvedExpr::Spread(_)))
        {
            return Ok(());
        }
        let ResolvedExpr::Ident(name) = callee else {
            return Ok(());
        };
        let Some(signature) = self.resolve_function_arity(name) else {
            return Ok(());
        };
        let got = args.len();
        if got < signature.required || signature.max.is_some_and(|max| got > max) {
            return Err(Diagnostic {
                code: DiagCode::ArityMismatch,
                message: format_typescript_arity_message(signature, got),
                span: Some(span),

                phase: None,
            });
        }
        Ok(())
    }

    fn resolve_function_arity(&self, name: &str) -> Option<TypeScriptFunctionArity> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }
}

fn collect_function_arities(
    statements: &[ResolvedStmt],
) -> HashMap<String, TypeScriptFunctionArity> {
    statements
        .iter()
        .filter_map(|statement| match statement {
            ResolvedStmt::Function {
                name, params, body, ..
            } => Some((name.clone(), function_arity(params, body))),
            _ => None,
        })
        .collect()
}

fn function_arity(params: &[ResolvedParam], body: &[ResolvedStmt]) -> TypeScriptFunctionArity {
    let reads_implicit_arguments =
        block_contains_arguments(body) && !params.iter().any(|param| param.name == "arguments");
    TypeScriptFunctionArity {
        required: params
            .iter()
            .filter(|param| param.default.is_none() && !param.is_rest)
            .count(),
        max: if reads_implicit_arguments || params.iter().any(|param| param.is_rest) {
            None
        } else {
            Some(params.len())
        },
    }
}

fn block_contains_arguments(stmts: &[ResolvedStmt]) -> bool {
    stmts.iter().any(stmt_contains_arguments)
}

fn stmt_contains_arguments(stmt: &ResolvedStmt) -> bool {
    match stmt {
        ResolvedStmt::AmbientValue(_)
        | ResolvedStmt::Break { .. }
        | ResolvedStmt::Continue { .. } => false,
        ResolvedStmt::Let(_, expr)
        | ResolvedStmt::Assign(_, expr)
        | ResolvedStmt::Expr(expr)
        | ResolvedStmt::Return(expr)
        | ResolvedStmt::Throw(expr)
        | ResolvedStmt::DestructureLet { expr, .. } => expr_contains_arguments(expr),
        ResolvedStmt::Export { expr, .. } | ResolvedStmt::ModuleExportsAssign { expr } => {
            expr_contains_arguments(expr)
        }
        ResolvedStmt::Function { .. } => false,
        ResolvedStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            expr_contains_arguments(condition)
                || block_contains_arguments(then_body)
                || block_contains_arguments(else_body)
        }
        ResolvedStmt::While { condition, body } | ResolvedStmt::DoWhile { condition, body } => {
            expr_contains_arguments(condition) || block_contains_arguments(body)
        }
        ResolvedStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            init.as_ref()
                .is_some_and(|stmt| stmt_contains_arguments(stmt))
                || condition.as_ref().is_some_and(expr_contains_arguments)
                || update.as_ref().is_some_and(expr_contains_arguments)
                || block_contains_arguments(body)
        }
        ResolvedStmt::ForIn { iter, body, .. } | ResolvedStmt::ForOf { iter, body, .. } => {
            expr_contains_arguments(iter) || block_contains_arguments(body)
        }
        ResolvedStmt::TryCatch {
            try_block,
            catch_block,
            finally_block,
            ..
        } => {
            block_contains_arguments(try_block)
                || catch_block
                    .as_ref()
                    .is_some_and(|block| block_contains_arguments(block))
                || finally_block
                    .as_ref()
                    .is_some_and(|block| block_contains_arguments(block))
        }
        ResolvedStmt::Switch { expr, cases } => {
            expr_contains_arguments(expr)
                || cases.iter().any(|(case_expr, body)| {
                    case_expr.as_ref().is_some_and(expr_contains_arguments)
                        || block_contains_arguments(body)
                })
        }
        ResolvedStmt::Labeled { body, .. } => stmt_contains_arguments(body),
        ResolvedStmt::Block { statements, .. } => block_contains_arguments(statements),
        ResolvedStmt::ClassDecl { .. } => false,
    }
}

fn expr_contains_arguments(expr: &ResolvedExpr) -> bool {
    match expr {
        ResolvedExpr::Ident(name) => name == "arguments",
        ResolvedExpr::This { .. }
        | ResolvedExpr::NewTarget { .. }
        | ResolvedExpr::Number(_)
        | ResolvedExpr::BigIntLiteral { .. }
        | ResolvedExpr::String(_)
        | ResolvedExpr::Bool(_)
        | ResolvedExpr::Null
        | ResolvedExpr::Undefined
        | ResolvedExpr::ModuleLoad { .. } => false,
        ResolvedExpr::Await { expr } => expr_contains_arguments(expr),
        ResolvedExpr::Unary { expr, .. } => expr_contains_arguments(expr),
        ResolvedExpr::Binary { left, right, .. } => {
            expr_contains_arguments(left) || expr_contains_arguments(right)
        }
        ResolvedExpr::Ternary {
            condition,
            then_expr,
            else_expr,
            ..
        } => {
            expr_contains_arguments(condition)
                || expr_contains_arguments(then_expr)
                || expr_contains_arguments(else_expr)
        }
        ResolvedExpr::Call { callee, args, .. }
        | ResolvedExpr::OptionalCall { callee, args, .. } => {
            expr_contains_arguments(callee) || args.iter().any(expr_contains_arguments)
        }
        ResolvedExpr::New { args, .. } => args.iter().any(expr_contains_arguments),
        ResolvedExpr::Assign { name, expr } => name == "arguments" || expr_contains_arguments(expr),
        ResolvedExpr::PropertyAssign { object, value, .. } => {
            expr_contains_arguments(object) || expr_contains_arguments(value)
        }
        ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
            expr_contains_arguments(object)
                || expr_contains_arguments(key)
                || expr_contains_arguments(value)
        }
        ResolvedExpr::Array(elements) => elements.iter().any(|element| match element {
            ResolvedArrayElement::Present(expr) => expr_contains_arguments(expr),
            ResolvedArrayElement::Hole => false,
        }),
        ResolvedExpr::Object(props) => props
            .iter()
            .any(|(_, value)| expr_contains_arguments(value)),
        ResolvedExpr::PropertyAccess { object, .. }
        | ResolvedExpr::OptionalPropertyAccess { object, .. } => expr_contains_arguments(object),
        ResolvedExpr::ComputedIndex { object, index }
        | ResolvedExpr::OptionalComputedIndex { object, index, .. } => {
            expr_contains_arguments(object) || expr_contains_arguments(index)
        }
        ResolvedExpr::BuiltinCall { args, .. } => args.iter().any(expr_contains_arguments),
        ResolvedExpr::BuiltinProperty { object, .. } => expr_contains_arguments(object),
        ResolvedExpr::MethodCall { object, args, .. } => {
            expr_contains_arguments(object) || args.iter().any(expr_contains_arguments)
        }
        ResolvedExpr::LogicalAssign { expr, .. } => expr_contains_arguments(expr),
        ResolvedExpr::LogicalPropertyAssign {
            object, key, expr, ..
        } => object == "arguments" || key == "arguments" || expr_contains_arguments(expr),
        ResolvedExpr::LogicalComputedPropertyAssign {
            object, key, expr, ..
        } => object == "arguments" || expr_contains_arguments(key) || expr_contains_arguments(expr),
        ResolvedExpr::LogicalMemberAssign { object, expr, .. } => {
            expr_contains_arguments(object) || expr_contains_arguments(expr)
        }
        ResolvedExpr::LogicalComputedMemberAssign {
            object, key, expr, ..
        } => {
            expr_contains_arguments(object)
                || expr_contains_arguments(key)
                || expr_contains_arguments(expr)
        }
        ResolvedExpr::Spread(expr) => expr_contains_arguments(expr),
        ResolvedExpr::FunctionExpr { .. } => false,
        ResolvedExpr::ArrowFn { body, .. } => expr_contains_arguments(body),
        ResolvedExpr::ClassExpr { .. } => false,
    }
}

fn format_typescript_arity_message(signature: TypeScriptFunctionArity, got: usize) -> String {
    match signature.max {
        Some(max) if max == signature.required => {
            format!(
                "TS2554: Expected {} arguments, but got {got}.",
                signature.required
            )
        }
        Some(max) => format!(
            "TS2554: Expected {}-{} arguments, but got {got}.",
            signature.required, max
        ),
        None => format!(
            "TS2554: Expected at least {} arguments, but got {got}.",
            signature.required
        ),
    }
}

#[derive(Clone, Copy)]
struct ValidationContext<'a> {
    locals: &'a [HirLocalId],
    functions_len: usize,
    allow_return: bool,
}

fn validate_function_ids(program: &HirProgram, errors: &mut Vec<Diagnostic>) {
    for (index, local) in program.locals.iter().enumerate() {
        if local.0 != index {
            errors.push(invariant(format!(
                "top-level local id {:?} does not match locals index {index}",
                local
            )));
        }
    }
}

fn validate_stmts(stmts: &[HirStmt], context: ValidationContext<'_>, errors: &mut Vec<Diagnostic>) {
    for stmt in stmts {
        match stmt {
            HirStmt::Let { local, init } | HirStmt::StoreLocal { local, value: init } => {
                validate_local(*local, context, errors);
                validate_expr(init, context, errors);
            }
            HirStmt::Expr(expr) => validate_expr(expr, context, errors),
            HirStmt::BranchIfTruthy {
                condition,
                then_body,
                else_body,
            } => {
                if !matches!(condition, HirExpr::ToBoolean(_)) {
                    errors.push(invariant(
                        "BranchIfTruthy condition must be explicit ToBoolean".to_owned(),
                    ));
                }
                validate_expr(condition, context, errors);
                validate_stmts(then_body, context, errors);
                validate_stmts(else_body, context, errors);
            }
            HirStmt::LoopWhile { condition, body } => {
                if !matches!(condition, HirExpr::ToBoolean(_)) {
                    errors.push(invariant(
                        "LoopWhile condition must be explicit ToBoolean".to_owned(),
                    ));
                }
                validate_expr(condition, context, errors);
                validate_stmts(body, context, errors);
            }
            HirStmt::Return(expr) => {
                if !context.allow_return {
                    errors.push(invariant("top-level Return is not valid HIR".to_owned()));
                }
                validate_expr(expr, context, errors);
            }
        }
    }
}

fn validate_expr(expr: &HirExpr, context: ValidationContext<'_>, errors: &mut Vec<Diagnostic>) {
    match expr {
        HirExpr::LoadLocal(local) => validate_local(*local, context, errors),
        HirExpr::LoadBuiltin(_) => {}
        HirExpr::ToBoolean(expr) | HirExpr::JsUnaryNot(expr) | HirExpr::ArrayLength(expr) => {
            validate_expr(expr, context, errors);
        }
        HirExpr::JsAdd { left, right }
        | HirExpr::JsStrictEqual { left, right }
        | HirExpr::JsAbstractEqual { left, right }
        | HirExpr::JsRelational { left, right, .. }
        | HirExpr::GetIndex {
            object: left,
            index: right,
        } => {
            validate_expr(left, context, errors);
            validate_expr(right, context, errors);
        }
        HirExpr::GetProp { object, .. } => validate_expr(object, context, errors),
        HirExpr::CallBuiltin { args, .. } => {
            for arg in args {
                validate_expr(arg, context, errors);
            }
        }
        HirExpr::CallFunction { function, args } => {
            if function.0 >= context.functions_len {
                errors.push(invariant(format!("invalid function id {:?}", function)));
            }
            for arg in args {
                validate_expr(arg, context, errors);
            }
        }
        HirExpr::CallMethod { receiver, args, .. } => {
            validate_expr(receiver, context, errors);
            for arg in args {
                validate_expr(arg, context, errors);
            }
        }
        HirExpr::ConstUndefined
        | HirExpr::ConstNull
        | HirExpr::ConstBool(_)
        | HirExpr::ConstNumber(_)
        | HirExpr::ConstBigInt(_)
        | HirExpr::ConstString(_) => {}
    }
}

fn validate_local(local: HirLocalId, context: ValidationContext<'_>, errors: &mut Vec<Diagnostic>) {
    if !context.locals.contains(&local) {
        errors.push(invariant(format!("invalid local id {:?}", local)));
    }
}

fn invariant(message: String) -> Diagnostic {
    Diagnostic {
        code: DiagCode::InvariantViolation,
        message,
        span: None,

        phase: None,
    }
}

fn collect_function_ids(
    program: &[ResolvedStmt],
) -> Result<HashMap<String, HirFunctionId>, Diagnostic> {
    let mut ids = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::Function {
            name,
            body,
            is_generator,
            ..
        } = stmt
        {
            // Skip bodyless overload signatures, but not generator functions
            // (generator bodies are erased by the parser, we still need to
            // register them in function_ids so calls to generator functions
            // are resolved).
            if body.is_empty() && !is_generator {
                continue;
            }
            if ids.contains_key(name.as_str()) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateFunction,
                    message: format!("duplicate function definition: `{name}`"),
                    span: None,

                    phase: None,
                });
            }
            ids.insert(name.clone(), HirFunctionId(ids.len()));
        }
    }
    Ok(ids)
}

fn lower_function(
    id: HirFunctionId,
    params: &[ResolvedParam],
    body: &[ResolvedStmt],
    function_ids: &HashMap<String, HirFunctionId>,
) -> Result<HirFunction, Diagnostic> {
    let mut lowerer = HirLowerer::new(function_ids);
    let mut param_ids = Vec::new();
    for param in params {
        param_ids.push(lowerer.declare_local(&param.name)?);
    }
    let body = lowerer.lower_block(body)?;
    Ok(HirFunction {
        id,
        params: param_ids,
        locals: lowerer.locals,
        body,
    })
}

struct HirLowerer<'a> {
    function_ids: &'a HashMap<String, HirFunctionId>,
    scopes: Vec<HashMap<String, HirLocalId>>,
    locals: Vec<HirLocalId>,
}

impl<'a> HirLowerer<'a> {
    fn new(function_ids: &'a HashMap<String, HirFunctionId>) -> Self {
        Self {
            function_ids,
            scopes: vec![HashMap::new()],
            locals: Vec::new(),
        }
    }

    fn lower_block(&mut self, statements: &[ResolvedStmt]) -> Result<Vec<HirStmt>, Diagnostic> {
        let mut lowered = Vec::with_capacity(statements.len());
        for statement in statements {
            lowered.push(self.lower_stmt(statement)?);
        }
        Ok(lowered)
    }

    fn lower_nested_block(
        &mut self,
        statements: &[ResolvedStmt],
    ) -> Result<Vec<HirStmt>, Diagnostic> {
        self.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.scopes.pop();
        lowered
    }

    fn lower_stmt(&mut self, stmt: &ResolvedStmt) -> Result<HirStmt, Diagnostic> {
        match stmt {
            ResolvedStmt::Let(name, expr) => {
                let local = self.declare_local(name)?;
                Ok(HirStmt::Let {
                    local,
                    init: self.lower_expr(expr)?,
                })
            }
            ResolvedStmt::Assign(name, expr) => {
                let local = self.resolve_local(name)?;
                Ok(HirStmt::StoreLocal {
                    local,
                    value: self.lower_expr(expr)?,
                })
            }
            ResolvedStmt::Expr(expr) => Ok(HirStmt::Expr(self.lower_expr(expr)?)),
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => Ok(HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(Box::new(self.lower_expr(condition)?)),
                then_body: self.lower_nested_block(then_body)?,
                else_body: self.lower_nested_block(else_body)?,
            }),
            ResolvedStmt::While { condition, body } => Ok(HirStmt::LoopWhile {
                condition: HirExpr::ToBoolean(Box::new(self.lower_expr(condition)?)),
                body: self.lower_nested_block(body)?,
            }),
            ResolvedStmt::Return(expr) => Ok(HirStmt::Return(self.lower_expr(expr)?)),
            ResolvedStmt::Function { .. } => Err(unsupported("nested function declarations")),
            _ => Err(unsupported(
                "statement kind is not part of the initial HIR slice",
            )),
        }
    }

    fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<HirExpr, Diagnostic> {
        match expr {
            ResolvedExpr::Number(value) => Ok(HirExpr::ConstNumber(*value)),
            ResolvedExpr::BigIntLiteral { decimal, .. } => {
                Ok(HirExpr::ConstBigInt(decimal.clone()))
            }
            ResolvedExpr::String(value) => Ok(HirExpr::ConstString(value.clone())),
            ResolvedExpr::Bool(value) => Ok(HirExpr::ConstBool(*value)),
            ResolvedExpr::Null => Ok(HirExpr::ConstNull),
            ResolvedExpr::Undefined => Ok(HirExpr::ConstUndefined),
            ResolvedExpr::Ident(name) => match self.resolve_local(name) {
                Ok(local) => Ok(HirExpr::LoadLocal(local)),
                Err(_) => Ok(HirExpr::LoadBuiltin(name.clone())),
            },
            ResolvedExpr::Unary {
                op: UnaryOp::Not,
                expr,
            } => Ok(HirExpr::JsUnaryNot(Box::new(self.lower_expr(expr)?))),
            ResolvedExpr::Binary { left, op, right } => self.lower_binary(left, *op, right),
            ResolvedExpr::Ternary { .. } => Err(unsupported(
                "ternary expressions are not part of the initial HIR slice",
            )),
            ResolvedExpr::BuiltinCall { builtin, args } => Ok(HirExpr::CallBuiltin {
                builtin: *builtin,
                args: self.lower_args(args)?,
            }),
            ResolvedExpr::BuiltinProperty {
                builtin, object, ..
            } => match builtin {
                BuiltinPropertyId::Length => {
                    Ok(HirExpr::ArrayLength(Box::new(self.lower_expr(object)?)))
                }
            },
            ResolvedExpr::PropertyAccess { object, key, .. } => Ok(HirExpr::GetProp {
                object: Box::new(self.lower_expr(object)?),
                key: key.clone(),
            }),
            ResolvedExpr::ComputedIndex { object, index } => Ok(HirExpr::GetIndex {
                object: Box::new(self.lower_expr(object)?),
                index: Box::new(self.lower_expr(index)?),
            }),
            ResolvedExpr::Call { callee, args, .. } => match callee.as_ref() {
                ResolvedExpr::Ident(name) if name == "String" => {
                    let _ = args;
                    Err(unsupported("String(...) calls in initial HIR slice"))
                }
                ResolvedExpr::Ident(name) if name == "Symbol" => {
                    // Symbol() is handled by the lowered resolver (resolver_expr.rs).
                    // Return a no-op HIR expr to pass the validator.
                    Ok(HirExpr::ConstUndefined)
                }
                ResolvedExpr::Ident(name) => {
                    let function =
                        self.function_ids
                            .get(name.as_str())
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnresolvedFunction,
                                message: format!("unresolved function: `{name}`"),
                                span: None,

                                phase: None,
                            })?;
                    Ok(HirExpr::CallFunction {
                        function: *function,
                        args: self.lower_args(args)?,
                    })
                }
                _ => Err(unsupported("dynamic function calls in initial HIR slice")),
            },
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
                ..
            } => Ok(HirExpr::CallMethod {
                receiver: Box::new(self.lower_expr(object)?),
                method: method.clone(),
                args: self.lower_args(args)?,
            }),
            ResolvedExpr::Assign { .. }
            | ResolvedExpr::LogicalAssign { .. }
            | ResolvedExpr::LogicalPropertyAssign { .. }
            | ResolvedExpr::LogicalMemberAssign { .. }
            | ResolvedExpr::LogicalComputedMemberAssign { .. }
            | ResolvedExpr::LogicalComputedPropertyAssign { .. } => Err(unsupported(
                "assignment expressions are not part of the initial HIR slice",
            )),
            _ => Err(unsupported(
                "expression kind is not part of the initial HIR slice",
            )),
        }
    }

    fn lower_binary(
        &mut self,
        left: &ResolvedExpr,
        op: BinaryOp,
        right: &ResolvedExpr,
    ) -> Result<HirExpr, Diagnostic> {
        self.lower_binary_chain(left, op, right)
    }

    fn lower_binary_chain(
        &mut self,
        left: &ResolvedExpr,
        op: BinaryOp,
        right: &ResolvedExpr,
    ) -> Result<HirExpr, Diagnostic> {
        let mut chain: Vec<(BinaryOp, &ResolvedExpr)> = Vec::new();
        let mut current_left = left;
        let mut current_op = op;
        let mut current_right = right;

        loop {
            chain.push((current_op, current_right));
            if let ResolvedExpr::Binary { left, op, right } = current_left {
                current_left = left;
                current_op = *op;
                current_right = right;
            } else {
                break;
            }
        }

        let mut accumulated = self.lower_expr(current_left)?;
        while let Some((binary_op, binary_right)) = chain.pop() {
            let right = self.lower_expr(binary_right)?;
            let left = Box::new(accumulated);
            let right = Box::new(right);
            accumulated = self.lower_binary_expr(left, binary_op, right)?;
        }
        Ok(accumulated)
    }

    fn lower_binary_expr(
        &self,
        left: Box<HirExpr>,
        op: BinaryOp,
        right: Box<HirExpr>,
    ) -> Result<HirExpr, Diagnostic> {
        match op {
            BinaryOp::Add => Ok(HirExpr::JsAdd { left, right }),
            BinaryOp::StrictEqual => Ok(HirExpr::JsStrictEqual { left, right }),
            BinaryOp::EqualEqual => Ok(HirExpr::JsAbstractEqual { left, right }),
            BinaryOp::Less => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::Less,
                left,
                right,
            }),
            BinaryOp::LessEqual => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::LessEqual,
                left,
                right,
            }),
            BinaryOp::Greater => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::Greater,
                left,
                right,
            }),
            BinaryOp::GreaterEqual => Ok(HirExpr::JsRelational {
                op: HirRelationalOp::GreaterEqual,
                left,
                right,
            }),
            _ => Err(unsupported(
                "binary operator is not part of the initial HIR slice",
            )),
        }
    }

    fn lower_args(&mut self, args: &[ResolvedExpr]) -> Result<Vec<HirExpr>, Diagnostic> {
        args.iter().map(|arg| self.lower_expr(arg)).collect()
    }

    fn declare_local(&mut self, name: &str) -> Result<HirLocalId, Diagnostic> {
        if let Some(&existing) = self.scopes.last().expect("scope must exist").get(name) {
            return Ok(existing);
        }
        let local = HirLocalId(self.locals.len());
        self.locals.push(local);
        self.scopes
            .last_mut()
            .expect("scope must exist")
            .insert(name.to_owned(), local);
        Ok(local)
    }

    fn resolve_local(&self, name: &str) -> Result<HirLocalId, Diagnostic> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: None,

                phase: None,
            })
    }
}

fn unsupported(message: &str) -> Diagnostic {
    Diagnostic {
        code: DiagCode::UnsupportedSyntax,
        message: message.to_owned(),
        span: None,

        phase: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ts2wasm_frontend::{Lexer, Parser};

    fn parse_to_hir(source: &str) -> HirProgram {
        let tokens = Lexer::new(source).tokenize().unwrap();
        let ast = Parser::new(tokens, source).parse_program().unwrap();
        let named = crate::name_resolver::resolve_names(&ast).unwrap();
        let resolved = crate::builtin_resolver::resolve_builtins(&named).unwrap();
        lower_to_hir(&resolved).unwrap()
    }

    #[test]
    fn lowers_addition_to_js_add() {
        let hir = parse_to_hir("let a = 1; let b = 2; let c = a + b;");
        let HirStmt::Let { init, .. } = &hir.body[2] else {
            panic!("expected third statement to be let");
        };
        assert!(matches!(init, HirExpr::JsAdd { .. }));
    }

    #[test]
    fn lowers_if_condition_to_truthy_branch() {
        let hir = parse_to_hir("let a = 1; if (a) { console.log(\"yes\"); }");
        assert!(matches!(
            &hir.body[1],
            HirStmt::BranchIfTruthy {
                condition: HirExpr::ToBoolean(_),
                ..
            }
        ));
    }

    #[test]
    fn lowers_console_log_to_builtin_call() {
        let hir = parse_to_hir("console.log(\"ok\");");
        assert!(matches!(
            &hir.body[0],
            HirStmt::Expr(HirExpr::CallBuiltin {
                builtin: BuiltinId::ConsoleLog,
                ..
            })
        ));
    }

    #[test]
    fn validates_valid_hir() {
        let hir = parse_to_hir("let a = 1; if (a) { console.log(\"yes\"); }");
        validate_hir(&hir).unwrap();
    }

    #[test]
    fn validation_rejects_invalid_local_id() {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::LoadLocal(HirLocalId(99)))],
            locals: vec![],
            functions: vec![],
        };
        let errors = validate_hir(&hir).unwrap_err();
        assert!(errors.iter().any(|error| {
            error.code == DiagCode::InvariantViolation && error.message.contains("local id")
        }));
    }

    #[test]
    fn validation_rejects_branch_without_to_boolean() {
        let hir = HirProgram {
            body: vec![HirStmt::BranchIfTruthy {
                condition: HirExpr::ConstBool(true),
                then_body: vec![],
                else_body: vec![],
            }],
            locals: vec![],
            functions: vec![],
        };
        let errors = validate_hir(&hir).unwrap_err();
        assert!(errors.iter().any(|error| {
            error.code == DiagCode::InvariantViolation && error.message.contains("ToBoolean")
        }));
    }

    #[test]
    fn validation_rejects_invalid_function_id() {
        let hir = HirProgram {
            body: vec![HirStmt::Expr(HirExpr::CallFunction {
                function: HirFunctionId(7),
                args: vec![],
            })],
            locals: vec![],
            functions: vec![],
        };
        let errors = validate_hir(&hir).unwrap_err();
        assert!(errors.iter().any(|error| {
            error.code == DiagCode::InvariantViolation && error.message.contains("function id")
        }));
    }
}

#[cfg(test)]
mod completion_record_tests {
    use super::*;

    #[test]
    fn normal_completion() {
        let cr = CompletionRecord::normal(42);
        assert_eq!(cr.status, CompletionStatus::Normal);
        assert_eq!(cr.value, 42);
        assert_eq!(cr.target, TARGET_EMPTY);
    }

    #[test]
    fn return_completion() {
        let cr = CompletionRecord::return_completion(42);
        assert_eq!(cr.status, CompletionStatus::Return);
        assert_eq!(cr.value, 42);
        assert_eq!(cr.target, TARGET_EMPTY);
    }

    #[test]
    fn throw_completion() {
        let cr = CompletionRecord::throw_completion(99);
        assert_eq!(cr.status, CompletionStatus::Throw);
        assert_eq!(cr.value, 99);
    }

    #[test]
    fn break_completion_has_empty_value() {
        let cr = CompletionRecord::break_completion(TARGET_EMPTY);
        assert_eq!(cr.status, CompletionStatus::Break);
        assert_eq!(cr.value, JSVAL_EMPTY);
        assert_eq!(cr.target, TARGET_EMPTY);
    }

    #[test]
    fn continue_completion_has_empty_value() {
        let cr = CompletionRecord::continue_completion(42);
        assert_eq!(cr.status, CompletionStatus::Continue);
        assert_eq!(cr.value, JSVAL_EMPTY);
        assert_eq!(cr.target, 42);
    }

    #[test]
    fn update_empty_preserves_non_empty_value() {
        let cr = CompletionRecord::return_completion(10);
        let updated = cr.update_empty(99);
        assert_eq!(updated.value, 10);
        assert_eq!(updated.status, CompletionStatus::Return);
    }

    #[test]
    fn update_empty_replaces_empty_value() {
        let cr = CompletionRecord::break_completion(TARGET_EMPTY);
        let updated = cr.update_empty(99);
        assert_eq!(updated.value, 99);
        assert_eq!(updated.status, CompletionStatus::Break);
        assert_eq!(updated.target, TARGET_EMPTY);
    }

    #[test]
    fn update_empty_does_not_change_status_or_target() {
        let cr = CompletionRecord::continue_completion(7);
        let updated = cr.update_empty(0);
        assert_eq!(updated.status, CompletionStatus::Continue);
        assert_eq!(updated.target, 7);
        assert_eq!(updated.value, 0);
    }

    #[test]
    fn status_discriminants_match_design() {
        assert_eq!(CompletionStatus::Normal as i32, 0);
        assert_eq!(CompletionStatus::Return as i32, 1);
        assert_eq!(CompletionStatus::Throw as i32, 2);
        assert_eq!(CompletionStatus::Break as i32, 3);
        assert_eq!(CompletionStatus::Continue as i32, 4);
    }

    #[test]
    fn target_empty_is_zero() {
        assert_eq!(TARGET_EMPTY, 0);
    }

    #[test]
    fn jsval_empty_is_min_i64() {
        assert_eq!(JSVAL_EMPTY, i64::MIN);
    }

    #[test]
    fn is_abrupt_returns_false_for_normal() {
        let cr = CompletionRecord::normal(0);
        assert!(!cr.is_abrupt());
    }

    #[test]
    fn is_abrupt_returns_true_for_return() {
        let cr = CompletionRecord::return_completion(1);
        assert!(cr.is_abrupt());
    }

    #[test]
    fn is_abrupt_returns_true_for_throw() {
        let cr = CompletionRecord::throw_completion(1);
        assert!(cr.is_abrupt());
    }

    #[test]
    fn is_abrupt_returns_true_for_break() {
        let cr = CompletionRecord::break_completion(0);
        assert!(cr.is_abrupt());
    }

    #[test]
    fn is_abrupt_returns_true_for_continue() {
        let cr = CompletionRecord::continue_completion(0);
        assert!(cr.is_abrupt());
    }
}

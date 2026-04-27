use std::collections::HashMap;

use super::builtin::{BuiltinId, BuiltinPropertyId, BuiltinResult};
use super::builtin_resolved::{ResolvedExpr, ResolvedStmt};
use ts2wasm_frontend::{BinaryOp, DiagCode, Diagnostic, UnaryOp};
use ts2wasm_runtime_abi::ValueTag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassPrototypeRef {
    pub constructor: FuncId,
    pub parent_constructors: Vec<FuncId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleInfo {
    pub id: usize,
    pub specifier: String,
    pub statements: Vec<LoweredStmt>,
    pub locals_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredProgram {
    pub top_level_statements: Vec<LoweredStmt>,
    pub top_level_locals: Vec<LocalId>,
    pub functions: Vec<LoweredFunction>,
    pub modules: Vec<ModuleInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredFunction {
    pub id: FuncId,
    pub params: Vec<LocalId>,
    pub min_required_params: usize,
    pub locals: Vec<LocalId>,
    pub body: Vec<LoweredStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredStmt {
    Let(LocalId, LoweredExpr),
    Assign(LocalId, LoweredExpr),
    Expr(LoweredExpr),
    If {
        condition: LoweredExpr,
        then_body: Vec<LoweredStmt>,
        else_body: Vec<LoweredStmt>,
    },
    While {
        condition: LoweredExpr,
        body: Vec<LoweredStmt>,
    },
    Return(LoweredExpr),
    Throw(LoweredExpr),
    TryCatch {
        try_body: Vec<LoweredStmt>,
        catch_var: Option<LocalId>,
        catch_body: Option<Vec<LoweredStmt>>,
        finally_body: Option<Vec<LoweredStmt>>,
    },
    Switch {
        expr: LoweredExpr,
        cases: Vec<(Option<LoweredExpr>, Vec<LoweredStmt>)>,
    },
    DoWhile {
        body: Vec<LoweredStmt>,
        condition: LoweredExpr,
    },
    For {
        init: Option<Box<LoweredStmt>>,
        condition: Option<LoweredExpr>,
        update: Option<LoweredExpr>,
        body: Vec<LoweredStmt>,
    },
    ForIn {
        var: LocalId,
        iter: LoweredExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<LoweredStmt>,
    },
    ForOf {
        var: LocalId,
        iter: LoweredExpr,
        iter_local: LocalId,
        index_local: LocalId,
        len_local: LocalId,
        body: Vec<LoweredStmt>,
    },
    Labeled {
        label: String,
        body: Box<LoweredStmt>,
    },
    Break {
        label: Option<String>,
    },
    Continue {
        label: Option<String>,
    },
    Export {
        name: String,
        expr: LoweredExpr,
    },
    ModuleExportsAssign {
        expr: LoweredExpr,
    },
    ClassDecl {
        name: String,
        extends: Option<String>,
        constructor: Option<(Vec<LocalId>, Vec<LoweredStmt>)>,
        methods: Vec<(String, Vec<LocalId>, Vec<LoweredStmt>)>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCallKind {
    User(FuncId),
    Builtin(BuiltinId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredExpr {
    Number(i32),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    Local(LocalId),
    Unary {
        op: LoweredUnaryOp,
        expr: Box<LoweredExpr>,
    },
    Binary {
        left: Box<LoweredExpr>,
        op: LoweredBinaryOp,
        right: Box<LoweredExpr>,
    },
    PropertyIn {
        obj: Box<LoweredExpr>,
        key: String,
    },
    PropertyInDynamic {
        obj: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
    },
    Call {
        kind: FunctionCallKind,
        args: Vec<LoweredExpr>,
    },
    Assign {
        local: LocalId,
        expr: Box<LoweredExpr>,
    },
    ArrayNew {
        elements: Vec<LoweredExpr>,
    },
    ArrayGet {
        arr: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
    },
    Index {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
    },
    GetLength(Box<LoweredExpr>),
    ObjectNew {
        props: Vec<(String, LoweredExpr)>,
    },
    PropertyGet {
        obj: Box<LoweredExpr>,
        key: String,
    },
    PropertyGetDynamic {
        obj: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
    },
    MethodCall {
        object: Box<LoweredExpr>,
        method: String,
    },
    RuntimeCall {
        runtime_fn: String,
        args: Vec<LoweredExpr>,
    },
    PropertySet {
        object: Box<LoweredExpr>,
        key: String,
        value: Box<LoweredExpr>,
    },
    PropertyDelete {
        object: Box<LoweredExpr>,
        key: String,
    },
    PropertyDeleteDynamic {
        object: Box<LoweredExpr>,
        key: Box<LoweredExpr>,
    },
    PropertySetDynamic {
        object: Box<LoweredExpr>,
        index: Box<LoweredExpr>,
        value: Box<LoweredExpr>,
    },
    New {
        constructor: FuncId,
        prototype: ClassPrototypeRef,
        args: Vec<LoweredExpr>,
        base_local: LocalId,
    },
    ClassPrototype(ClassPrototypeRef),
    ModuleLoad {
        module_id: usize,
    },
    This,
    ArrowFn {
        func_id: FuncId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoweredBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    StrictEqual,
    EqualEqual,
    BangEqual,
    StrictNotEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoweredUnaryOp {
    Not,
    Negate,
    TypeOf,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferredType {
    Number,
    String,
    Boolean,
    Unknown,
}

impl LoweredExpr {
    pub fn inferred_type(&self) -> InferredType {
        match self {
            Self::Number(_) => InferredType::Number,
            Self::String(_) => InferredType::String,
            Self::Bool(_) => InferredType::Boolean,
            Self::Unary { op, expr } => match op {
                LoweredUnaryOp::Negate if expr.inferred_type() == InferredType::Number => {
                    InferredType::Number
                }
                LoweredUnaryOp::Not => InferredType::Boolean,
                _ => InferredType::Unknown,
            },
            Self::Binary { left, op, right } => match op {
                LoweredBinaryOp::Add => match (left.inferred_type(), right.inferred_type()) {
                    (InferredType::Number, InferredType::Number) => InferredType::Number,
                    (InferredType::String, InferredType::String) => InferredType::String,
                    _ => InferredType::Unknown,
                },
                LoweredBinaryOp::Subtract
                | LoweredBinaryOp::Multiply
                | LoweredBinaryOp::Divide
                | LoweredBinaryOp::Modulo => {
                    if left.inferred_type() == InferredType::Number
                        && right.inferred_type() == InferredType::Number
                    {
                        InferredType::Number
                    } else {
                        InferredType::Unknown
                    }
                }
                LoweredBinaryOp::Less
                | LoweredBinaryOp::LessEqual
                | LoweredBinaryOp::Greater
                | LoweredBinaryOp::GreaterEqual
                | LoweredBinaryOp::StrictEqual
                | LoweredBinaryOp::EqualEqual
                | LoweredBinaryOp::BangEqual
                | LoweredBinaryOp::StrictNotEqual => InferredType::Boolean,
                LoweredBinaryOp::And | LoweredBinaryOp::Or => InferredType::Unknown,
            },
            Self::Assign { expr, .. } => expr.inferred_type(),
            _ => InferredType::Unknown,
        }
    }
}

pub fn lower_program(program: &[ResolvedStmt]) -> Result<LoweredProgram, Diagnostic> {
    let function_ids = collect_function_ids(program)?;
    let class_parents = collect_class_parents(program);
    let mut resolver = Resolver::new(&function_ids, class_parents.clone());
    let mut top_level_statements = Vec::new();
    let mut functions = Vec::new();

    for stmt in program {
        match stmt {
            ResolvedStmt::Function { name, params, body } => {
                let func_id = function_ids[name];
                functions.push(lower_function(
                    func_id,
                    params,
                    body,
                    &function_ids,
                    class_parents.clone(),
                    None,
                    false,
                )?);
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                methods,
                ..
            } => {
                let ctor_key = class_constructor_key(name);
                let ctor_id = function_ids[&ctor_key];

                let (ctor_params, ctor_body) = if let Some((params, body)) = constructor {
                    (params.clone(), body.clone())
                } else {
                    (Vec::new(), Vec::new())
                };

                let mut ctor_params_with_this: Vec<(String, Option<ResolvedExpr>, bool)> =
                    vec![("this".to_owned(), None, false)];
                ctor_params_with_this.extend(ctor_params.clone());

                functions.push(lower_function(
                    ctor_id,
                    &ctor_params_with_this,
                    &ctor_body,
                    &function_ids,
                    class_parents.clone(),
                    Some(name),
                    true,
                )?);

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    let method_id = function_ids[&method_key];
                    let method_params_with_this: Vec<(String, Option<ResolvedExpr>, bool)> =
                        if method.name.starts_with("static::") {
                            method.params.clone()
                        } else {
                            let mut params = vec![("this".to_owned(), None, false)];
                            params.extend(method.params.clone());
                            params
                        };
                    functions.push(lower_function(
                        method_id,
                        &method_params_with_this,
                        &method.body,
                        &function_ids,
                        class_parents.clone(),
                        Some(name),
                        false,
                    )?);
                }
            }
            _ => top_level_statements.push(resolver.lower_stmt(stmt)?),
        }
    }

    Ok(LoweredProgram {
        top_level_statements,
        top_level_locals: resolver.locals,
        functions,
        modules: resolver.modules,
    })
}

fn collect_function_ids(program: &[ResolvedStmt]) -> Result<HashMap<String, FuncId>, Diagnostic> {
    let mut function_ids = HashMap::new();
    let mut next_func_id = 0;

    for stmt in program {
        match stmt {
            ResolvedStmt::Function { name, .. } => {
                if function_ids.contains_key(name.as_str()) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate function definition: `{name}`"),
                        span: None,
                    });
                }
                function_ids.insert(name.clone(), FuncId(next_func_id));
                next_func_id += 1;
            }
            ResolvedStmt::ClassDecl {
                name,
                constructor,
                methods,
                ..
            } => {
                let ctor_key = class_constructor_key(name);
                if function_ids.contains_key(&ctor_key) {
                    return Err(Diagnostic {
                        code: DiagCode::DuplicateFunction,
                        message: format!("duplicate constructor definition: `{name}`"),
                        span: None,
                    });
                }
                function_ids.insert(ctor_key, FuncId(next_func_id));
                next_func_id += 1;

                if constructor.is_some() {
                    // constructor body is lowered into the constructor function ID above.
                }

                for method in methods {
                    let method_key = class_method_key(name, &method.name);
                    if function_ids.contains_key(&method_key) {
                        return Err(Diagnostic {
                            code: DiagCode::DuplicateFunction,
                            message: format!(
                                "duplicate method definition: `{}.{}`",
                                name, method.name
                            ),
                            span: None,
                        });
                    }
                    function_ids.insert(method_key, FuncId(next_func_id));
                    next_func_id += 1;
                }
            }
            _ => {}
        }
    }

    Ok(function_ids)
}

fn class_constructor_key(class_name: &str) -> String {
    format!("class::{class_name}::constructor")
}

fn class_method_key(class_name: &str, method_name: &str) -> String {
    format!("class::{class_name}::{method_name}")
}

fn collect_class_parents(program: &[ResolvedStmt]) -> HashMap<String, Option<String>> {
    let mut parents = HashMap::new();
    for stmt in program {
        if let ResolvedStmt::ClassDecl { name, extends, .. } = stmt {
            parents.insert(name.clone(), extends.clone());
        }
    }
    parents
}

fn lower_function(
    id: FuncId,
    params: &[(String, Option<ResolvedExpr>, bool)],
    body: &[ResolvedStmt],
    function_ids: &HashMap<String, FuncId>,
    class_parents: HashMap<String, Option<String>>,
    current_class: Option<&str>,
    in_constructor: bool,
) -> Result<LoweredFunction, Diagnostic> {
    let (mut resolver, param_ids) = Resolver::with_params(
        function_ids,
        params
            .iter()
            .map(|(name, _, _)| name.clone())
            .collect::<Vec<_>>()
            .as_slice(),
        class_parents,
        current_class,
        in_constructor,
    )?;

    // Insert default parameter assignments and rest parameter collection at the start of the body
    let mut body_with_defaults = Vec::new();
    for (param_name, default_expr, is_rest) in params {
        if *is_rest {
            // Rest parameter: collect remaining arguments into an array
            let param_local = resolver.resolve_local(param_name)?;
            body_with_defaults.push(LoweredStmt::Assign(
                param_local,
                LoweredExpr::ArrayNew { elements: vec![] },
            ));
        } else if let Some(default) = default_expr {
            let param_local = resolver.resolve_local(param_name)?;
            let lowered_default = resolver.lower_expr(default)?;
            // Generate: if (param === undefined) { param = default; }
            body_with_defaults.push(LoweredStmt::If {
                condition: LoweredExpr::Binary {
                    left: Box::new(LoweredExpr::Local(param_local)),
                    op: LoweredBinaryOp::StrictEqual,
                    right: Box::new(LoweredExpr::Undefined),
                },
                then_body: vec![LoweredStmt::Assign(param_local, lowered_default)],
                else_body: vec![],
            });
        }
    }
    body_with_defaults.extend(resolver.lower_block(body)?);

    let min_required = params
        .iter()
        .filter(|(_, default, is_rest)| default.is_none() && !*is_rest)
        .count();
    Ok(LoweredFunction {
        id,
        params: param_ids,
        min_required_params: min_required,
        locals: resolver.locals,
        body: body_with_defaults,
    })
}

fn lower_binary_op(op: BinaryOp) -> Result<LoweredBinaryOp, Diagnostic> {
    match op {
        BinaryOp::Add => Ok(LoweredBinaryOp::Add),
        BinaryOp::Subtract => Ok(LoweredBinaryOp::Subtract),
        BinaryOp::Multiply => Ok(LoweredBinaryOp::Multiply),
        BinaryOp::Divide => Ok(LoweredBinaryOp::Divide),
        BinaryOp::Modulo => Ok(LoweredBinaryOp::Modulo),
        BinaryOp::Less => Ok(LoweredBinaryOp::Less),
        BinaryOp::LessEqual => Ok(LoweredBinaryOp::LessEqual),
        BinaryOp::Greater => Ok(LoweredBinaryOp::Greater),
        BinaryOp::GreaterEqual => Ok(LoweredBinaryOp::GreaterEqual),
        BinaryOp::StrictEqual => Ok(LoweredBinaryOp::StrictEqual),
        BinaryOp::EqualEqual => Ok(LoweredBinaryOp::EqualEqual),
        BinaryOp::BangEqual => Ok(LoweredBinaryOp::BangEqual),
        BinaryOp::StrictNotEqual => Ok(LoweredBinaryOp::StrictNotEqual),
        BinaryOp::And => Ok(LoweredBinaryOp::And),
        BinaryOp::Or => Ok(LoweredBinaryOp::Or),
        BinaryOp::Power
        | BinaryOp::BitwiseAnd
        | BinaryOp::BitwiseOr
        | BinaryOp::BitwiseXor
        | BinaryOp::LeftShift
        | BinaryOp::RightShift
        | BinaryOp::UnsignedRightShift
        | BinaryOp::In
        | BinaryOp::InstanceOf => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("binary operator {:?} not yet supported", op),
            span: None,
        }),
    }
}

fn resolve_method_to_runtime_fn(object: &ResolvedExpr, method: &str) -> Option<String> {
    if let ResolvedExpr::Ident(name) = object {
        if name == "Math" {
            return match method {
                "floor" => Some("MathFloor".to_owned()),
                "ceil" => Some("MathCeil".to_owned()),
                "round" => Some("MathRound".to_owned()),
                "abs" => Some("MathAbs".to_owned()),
                "max" => Some("MathMax".to_owned()),
                "min" => Some("MathMin".to_owned()),
                "random" => Some("MathRandom".to_owned()),
                _ => None,
            };
        }
        if name == "JSON" {
            return match method {
                "stringify" => Some("JsonStringify".to_owned()),
                "parse" => Some("JsonParse".to_owned()),
                _ => None,
            };
        }
        if name == "Object" {
            return match method {
                "keys" => Some("ObjectKeys".to_owned()),
                "values" => Some("ObjectValues".to_owned()),
                "entries" => Some("ObjectEntries".to_owned()),
                "getPrototypeOf" => Some("ObjectGetPrototypeOf".to_owned()),
                "setPrototypeOf" => Some("ObjectSetPrototypeOf".to_owned()),
                _ => None,
            };
        }
        if name == "String" {
            return match method {
                "fromCharCode" => Some("StringFromCharCode".to_owned()),
                _ => None,
            };
        }
    }
    match method {
        "charAt" => Some("StringCharAt".to_owned()),
        "substring" => Some("StringSubstring".to_owned()),
        "slice" => Some("StringSlice".to_owned()),
        "indexOf" => Some("StringIndexOf".to_owned()),
        "split" => Some("StringSplit".to_owned()),
        "trim" => Some("StringTrim".to_owned()),
        "toUpperCase" => Some("StringToUpperCase".to_owned()),
        "toLowerCase" => Some("StringToLowerCase".to_owned()),
        "charCodeAt" => Some("StringCharCodeAt".to_owned()),
        "push" => Some("ArrayPush".to_owned()),
        "pop" => Some("ArrayPop".to_owned()),
        "concat" => Some("ArrayConcat".to_owned()),
        "join" => Some("ArrayJoin".to_owned()),
        "reverse" => Some("ArrayReverse".to_owned()),
        _ => None,
    }
}

fn lower_unary_op(op: UnaryOp) -> Result<LoweredUnaryOp, Diagnostic> {
    match op {
        UnaryOp::Not => Ok(LoweredUnaryOp::Not),
        UnaryOp::Negate => Ok(LoweredUnaryOp::Negate),
        UnaryOp::TypeOf => Ok(LoweredUnaryOp::TypeOf),
        UnaryOp::Delete => Ok(LoweredUnaryOp::Delete),
        UnaryOp::Increment
        | UnaryOp::Decrement
        | UnaryOp::PreIncrement
        | UnaryOp::PreDecrement
        | UnaryOp::BitwiseNot
        | UnaryOp::Void => Err(Diagnostic {
            code: DiagCode::UnsupportedSyntax,
            message: format!("unary operator {:?} not yet supported", op),
            span: None,
        }),
    }
}

struct Resolver<'a> {
    function_ids: &'a HashMap<String, FuncId>,
    scopes: Vec<HashMap<String, LocalId>>,
    next_local_id: usize,
    locals: Vec<LocalId>,
    module_ids: HashMap<String, usize>,
    modules: Vec<ModuleInfo>,
    class_constructor_ids: HashMap<String, FuncId>,
    class_method_ids: HashMap<(String, String), FuncId>,
    class_static_method_ids: HashMap<(String, String), FuncId>,
    class_parents: HashMap<String, Option<String>>,
    local_classes: HashMap<LocalId, String>,
    current_class: Option<String>,
    in_constructor: bool,
}

impl<'a> Resolver<'a> {
    fn new(
        function_ids: &'a HashMap<String, FuncId>,
        class_parents: HashMap<String, Option<String>>,
    ) -> Self {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        Self {
            function_ids,
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
            module_ids: HashMap::new(),
            modules: Vec::new(),
            class_constructor_ids,
            class_method_ids,
            class_static_method_ids,
            class_parents,
            local_classes: HashMap::new(),
            current_class: None,
            in_constructor: false,
        }
    }

    fn with_params(
        function_ids: &'a HashMap<String, FuncId>,
        params: &[String],
        class_parents: HashMap<String, Option<String>>,
        current_class: Option<&str>,
        in_constructor: bool,
    ) -> Result<(Self, Vec<LocalId>), Diagnostic> {
        let (class_constructor_ids, class_method_ids, class_static_method_ids) =
            class_maps(function_ids);
        let mut resolver = Self {
            function_ids,
            scopes: vec![HashMap::new()],
            next_local_id: 0,
            locals: Vec::new(),
            module_ids: HashMap::new(),
            modules: Vec::new(),
            class_constructor_ids,
            class_method_ids,
            class_static_method_ids,
            class_parents,
            local_classes: HashMap::new(),
            current_class: current_class.map(ToOwned::to_owned),
            in_constructor,
        };
        let mut param_ids = Vec::new();
        let mut seen_params = HashMap::new();

        for param in params {
            if seen_params.contains_key(param) {
                return Err(Diagnostic {
                    code: DiagCode::DuplicateParameter,
                    message: format!("duplicate parameter name: `{param}`"),
                    span: None,
                });
            }
            seen_params.insert(param.clone(), ());
            let local_id = LocalId(resolver.next_local_id);
            resolver.next_local_id += 1;
            resolver
                .scopes
                .last_mut()
                .expect("function scope must exist")
                .insert(param.clone(), local_id);
            if let Some(current_class) = current_class {
                if param == "this" {
                    resolver
                        .local_classes
                        .insert(local_id, current_class.to_owned());
                }
            }
            param_ids.push(local_id);
        }

        Ok((resolver, param_ids))
    }

    fn lower_block(&mut self, statements: &[ResolvedStmt]) -> Result<Vec<LoweredStmt>, Diagnostic> {
        let mut lowered = Vec::with_capacity(statements.len());
        for statement in statements {
            lowered.push(self.lower_stmt(statement)?);
        }
        Ok(lowered)
    }

    fn lower_nested_block(
        &mut self,
        statements: &[ResolvedStmt],
    ) -> Result<Vec<LoweredStmt>, Diagnostic> {
        self.scopes.push(HashMap::new());
        let lowered = self.lower_block(statements);
        self.scopes.pop();
        lowered
    }

    fn lower_stmt(&mut self, stmt: &ResolvedStmt) -> Result<LoweredStmt, Diagnostic> {
        match stmt {
            ResolvedStmt::Let(name, expr) => {
                let local_id = self.declare_local(name)?;
                let lowered = self.lower_expr(expr)?;
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = expr_class {
                    self.local_classes.insert(local_id, class_name);
                } else {
                    self.local_classes.remove(&local_id);
                }
                Ok(LoweredStmt::Let(local_id, lowered))
            }
            ResolvedStmt::Assign(name, expr) => {
                let local_id = self.resolve_local(name)?;
                let lowered = self.lower_expr(expr)?;
                let expr_class = self.infer_class_for_expr(expr);
                if let Some(class_name) = expr_class {
                    self.local_classes.insert(local_id, class_name);
                } else {
                    self.local_classes.remove(&local_id);
                }
                Ok(LoweredStmt::Assign(local_id, lowered))
            }
            ResolvedStmt::Expr(expr) => Ok(LoweredStmt::Expr(self.lower_expr(expr)?)),
            ResolvedStmt::If {
                condition,
                then_body,
                else_body,
            } => Ok(LoweredStmt::If {
                condition: self.lower_expr(condition)?,
                then_body: self.lower_nested_block(then_body)?,
                else_body: self.lower_nested_block(else_body)?,
            }),
            ResolvedStmt::While { condition, body } => Ok(LoweredStmt::While {
                condition: self.lower_expr(condition)?,
                body: self.lower_nested_block(body)?,
            }),
            ResolvedStmt::Return(expr) => Ok(LoweredStmt::Return(self.lower_expr(expr)?)),
            ResolvedStmt::Function { .. } => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "nested function declarations are not supported in this milestone"
                    .to_owned(),
                span: None,
            }),
            ResolvedStmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
            } => {
                let catch_var = if let Some(param) = catch_param {
                    Some(self.declare_local(param)?)
                } else {
                    None
                };
                Ok(LoweredStmt::TryCatch {
                    try_body: self.lower_nested_block(try_block)?,
                    catch_var,
                    catch_body: catch_block
                        .as_ref()
                        .map(|b| self.lower_nested_block(b))
                        .transpose()?,
                    finally_body: finally_block
                        .as_ref()
                        .map(|b| self.lower_nested_block(b))
                        .transpose()?,
                })
            }
            ResolvedStmt::Throw(expr) => Ok(LoweredStmt::Throw(self.lower_expr(expr)?)),
            ResolvedStmt::Switch { expr, cases } => {
                let resolved_cases = cases
                    .iter()
                    .map(|(cond, body)| {
                        Ok((
                            cond.as_ref().map(|e| self.lower_expr(e)).transpose()?,
                            self.lower_nested_block(body)?,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredStmt::Switch {
                    expr: self.lower_expr(expr)?,
                    cases: resolved_cases,
                })
            }
            ResolvedStmt::DoWhile { body, condition } => Ok(LoweredStmt::DoWhile {
                body: self.lower_nested_block(body)?,
                condition: self.lower_expr(condition)?,
            }),
            ResolvedStmt::For {
                init,
                condition,
                update,
                body,
            } => {
                let resolved_init = if let Some(i) = init {
                    Some(Box::new(self.lower_stmt(i)?))
                } else {
                    None
                };
                Ok(LoweredStmt::For {
                    init: resolved_init,
                    condition: condition.as_ref().map(|c| self.lower_expr(c)).transpose()?,
                    update: update.as_ref().map(|u| self.lower_expr(u)).transpose()?,
                    body: self.lower_nested_block(body)?,
                })
            }
            ResolvedStmt::ForIn { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                Ok(LoweredStmt::ForIn {
                    var: var_id,
                    iter: self.lower_expr(iter)?,
                    iter_local: self.alloc_temp(),
                    index_local: self.alloc_temp(),
                    len_local: self.alloc_temp(),
                    body: self.lower_nested_block(body)?,
                })
            }
            ResolvedStmt::ForOf { var, iter, body } => {
                let var_id = self.declare_local(var)?;
                Ok(LoweredStmt::ForOf {
                    var: var_id,
                    iter: self.lower_expr(iter)?,
                    iter_local: self.alloc_temp(),
                    index_local: self.alloc_temp(),
                    len_local: self.alloc_temp(),
                    body: self.lower_nested_block(body)?,
                })
            }
            ResolvedStmt::Labeled { label, body } => Ok(LoweredStmt::Labeled {
                label: label.clone(),
                body: Box::new(self.lower_stmt(body)?),
            }),
            ResolvedStmt::Break { label } => Ok(LoweredStmt::Break {
                label: label.clone(),
            }),
            ResolvedStmt::Continue { label } => Ok(LoweredStmt::Continue {
                label: label.clone(),
            }),
            ResolvedStmt::Export { name, expr } => Ok(LoweredStmt::Export {
                name: name.clone(),
                expr: self.lower_expr(expr)?,
            }),
            ResolvedStmt::ModuleExportsAssign { expr } => Ok(LoweredStmt::ModuleExportsAssign {
                expr: self.lower_expr(expr)?,
            }),
            ResolvedStmt::ClassDecl { .. } => Ok(LoweredStmt::Expr(LoweredExpr::Undefined)),
        }
    }

    fn lower_expr(&mut self, expr: &ResolvedExpr) -> Result<LoweredExpr, Diagnostic> {
        match expr {
            ResolvedExpr::Number(value) => Ok(LoweredExpr::Number(*value)),
            ResolvedExpr::String(value) => Ok(LoweredExpr::String(value.clone())),
            ResolvedExpr::Bool(value) => Ok(LoweredExpr::Bool(*value)),
            ResolvedExpr::Null => Ok(LoweredExpr::Null),
            ResolvedExpr::Undefined => Ok(LoweredExpr::Undefined),
            ResolvedExpr::This => match self.resolve_local("this") {
                Ok(local) => Ok(LoweredExpr::Local(local)),
                Err(_) => Ok(LoweredExpr::This),
            },
            ResolvedExpr::Ident(name) => Ok(LoweredExpr::Local(self.resolve_local(name)?)),
            ResolvedExpr::Spread(_) => Err(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: "spread expressions are only supported in call arguments".to_owned(),
                span: None,
            }),
            ResolvedExpr::Unary { op, expr } => {
                if *op == UnaryOp::Delete {
                    // Lower delete to PropertyDelete or PropertyDeleteDynamic
                    match expr.as_ref() {
                        ResolvedExpr::PropertyAccess { object, key } => {
                            Ok(LoweredExpr::PropertyDelete {
                                object: Box::new(self.lower_expr(object)?),
                                key: key.clone(),
                            })
                        }
                        ResolvedExpr::ComputedIndex { object, index } => {
                            Ok(LoweredExpr::PropertyDeleteDynamic {
                                object: Box::new(self.lower_expr(object)?),
                                key: Box::new(self.lower_expr(index)?),
                            })
                        }
                        _ => Ok(LoweredExpr::Unary {
                            op: lower_unary_op(*op)?,
                            expr: Box::new(self.lower_expr(expr)?),
                        }),
                    }
                } else {
                    Ok(LoweredExpr::Unary {
                        op: lower_unary_op(*op)?,
                        expr: Box::new(self.lower_expr(expr)?),
                    })
                }
            }
            ResolvedExpr::Binary { left, op, right } => {
                if *op == BinaryOp::InstanceOf {
                    let prototype = match right.as_ref() {
                        ResolvedExpr::Ident(name) => self
                            .class_prototype_ref(name)
                            .map(LoweredExpr::ClassPrototype)?,
                        _ => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "issue-207: instanceof right-hand side must be a supported class constructor".to_owned(),
                                span: None,
                            });
                        }
                    };
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn: "$instanceof".to_string(),
                        args: vec![self.lower_expr(left)?, prototype],
                    })
                } else if *op == BinaryOp::In {
                    // Lower in to PropertyIn or PropertyInDynamic
                    // key in object -> check if key exists in object
                    match left.as_ref() {
                        ResolvedExpr::String(key) => Ok(LoweredExpr::PropertyIn {
                            obj: Box::new(self.lower_expr(right)?),
                            key: key.clone(),
                        }),
                        _ => Ok(LoweredExpr::PropertyInDynamic {
                            obj: Box::new(self.lower_expr(right)?),
                            key: Box::new(self.lower_expr(left)?),
                        }),
                    }
                } else {
                    Ok(LoweredExpr::Binary {
                        left: Box::new(self.lower_expr(left)?),
                        op: lower_binary_op(*op)?,
                        right: Box::new(self.lower_expr(right)?),
                    })
                }
            }
            ResolvedExpr::Assign { name, expr } => {
                let local = self.resolve_local(name)?;
                Ok(LoweredExpr::Assign {
                    local,
                    expr: Box::new(self.lower_expr(expr)?),
                })
            }
            ResolvedExpr::Call { callee, args } => {
                let func_name = match callee.as_ref() {
                    ResolvedExpr::Ident(name) => name,
                    _ => {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "only identifier calls are supported in expression context"
                                .to_owned(),
                            span: None,
                        });
                    }
                };

                if func_name == "super" {
                    if !self.in_constructor {
                        return Err(Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super(...) is only supported in constructors".to_owned(),
                            span: None,
                        });
                    }
                    let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                        code: DiagCode::UnsupportedSyntax,
                        message: "super(...) requires class context".to_owned(),
                        span: None,
                    })?;
                    let parent_name = self
                        .class_parents
                        .get(class_name)
                        .and_then(|p| p.clone())
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super(...) used in class without extends".to_owned(),
                            span: None,
                        })?;
                    let parent_ctor = self
                        .class_constructor_ids
                        .get(&parent_name)
                        .copied()
                        .ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: format!(
                                "super class constructor for `{}` not found",
                                parent_name
                            ),
                            span: None,
                        })?;

                    let mut lowered_args = vec![LoweredExpr::Local(self.resolve_local("this")?)];
                    lowered_args.extend(
                        args.iter()
                            .map(|arg| self.lower_expr(arg))
                            .collect::<Result<Vec<_>, _>>()?,
                    );

                    return Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(parent_ctor),
                        args: lowered_args,
                    });
                }

                // Lower arguments, expanding spread arrays
                let mut lowered_args = Vec::new();
                for arg in args {
                    match arg {
                        ResolvedExpr::Spread(spread_expr) => {
                            // For now, only support spreading literal arrays
                            if let ResolvedExpr::Array(elements) = spread_expr.as_ref() {
                                for elem in elements {
                                    lowered_args.push(self.lower_expr(elem)?);
                                }
                            } else {
                                return Err(Diagnostic {
                                    code: DiagCode::UnsupportedSyntax,
                                    message: "spread arguments are only supported for literal arrays in this milestone".to_owned(),
                                    span: None,
                                });
                            }
                        }
                        _ => {
                            lowered_args.push(self.lower_expr(arg)?);
                        }
                    }
                }
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::User(self.resolve_func(func_name)?),
                    args: lowered_args,
                })
            }
            ResolvedExpr::BuiltinCall { builtin, args } => {
                let lowered_args = args
                    .iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredExpr::Call {
                    kind: FunctionCallKind::Builtin(*builtin),
                    args: lowered_args,
                })
            }
            ResolvedExpr::BuiltinProperty { builtin, object } => match builtin {
                BuiltinPropertyId::Length => {
                    Ok(LoweredExpr::GetLength(Box::new(self.lower_expr(object)?)))
                }
            },
            ResolvedExpr::PropertyAccess { object, key } => Ok(LoweredExpr::PropertyGet {
                obj: Box::new(self.lower_expr(object)?),
                key: key.clone(),
            }),
            ResolvedExpr::ComputedIndex { object, index } => {
                // Lower the object first to determine its type
                let lowered_object = self.lower_expr(object)?;
                let lowered_index = self.lower_expr(index)?;

                // Keep obvious array literals on the compact array helper. Unknown
                // receivers must use the generic index helper so object[stringKey]
                // and array[numberIndex] both preserve JavaScript semantics.
                if matches!(object.as_ref(), ResolvedExpr::String(_)) {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                } else if matches!(object.as_ref(), ResolvedExpr::Array(_))
                    || matches!(lowered_object, LoweredExpr::ArrayNew { .. })
                {
                    Ok(LoweredExpr::ArrayGet {
                        arr: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                } else {
                    Ok(LoweredExpr::Index {
                        object: Box::new(lowered_object),
                        index: Box::new(lowered_index),
                    })
                }
            }
            ResolvedExpr::Array(elements) => {
                let lowered = elements
                    .iter()
                    .map(|e| self.lower_expr(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredExpr::ArrayNew { elements: lowered })
            }
            ResolvedExpr::Object(props) => {
                let lowered = props
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), self.lower_expr(v)?)))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(LoweredExpr::ObjectNew { props: lowered })
            }
            ResolvedExpr::MethodCall {
                object,
                method,
                args,
            } => {
                if let Some(runtime_fn) = resolve_method_to_runtime_fn(object, method) {
                    let mut lowered_args = Vec::new();
                    let is_static_call = matches!(
                        object.as_ref(),
                        ResolvedExpr::Ident(name) if name == "Math" || name == "JSON" || name == "Object" || name == "String"
                    );
                    if !is_static_call {
                        lowered_args.push(self.lower_expr(object)?);
                    }
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);
                    Ok(LoweredExpr::RuntimeCall {
                        runtime_fn,
                        args: lowered_args,
                    })
                } else {
                    if matches!(object.as_ref(), ResolvedExpr::This) {
                        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "this.method(...) requires class context".to_owned(),
                            span: None,
                        })?;
                        let method_id =
                            self.resolve_class_method(class_name, method)
                                .ok_or_else(|| Diagnostic {
                                    code: DiagCode::UnsupportedSyntax,
                                    message: format!(
                                        "method `{}.{}` not found",
                                        class_name, method
                                    ),
                                    span: None,
                                })?;

                        let mut lowered_args =
                            vec![LoweredExpr::Local(self.resolve_local("this")?)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    let receiver_name = match object.as_ref() {
                        ResolvedExpr::Ident(name) => name,
                        _ => {
                            return Err(Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "method `{}` requires an identifier receiver",
                                    method
                                ),
                                span: None,
                            });
                        }
                    };

                    if receiver_name == "super" {
                        let class_name = self.current_class.as_ref().ok_or_else(|| Diagnostic {
                            code: DiagCode::UnsupportedSyntax,
                            message: "super.method(...) requires class context".to_owned(),
                            span: None,
                        })?;
                        let parent_name = self
                            .class_parents
                            .get(class_name)
                            .and_then(|p| p.clone())
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: "super.method(...) used in class without extends"
                                    .to_owned(),
                                span: None,
                            })?;
                        let method_id = self
                            .resolve_class_method(&parent_name, method)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!(
                                    "super method `{}.{}` not found",
                                    parent_name, method
                                ),
                                span: None,
                            })?;

                        let mut lowered_args =
                            vec![LoweredExpr::Local(self.resolve_local("this")?)];
                        lowered_args.extend(
                            args.iter()
                                .map(|e| self.lower_expr(e))
                                .collect::<Result<Vec<_>, _>>()?,
                        );
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    if let Some(method_id) = self
                        .class_static_method_ids
                        .get(&(receiver_name.clone(), method.clone()))
                        .copied()
                    {
                        let lowered_args = args
                            .iter()
                            .map(|e| self.lower_expr(e))
                            .collect::<Result<Vec<_>, _>>()?;
                        return Ok(LoweredExpr::Call {
                            kind: FunctionCallKind::User(method_id),
                            args: lowered_args,
                        });
                    }

                    let obj_local = self.resolve_local(receiver_name)?;

                    let class_name =
                        self.local_classes
                            .get(&obj_local)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!("unknown receiver class for method `{}`", method),
                                span: None,
                            })?;

                    let method_id =
                        self.resolve_class_method(class_name, method)
                            .ok_or_else(|| Diagnostic {
                                code: DiagCode::UnsupportedSyntax,
                                message: format!("method `{}.{}` not found", class_name, method),
                                span: None,
                            })?;

                    let mut lowered_args = vec![LoweredExpr::Local(obj_local)];
                    lowered_args.extend(args.iter().map(|e| self.lower_expr(e)).collect::<Result<
                        Vec<_>,
                        _,
                    >>(
                    )?);

                    Ok(LoweredExpr::Call {
                        kind: FunctionCallKind::User(method_id),
                        args: lowered_args,
                    })
                }
            }
            ResolvedExpr::PropertyAssign { object, key, value } => Ok(LoweredExpr::PropertySet {
                object: Box::new(self.lower_expr(object)?),
                key: key.clone(),
                value: Box::new(self.lower_expr(value)?),
            }),
            ResolvedExpr::PropertyAssignDynamic { object, key, value } => {
                Ok(LoweredExpr::PropertySetDynamic {
                    object: Box::new(self.lower_expr(object)?),
                    index: Box::new(self.lower_expr(key)?),
                    value: Box::new(self.lower_expr(value)?),
                })
            }
            ResolvedExpr::New { class_name, args } => {
                let prototype = self.class_prototype_ref(class_name)?;

                let lowered_args = args
                    .iter()
                    .map(|arg| self.lower_expr(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(LoweredExpr::New {
                    constructor: prototype.constructor,
                    prototype,
                    args: lowered_args,
                    base_local: self.alloc_temp(),
                })
            }
            ResolvedExpr::ModuleLoad { specifier } => Ok(LoweredExpr::ModuleLoad {
                module_id: self.module_id_for_specifier(specifier),
            }),
            ResolvedExpr::ArrowFn { params: _, body: _ } => {
                // For now, lower arrow function to undefined as placeholder
                // Full implementation requires closure creation with lexical this capture
                Ok(LoweredExpr::Undefined)
            }
        }
    }

    fn declare_local(&mut self, name: &str) -> Result<LocalId, Diagnostic> {
        let scope = self.scopes.last_mut().expect("scope must exist");
        if scope.contains_key(name) {
            return Err(Diagnostic {
                code: DiagCode::DuplicateLocal,
                message: format!("duplicate local binding: `{name}`"),
                span: None,
            });
        }
        let local_id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(local_id);
        scope.insert(name.to_owned(), local_id);
        Ok(local_id)
    }

    fn alloc_temp(&mut self) -> LocalId {
        let id = LocalId(self.next_local_id);
        self.next_local_id += 1;
        self.locals.push(id);
        id
    }

    fn resolve_local(&self, name: &str) -> Result<LocalId, Diagnostic> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedName,
                message: format!("unresolved name: `{name}`"),
                span: None,
            })
    }

    fn resolve_func(&self, name: &str) -> Result<FuncId, Diagnostic> {
        self.function_ids
            .get(name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnresolvedFunction,
                message: format!("unresolved function: `{name}`"),
                span: None,
            })
    }

    fn module_id_for_specifier(&mut self, specifier: &str) -> usize {
        if let Some(id) = self.module_ids.get(specifier) {
            return *id;
        }

        let id = self.modules.len() + 1;
        self.module_ids.insert(specifier.to_owned(), id);
        self.modules.push(ModuleInfo {
            id,
            specifier: specifier.to_owned(),
            statements: Vec::new(),
            locals_count: 0,
        });
        id
    }

    fn resolve_class_method(&self, class_name: &str, method: &str) -> Option<FuncId> {
        let mut current = Some(class_name.to_owned());
        while let Some(class) = current {
            if let Some(id) = self
                .class_method_ids
                .get(&(class.clone(), method.to_owned()))
                .copied()
            {
                return Some(id);
            }
            current = self.class_parents.get(&class).and_then(|p| p.clone());
        }
        None
    }

    fn class_prototype_ref(&self, class_name: &str) -> Result<ClassPrototypeRef, Diagnostic> {
        let constructor = self
            .class_constructor_ids
            .get(class_name)
            .copied()
            .ok_or_else(|| Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message: format!(
                    "issue-207: instanceof right-hand side must be a supported class constructor `{}`",
                    class_name
                ),
                span: None,
            })?;

        let mut parent_constructors = Vec::new();
        let mut current = self.class_parents.get(class_name).and_then(|p| p.clone());
        while let Some(parent) = current {
            let parent_constructor = self
                .class_constructor_ids
                .get(&parent)
                .copied()
                .ok_or_else(|| Diagnostic {
                    code: DiagCode::UnsupportedSyntax,
                    message: format!(
                        "issue-207: superclass constructor `{}` is not available for instanceof",
                        parent
                    ),
                    span: None,
                })?;
            parent_constructors.push(parent_constructor);
            current = self.class_parents.get(&parent).and_then(|p| p.clone());
        }

        Ok(ClassPrototypeRef {
            constructor,
            parent_constructors,
        })
    }

    fn infer_class_for_expr(&self, expr: &ResolvedExpr) -> Option<String> {
        match expr {
            ResolvedExpr::New { class_name, .. } => Some(class_name.clone()),
            ResolvedExpr::Ident(name) => self
                .resolve_local(name)
                .ok()
                .and_then(|local_id| self.local_classes.get(&local_id).cloned()),
            _ => None,
        }
    }
}

fn class_maps(
    function_ids: &HashMap<String, FuncId>,
) -> (
    HashMap<String, FuncId>,
    HashMap<(String, String), FuncId>,
    HashMap<(String, String), FuncId>,
) {
    let mut ctor_ids = HashMap::new();
    let mut method_ids = HashMap::new();
    let mut static_method_ids = HashMap::new();

    for (name, id) in function_ids {
        if let Some(rest) = name.strip_prefix("class::") {
            let mut parts = rest.splitn(2, "::");
            let class = parts.next().unwrap_or_default();
            let member = parts.next().unwrap_or_default();
            if member == "constructor" {
                ctor_ids.insert(class.to_owned(), *id);
            } else if let Some(static_name) = member.strip_prefix("static::") {
                static_method_ids.insert((class.to_owned(), static_name.to_owned()), *id);
            } else if !class.is_empty() && !member.is_empty() {
                method_ids.insert((class.to_owned(), member.to_owned()), *id);
            }
        }
    }

    (ctor_ids, method_ids, static_method_ids)
}

pub fn validate_lowered(program: &LoweredProgram) -> Result<(), Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let num_funcs = program.functions.len();

    validate_functions(program, &mut errors);

    validate_stmts(
        &program.top_level_statements,
        program.top_level_locals.len(),
        num_funcs,
        program,
        &mut errors,
    );

    for func in &program.functions {
        let local_count = func.params.len() + func.locals.len();
        validate_stmts(&func.body, local_count, num_funcs, program, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_functions(program: &LoweredProgram, errors: &mut Vec<Diagnostic>) {
    for (idx, function) in program.functions.iter().enumerate() {
        if function.id.0 != idx {
            errors.push(Diagnostic {
                code: DiagCode::InvariantViolation,
                message: format!(
                    "function id {} does not match its index {}",
                    function.id.0, idx
                ),
                span: None,
            });
        }

        for (param_index, local_id) in function.params.iter().enumerate() {
            if local_id.0 != param_index {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "parameter LocalId {} must match parameter index {}",
                        local_id.0, param_index
                    ),
                    span: None,
                });
            }
        }

        let base = function.params.len();
        for (local_index, local_id) in function.locals.iter().enumerate() {
            let expected = base + local_index;
            if local_id.0 != expected {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: format!(
                        "local LocalId {} must be contiguous and start at {}",
                        local_id.0, base
                    ),
                    span: None,
                });
            }
        }
    }
}

fn validate_stmts(
    stmts: &[LoweredStmt],
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    for stmt in stmts {
        validate_stmt(stmt, local_count, num_funcs, program, errors);
    }
}

fn validate_stmt(
    stmt: &LoweredStmt,
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    match stmt {
        LoweredStmt::Let(id, expr) | LoweredStmt::Assign(id, expr) => {
            check_local_id(*id, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::Expr(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, false);
        }
        LoweredStmt::Return(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::Throw(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::If {
            condition,
            then_body,
            else_body,
        } => {
            validate_expr(condition, local_count, num_funcs, program, errors, true);
            validate_stmts(then_body, local_count, num_funcs, program, errors);
            validate_stmts(else_body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::While { condition, body } => {
            validate_expr(condition, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::TryCatch {
            try_body,
            catch_var,
            catch_body,
            finally_body,
        } => {
            validate_stmts(try_body, local_count, num_funcs, program, errors);
            if let Some(var_id) = catch_var {
                check_local_id(*var_id, local_count, errors);
            }
            if let Some(body) = catch_body {
                validate_stmts(body, local_count, num_funcs, program, errors);
            }
            if let Some(body) = finally_body {
                validate_stmts(body, local_count, num_funcs, program, errors);
            }
            if catch_body.is_none() && finally_body.is_none() {
                errors.push(Diagnostic {
                    code: DiagCode::InvariantViolation,
                    message: "try-catch must have at least a catch or finally block".to_owned(),
                    span: None,
                });
            }
        }
        LoweredStmt::Switch { expr, cases } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
            for (cond, body) in cases {
                if let Some(c) = cond {
                    validate_expr(c, local_count, num_funcs, program, errors, true);
                }
                validate_stmts(body, local_count, num_funcs, program, errors);
            }
        }
        LoweredStmt::DoWhile { body, condition } => {
            validate_stmts(body, local_count, num_funcs, program, errors);
            validate_expr(condition, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::For {
            init,
            condition,
            update,
            body,
        } => {
            if let Some(i) = init {
                validate_stmt(i, local_count, num_funcs, program, errors);
            }
            if let Some(c) = condition {
                validate_expr(c, local_count, num_funcs, program, errors, true);
            }
            if let Some(u) = update {
                validate_expr(u, local_count, num_funcs, program, errors, true);
            }
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::ForIn {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
        } => {
            check_local_id(*var, local_count, errors);
            check_local_id(*iter_local, local_count, errors);
            check_local_id(*index_local, local_count, errors);
            check_local_id(*len_local, local_count, errors);
            validate_expr(iter, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::ForOf {
            var,
            iter,
            iter_local,
            index_local,
            len_local,
            body,
        } => {
            check_local_id(*var, local_count, errors);
            check_local_id(*iter_local, local_count, errors);
            check_local_id(*index_local, local_count, errors);
            check_local_id(*len_local, local_count, errors);
            validate_expr(iter, local_count, num_funcs, program, errors, true);
            validate_stmts(body, local_count, num_funcs, program, errors);
        }
        LoweredStmt::Labeled { body, .. } => {
            validate_stmt(body, local_count, num_funcs, program, errors)
        }
        LoweredStmt::Break { .. } | LoweredStmt::Continue { .. } => {}
        LoweredStmt::Export { expr, .. } | LoweredStmt::ModuleExportsAssign { expr } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredStmt::ClassDecl { .. } => {}
    }
}

fn validate_expr(
    expr: &LoweredExpr,
    local_count: usize,
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
    value_required: bool,
) {
    match expr {
        LoweredExpr::Number(n) => {
            if !ValueTag::can_encode_number(*n) {
                errors.push(Diagnostic {
                    code: DiagCode::NumberOutOfRange,
                    message: format!(
                        "number literal {n} is out of small-int tagged range ({MIN}..={MAX})",
                        MIN = ValueTag::NUMBER_PAYLOAD_MIN,
                        MAX = ValueTag::NUMBER_PAYLOAD_MAX,
                    ),
                    span: None,
                });
            }
        }
        LoweredExpr::Local(id) => check_local_id(*id, local_count, errors),
        LoweredExpr::Unary { expr, .. } => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Assign { local, expr } => {
            check_local_id(*local, local_count, errors);
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Binary { left, right, .. } => {
            validate_expr(left, local_count, num_funcs, program, errors, true);
            validate_expr(right, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Call { kind, args } => {
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
            match kind {
                FunctionCallKind::User(func_id) => {
                    if func_id.0 >= num_funcs {
                        errors.push(Diagnostic {
                            code: DiagCode::InvariantViolation,
                            message: format!(
                                "FuncId {} is out of range (program has {} function(s))",
                                func_id.0, num_funcs
                            ),
                            span: None,
                        });
                    } else {
                        let func = &program.functions[func_id.0];
                        let min_required = func.min_required_params;
                        let max_allowed = func.params.len();
                        if args.len() < min_required || args.len() > max_allowed {
                            errors.push(Diagnostic {
                                code: DiagCode::ArityMismatch,
                                message: format!(
                                    "function {} expects between {} and {} argument(s), got {}",
                                    func_id.0,
                                    min_required,
                                    max_allowed,
                                    args.len()
                                ),
                                span: None,
                            });
                        }
                    }
                }
                FunctionCallKind::Builtin(builtin) => {
                    let expected = builtin.expected_arity();
                    if args.len() != expected {
                        errors.push(Diagnostic {
                            code: DiagCode::ArityMismatch,
                            message: format!(
                                "builtin {:?} expects {} argument(s), got {}",
                                builtin,
                                expected,
                                args.len()
                            ),
                            span: None,
                        });
                    }
                    if value_required && matches!(builtin.result(), BuiltinResult::EffectOnly) {
                        errors.push(Diagnostic {
                            code: DiagCode::InvariantViolation,
                            message: format!(
                                "builtin {:?} is effect-only and cannot be used in a value context",
                                builtin
                            ),
                            span: None,
                        });
                    }
                }
            }
        }
        LoweredExpr::ArrayNew { elements } => {
            for elem in elements {
                validate_expr(elem, local_count, num_funcs, program, errors, true);
            }
        }
        LoweredExpr::ArrayGet { arr, index } => {
            validate_expr(arr, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::Index { object, index } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::GetLength(expr) => {
            validate_expr(expr, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::ObjectNew { props } => {
            for (_, val) in props {
                validate_expr(val, local_count, num_funcs, program, errors, true);
            }
        }
        LoweredExpr::PropertyGet { obj, .. } => {
            validate_expr(obj, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertySet { object, value, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(value, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::PropertySetDynamic {
            object,
            index,
            value,
        } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            validate_expr(index, local_count, num_funcs, program, errors, true);
            validate_expr(value, local_count, num_funcs, program, errors, true);
        }
        LoweredExpr::New {
            constructor,
            prototype,
            args,
            base_local,
        } => {
            check_func_id(*constructor, num_funcs, errors);
            check_func_id(prototype.constructor, num_funcs, errors);
            for parent in &prototype.parent_constructors {
                check_func_id(*parent, num_funcs, errors);
            }
            check_local_id(*base_local, local_count, errors);
            validate_constructor_arity(*constructor, args, num_funcs, program, errors);
            for arg in args {
                validate_expr(arg, local_count, num_funcs, program, errors, true);
            }
        }
        LoweredExpr::ClassPrototype(prototype) => {
            check_func_id(prototype.constructor, num_funcs, errors);
            for parent in &prototype.parent_constructors {
                check_func_id(*parent, num_funcs, errors);
            }
        }
        LoweredExpr::MethodCall { object, .. } => {
            validate_expr(object, local_count, num_funcs, program, errors, true);
            errors.push(Diagnostic {
                code: DiagCode::UnsupportedSyntax,
                message:
                    "MethodCall must be resolved before backend; residual MethodCall is unsupported"
                        .to_owned(),
                span: None,
            });
        }
        _ => {}
    }
}

fn check_local_id(id: LocalId, local_count: usize, errors: &mut Vec<Diagnostic>) {
    if id.0 >= local_count {
        errors.push(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "LocalId {} is out of range (scope has {} local(s))",
                id.0, local_count
            ),
            span: None,
        });
    }
}

fn check_func_id(id: FuncId, num_funcs: usize, errors: &mut Vec<Diagnostic>) {
    if id.0 >= num_funcs {
        errors.push(Diagnostic {
            code: DiagCode::InvariantViolation,
            message: format!(
                "FuncId {} is out of range (program has {} function(s))",
                id.0, num_funcs
            ),
            span: None,
        });
    }
}

fn validate_constructor_arity(
    constructor: FuncId,
    args: &[LoweredExpr],
    num_funcs: usize,
    program: &LoweredProgram,
    errors: &mut Vec<Diagnostic>,
) {
    if constructor.0 >= num_funcs {
        return;
    }
    let func = &program.functions[constructor.0];
    let min_required = func.min_required_params.saturating_sub(1);
    let max_allowed = func.params.len().saturating_sub(1);
    if args.len() < min_required || args.len() > max_allowed {
        errors.push(Diagnostic {
            code: DiagCode::ArityMismatch,
            message: format!(
                "constructor {} expects between {} and {} argument(s), got {}",
                constructor.0,
                min_required,
                max_allowed,
                args.len()
            ),
            span: None,
        });
    }
}

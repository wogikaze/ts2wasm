use crate::Span;

pub const OBJECT_SPREAD_SENTINEL: &str = "\0ts2wasm_object_spread";
pub const SYMBOL_ITERATOR_OBJECT_KEY: &str = "\0ts2wasm_symbol_iterator";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
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
    NullishCoalesce,
    Multiply,
    Divide,
    Modulo,
    Power,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    In,
    InstanceOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Plus,
    Negate,
    Increment,
    Decrement,
    PreIncrement,
    PreDecrement,
    TypeOf,
    BitwiseNot,
    Delete,
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalAssignOp {
    And,
    Or,
    Nullish,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSpecifier {
    pub value: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportNamedSpecifier {
    pub imported: String,
    pub imported_span: Span,
    pub local: String,
    pub local_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDefaultSpecifier {
    pub local: String,
    pub local_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportNamespaceSpecifier {
    pub local: String,
    pub local_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportNamedSpecifier {
    pub local: String,
    pub local_span: Span,
    pub exported: String,
    pub exported_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReExportNamedSpecifier {
    pub imported: String,
    pub imported_span: Span,
    pub exported: String,
    pub exported_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReExportNamespaceSpecifier {
    pub exported: String,
    pub exported_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassStaticBlock {
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassPrivateElement {
    Field {
        name: String,
        name_span: Span,
        value: Option<Expr>,
        is_static: bool,
        span: Span,
    },
    Method {
        name: String,
        name_span: Span,
        params: Vec<(String, Option<Expr>, bool)>,
        body: Vec<Stmt>,
        is_static: bool,
        span: Span,
    },
    Getter {
        name: String,
        name_span: Span,
        body: Vec<Stmt>,
        is_static: bool,
        span: Span,
    },
    Setter {
        name: String,
        name_span: Span,
        param: String,
        body: Vec<Stmt>,
        is_static: bool,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    ImportSideEffect {
        specifier: ModuleSpecifier,
        span: Span,
    },
    ImportNamed {
        specifiers: Vec<ImportNamedSpecifier>,
        source: ModuleSpecifier,
        span: Span,
    },
    ImportDefault {
        specifier: ImportDefaultSpecifier,
        source: ModuleSpecifier,
        span: Span,
    },
    ImportDefaultNamed {
        default: ImportDefaultSpecifier,
        specifiers: Vec<ImportNamedSpecifier>,
        source: ModuleSpecifier,
        span: Span,
    },
    ImportNamespace {
        specifier: ImportNamespaceSpecifier,
        source: ModuleSpecifier,
        span: Span,
    },
    ImportDefaultNamespace {
        default: ImportDefaultSpecifier,
        namespace: ImportNamespaceSpecifier,
        source: ModuleSpecifier,
        span: Span,
    },
    ExportNamed {
        specifiers: Vec<ExportNamedSpecifier>,
        span: Span,
    },
    ExportNamedFrom {
        specifiers: Vec<ReExportNamedSpecifier>,
        source: ModuleSpecifier,
        span: Span,
    },
    ExportAllFrom {
        star_span: Span,
        source: ModuleSpecifier,
        span: Span,
    },
    ExportNamespaceFrom {
        namespace: ReExportNamespaceSpecifier,
        source: ModuleSpecifier,
        span: Span,
    },
    ExportDecl {
        declaration: Box<Stmt>,
        specifier: ExportNamedSpecifier,
        span: Span,
    },
    ExportDefault {
        expr: Expr,
        default_span: Span,
        span: Span,
    },
    Let {
        name: String,
        expr: Expr,
        span: Span,
        is_var: bool,
    },
    AmbientValueDecl {
        name: String,
        span: Span,
        is_var: bool,
    },
    Assign {
        name: String,
        expr: Expr,
        span: Span,
    },
    Expr {
        expr: Expr,
        span: Span,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    Function {
        name: String,
        params: Vec<(String, Option<Expr>, bool)>,
        body: Vec<Stmt>,
        is_generator: bool,
        is_ambient: bool,
        span: Span,
    },
    Return {
        expr: Expr,
        span: Span,
    },
    ClassDecl {
        name: String,
        extends: Option<Box<Expr>>,
        body: Vec<Stmt>,
        static_blocks: Vec<ClassStaticBlock>,
        private_elements: Vec<ClassPrivateElement>,
        ts_private_field_names: Vec<String>,
        interface_heritage: Vec<Expr>,
        span: Span,
    },
    TryCatch {
        try_block: Vec<Stmt>,
        catch_param: Option<String>,
        catch_block: Option<Vec<Stmt>>,
        finally_block: Option<Vec<Stmt>>,
        span: Span,
    },
    Throw {
        expr: Expr,
        span: Span,
    },
    Switch {
        expr: Expr,
        cases: Vec<(Option<Expr>, Vec<Stmt>)>,
        span: Span,
    },
    DoWhile {
        body: Vec<Stmt>,
        condition: Expr,
        span: Span,
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Expr>,
        body: Vec<Stmt>,
        span: Span,
    },
    ForIn {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    ForOf {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    Labeled {
        label: String,
        body: Box<Stmt>,
        span: Span,
    },
    Break {
        label: Option<String>,
        span: Span,
    },
    Continue {
        label: Option<String>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayLiteralElement {
    Present(Expr),
    Hole(Span),
    Spread(Expr),
}

impl ArrayLiteralElement {
    pub fn span(&self) -> Span {
        match self {
            Self::Present(expr) | Self::Spread(expr) => expr.span(),
            Self::Hole(span) => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number {
        value: i32,
        span: Span,
    },
    BigInt {
        raw: String,
        span: Span,
    },
    String {
        value: String,
        span: Span,
    },
    Bool {
        value: bool,
        span: Span,
    },
    Null {
        span: Span,
    },
    Undefined {
        span: Span,
    },
    Await {
        expr: Box<Expr>,
        span: Span,
    },
    Ident {
        name: String,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    Member {
        object: Box<Expr>,
        property: String,
        span: Span,
    },
    OptionalMember {
        object: Box<Expr>,
        property: String,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    OptionalCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
        span: Span,
    },
    LogicalAssign {
        name: String,
        op: LogicalAssignOp,
        expr: Box<Expr>,
        span: Span,
    },
    LogicalPropertyAssign {
        object: String,
        object_expr: Option<Box<Expr>>,
        property: String,
        computed_key: Option<Box<Expr>>,
        op: LogicalAssignOp,
        expr: Box<Expr>,
        span: Span,
    },
    Array {
        elements: Vec<ArrayLiteralElement>,
        span: Span,
    },
    Object {
        props: Vec<(String, Expr)>,
        span: Span,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    OptionalIndex {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    New {
        expr: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    TypeOf {
        expr: Box<Expr>,
        span: Span,
    },
    InstanceOf {
        expr: Box<Expr>,
        type_expr: Box<Expr>,
        span: Span,
    },
    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        span: Span,
    },
    ArrowFn {
        params: Vec<String>,
        body: Box<Expr>,
        body_stmts: Vec<Stmt>,
        span: Span,
    },
    FunctionExpr {
        name: String,
        params: Vec<(String, Option<Expr>, bool)>,
        body: Vec<Stmt>,
        span: Span,
    },
    Spread {
        expr: Box<Expr>,
        span: Span,
    },
    PropertyAssign {
        object: Box<Expr>,
        property: String,
        value: Box<Expr>,
        span: Span,
    },
    IndexAssign {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
        span: Span,
    },
    ClassExpr {
        name: String,
        extends: Option<Box<Expr>>,
        body: Vec<Stmt>,
        static_blocks: Vec<ClassStaticBlock>,
        private_elements: Vec<ClassPrivateElement>,
        span: Span,
    },
    This {
        span: Span,
    },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Self::ImportSideEffect { span, .. }
            | Self::ImportNamed { span, .. }
            | Self::ImportDefault { span, .. }
            | Self::ImportDefaultNamed { span, .. }
            | Self::ImportNamespace { span, .. }
            | Self::ImportDefaultNamespace { span, .. }
            | Self::ExportNamed { span, .. }
            | Self::ExportNamedFrom { span, .. }
            | Self::ExportAllFrom { span, .. }
            | Self::ExportNamespaceFrom { span, .. }
            | Self::ExportDecl { span, .. }
            | Self::ExportDefault { span, .. }
            | Self::Let { span, .. }
            | Self::AmbientValueDecl { span, .. }
            | Self::Assign { span, .. }
            | Self::Expr { span, .. }
            | Self::If { span, .. }
            | Self::While { span, .. }
            | Self::Function { span, .. }
            | Self::Return { span, .. }
            | Self::ClassDecl { span, .. }
            | Self::TryCatch { span, .. }
            | Self::Throw { span, .. }
            | Self::Switch { span, .. }
            | Self::DoWhile { span, .. }
            | Self::For { span, .. }
            | Self::ForIn { span, .. }
            | Self::ForOf { span, .. }
            | Self::Labeled { span, .. }
            | Self::Break { span, .. }
            | Self::Continue { span, .. } => *span,
        }
    }
}

impl Expr {
    pub fn is_direct_eval_call(&self) -> bool {
        matches!(
            self,
            Self::Call { callee, .. }
                if matches!(callee.as_ref(), Self::Ident { name, .. } if name == "eval")
        )
    }

    pub fn direct_eval_literal_source(&self) -> Option<&str> {
        let Self::Call { args, .. } = self else {
            return None;
        };
        if !self.is_direct_eval_call() {
            return None;
        }
        let [Self::String { value, .. }] = args.as_slice() else {
            return None;
        };
        Some(value)
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Number { span, .. }
            | Self::BigInt { span, .. }
            | Self::String { span, .. }
            | Self::Bool { span, .. }
            | Self::Null { span }
            | Self::Undefined { span }
            | Self::Await { span, .. }
            | Self::Ident { span, .. }
            | Self::Unary { span, .. }
            | Self::Binary { span, .. }
            | Self::Member { span, .. }
            | Self::OptionalMember { span, .. }
            | Self::Call { span, .. }
            | Self::OptionalCall { span, .. }
            | Self::Assign { span, .. }
            | Self::LogicalAssign { span, .. }
            | Self::LogicalPropertyAssign { span, .. }
            | Self::Array { span, .. }
            | Self::Object { span, .. }
            | Self::Index { span, .. }
            | Self::OptionalIndex { span, .. }
            | Self::New { span, .. }
            | Self::TypeOf { span, .. }
            | Self::InstanceOf { span, .. }
            | Self::Ternary { span, .. }
            | Self::ArrowFn { span, .. }
            | Self::FunctionExpr { span, .. }
            | Self::Spread { span, .. }
            | Self::PropertyAssign { span, .. }
            | Self::IndexAssign { span, .. }
            | Self::ClassExpr { span, .. }
            | Self::This { span } => *span,
        }
    }
}

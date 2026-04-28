use crate::Span;

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
pub struct ExportNamedSpecifier {
    pub local: String,
    pub local_span: Span,
    pub exported: String,
    pub exported_span: Span,
    pub span: Span,
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
    ExportNamed {
        specifiers: Vec<ExportNamedSpecifier>,
        span: Span,
    },
    Let {
        name: String,
        expr: Expr,
        span: Span,
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
pub enum Expr {
    Number {
        value: i32,
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
    Call {
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
        property: String,
        op: LogicalAssignOp,
        expr: Box<Expr>,
        span: Span,
    },
    Array {
        elements: Vec<Expr>,
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
            | Self::ExportNamed { span, .. }
            | Self::Let { span, .. }
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
    pub fn span(&self) -> Span {
        match self {
            Self::Number { span, .. }
            | Self::String { span, .. }
            | Self::Bool { span, .. }
            | Self::Null { span }
            | Self::Undefined { span }
            | Self::Ident { span, .. }
            | Self::Unary { span, .. }
            | Self::Binary { span, .. }
            | Self::Member { span, .. }
            | Self::Call { span, .. }
            | Self::Assign { span, .. }
            | Self::LogicalAssign { span, .. }
            | Self::LogicalPropertyAssign { span, .. }
            | Self::Array { span, .. }
            | Self::Object { span, .. }
            | Self::Index { span, .. }
            | Self::New { span, .. }
            | Self::TypeOf { span, .. }
            | Self::InstanceOf { span, .. }
            | Self::Ternary { span, .. }
            | Self::ArrowFn { span, .. }
            | Self::Spread { span, .. }
            | Self::PropertyAssign { span, .. }
            | Self::IndexAssign { span, .. }
            | Self::This { span } => *span,
        }
    }
}

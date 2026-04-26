use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Number(i32),
    String(String),
    TemplateLiteral(String),
    RegExp(String),
    True,
    False,
    Null,
    Undefined,
    Let,
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    // New keywords for OOP and control flow
    This,
    Class,
    Try,
    Catch,
    Throw,
    Finally,
    Extends,
    Super,
    Static,
    Async,
    Await,
    Import,
    Export,
    Default,
    Case,
    Do,
    For,
    In,
    Of,
    New,
    TypeOf,
    InstanceOf,
    Void,
    Delete,
    Switch,
    Break,
    Continue,
    // Operators
    Plus,
    Minus,
    Less,
    Bang,
    StrictEqual,
    Equal,
    EqualEqual,
    BangEqual,
    StrictNotEqual,
    AndAnd,
    OrOr,
    Greater,
    Power,
    Increment,
    Decrement,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    PowerEqual,
    Percent,
    Slash,
    Star,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Question,
    Spread,
    DotDotDot,
    Arrow,
    OptionalChain,
    NullishCoalesce,
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Dot,
    Semicolon,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken {
    pub kind: Token,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Let,
    Const,
    Var,
    Function,
    Return,
    If,
    Else,
    While,
    Class,
    Try,
    Catch,
    Throw,
    Finally,
    Extends,
    Super,
    Static,
    Async,
    Await,
    Import,
    Export,
    Default,
    Case,
    Do,
    For,
    In,
    Of,
    New,
    TypeOf,
    InstanceOf,
    Void,
    Delete,
    Switch,
    Break,
    Continue,
    Plus,
    Minus,
    Less,
    Bang,
    StrictEqual,
    Equal,
    EqualEqual,
    BangEqual,
    StrictNotEqual,
    AndAnd,
    OrOr,
    Greater,
    Power,
    Increment,
    Decrement,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    PowerEqual,
    Percent,
    Slash,
    Star,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    Question,
    Spread,
    DotDotDot,
    Arrow,
    OptionalChain,
    NullishCoalesce,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Dot,
    Semicolon,
    TemplateLiteral,
    RegExp,
}

impl TokenKind {
    pub fn matches(self, token: &Token) -> bool {
        matches!(
            (self, token),
            (Self::Let, Token::Let)
                | (Self::Const, Token::Const)
                | (Self::Var, Token::Var)
                | (Self::Function, Token::Function)
                | (Self::Return, Token::Return)
                | (Self::If, Token::If)
                | (Self::Else, Token::Else)
                | (Self::While, Token::While)
                | (Self::Class, Token::Class)
                | (Self::Try, Token::Try)
                | (Self::Catch, Token::Catch)
                | (Self::Throw, Token::Throw)
                | (Self::Finally, Token::Finally)
                | (Self::Extends, Token::Extends)
                | (Self::Super, Token::Super)
                | (Self::Static, Token::Static)
                | (Self::Async, Token::Async)
                | (Self::Await, Token::Await)
                | (Self::Import, Token::Import)
                | (Self::Export, Token::Export)
                | (Self::Default, Token::Default)
                | (Self::Case, Token::Case)
                | (Self::Do, Token::Do)
                | (Self::For, Token::For)
                | (Self::In, Token::In)
                | (Self::Of, Token::Of)
                | (Self::New, Token::New)
                | (Self::TypeOf, Token::TypeOf)
                | (Self::InstanceOf, Token::InstanceOf)
                | (Self::Void, Token::Void)
                | (Self::Delete, Token::Delete)
                | (Self::Switch, Token::Switch)
                | (Self::Break, Token::Break)
                | (Self::Continue, Token::Continue)
                | (Self::Plus, Token::Plus)
                | (Self::Minus, Token::Minus)
                | (Self::Less, Token::Less)
                | (Self::Bang, Token::Bang)
                | (Self::StrictEqual, Token::StrictEqual)
                | (Self::Equal, Token::Equal)
                | (Self::EqualEqual, Token::EqualEqual)
                | (Self::BangEqual, Token::BangEqual)
                | (Self::StrictNotEqual, Token::StrictNotEqual)
                | (Self::AndAnd, Token::AndAnd)
                | (Self::OrOr, Token::OrOr)
                | (Self::Greater, Token::Greater)
                | (Self::Power, Token::Power)
                | (Self::Increment, Token::Increment)
                | (Self::Decrement, Token::Decrement)
                | (Self::PlusEqual, Token::PlusEqual)
                | (Self::MinusEqual, Token::MinusEqual)
                | (Self::StarEqual, Token::StarEqual)
                | (Self::SlashEqual, Token::SlashEqual)
                | (Self::PercentEqual, Token::PercentEqual)
                | (Self::PowerEqual, Token::PowerEqual)
                | (Self::Percent, Token::Percent)
                | (Self::Slash, Token::Slash)
                | (Self::Star, Token::Star)
                | (Self::Ampersand, Token::Ampersand)
                | (Self::Pipe, Token::Pipe)
                | (Self::Caret, Token::Caret)
                | (Self::Tilde, Token::Tilde)
                | (Self::LeftShift, Token::LeftShift)
                | (Self::RightShift, Token::RightShift)
                | (Self::UnsignedRightShift, Token::UnsignedRightShift)
                | (Self::Question, Token::Question)
                | (Self::Spread, Token::Spread)
                | (Self::DotDotDot, Token::DotDotDot)
                | (Self::Arrow, Token::Arrow)
                | (Self::OptionalChain, Token::OptionalChain)
                | (Self::NullishCoalesce, Token::NullishCoalesce)
                | (Self::LeftParen, Token::LeftParen)
                | (Self::RightParen, Token::RightParen)
                | (Self::LeftBrace, Token::LeftBrace)
                | (Self::RightBrace, Token::RightBrace)
                | (Self::LeftBracket, Token::LeftBracket)
                | (Self::RightBracket, Token::RightBracket)
                | (Self::Comma, Token::Comma)
                | (Self::Colon, Token::Colon)
                | (Self::Dot, Token::Dot)
                | (Self::Semicolon, Token::Semicolon)
                | (Self::TemplateLiteral, Token::TemplateLiteral(_))
                | (Self::RegExp, Token::RegExp(_))
        )
    }
}

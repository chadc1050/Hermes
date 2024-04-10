#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Comment(Comment<'a>),
    /// End of file
    Eof,
    /// Alphabetic tokens that are not string literals, boolean literals, or keywords.
    Identifier(String),
    /// Language keywords
    Keyword(Keyword),
    LineTerminator(LineTerminator),
    /// Value literals
    Literal(Literal<'a>),
    Punc(Punc),
    WhiteSpace(WhiteSpace),
    /// Any unknown characters that we are unable to identify
    Unicode(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WhiteSpace {
    ///
    Space,
    /// \t
    HorizontalTabulation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineTerminator {
    /// \n
    LineFeed,
    /// \r
    CarridgeReturn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Boolean {
    /// true
    True,
    /// false
    False,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comment<'a> {
    SingleLine(&'a str),
    MultiLine(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    BigIntSuffix(&'a str),
    Boolean(Boolean),
    Decimal(&'a str),
    DecimalBigInteger(&'a str),
    DecimalInteger(&'a str),
    NonDecimalInteger(NonDecimalIntegerLiteral<'a>),
    Null,
    Numeric(i64),
    StringLiteral(String),
    RegEx(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NonDecimalIntegerLiteral<'a> {
    BigInteger(&'a str),
    OctalInteger(&'a str),
    HexInteger(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    Extends,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    InstanceOf,
    Let,
    New,
    Return,
    Static,
    Super,
    Switch,
    This,
    Throw,
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,
    Yield,
}

pub fn map_keyword(keyword_str: &str) -> Option<Keyword> {
    match keyword_str {
        "await" => Some(Keyword::Await),
        "break" => Some(Keyword::Break),
        "case" => Some(Keyword::Case),
        "catch" => Some(Keyword::Catch),
        "class" => Some(Keyword::Class),
        "const" => Some(Keyword::Const),
        "continue" => Some(Keyword::Continue),
        "debugger" => Some(Keyword::Debugger),
        "default" => Some(Keyword::Default),
        "delete" => Some(Keyword::Delete),
        "do" => Some(Keyword::Do),
        "else" => Some(Keyword::Else),
        "export" => Some(Keyword::Export),
        "extends" => Some(Keyword::Extends),
        "finally" => Some(Keyword::Finally),
        "for" => Some(Keyword::For),
        "function" => Some(Keyword::Function),
        "if" => Some(Keyword::If),
        "import" => Some(Keyword::Import),
        "in" => Some(Keyword::In),
        "instanceof" => Some(Keyword::InstanceOf),
        "let" => Some(Keyword::Let),
        "new" => Some(Keyword::New),
        "return" => Some(Keyword::Return),
        "static" => Some(Keyword::Static),
        "super" => Some(Keyword::Super),
        "switch" => Some(Keyword::Switch),
        "this" => Some(Keyword::This),
        "throw" => Some(Keyword::Throw),
        "try" => Some(Keyword::Try),
        "typeof" => Some(Keyword::TypeOf),
        "var" => Some(Keyword::Var),
        "void" => Some(Keyword::Void),
        "while" => Some(Keyword::While),
        "with" => Some(Keyword::With),
        "yield" => Some(Keyword::Yield),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punc {
    Brace(Brace),
    Bracket(Bracket),
    /// .
    Dot,
    Op(Op),
    Parentheses(Parentheses),
    /// ;
    SemiColon,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parentheses {
    /// (
    Left,
    /// )
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    /// [
    Left,
    /// ]
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Brace {
    /// {
    Left,
    /// }
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    /// +
    Addition,
    /// +
    Increment,
    /// +=
    AdditonAssign,
    /// =
    Assign,
    /// -
    Subtraction,
    /// --
    Decrement,
    /// -=
    SubtractionAssign,
    /// *
    Multiplication,
    /// *=
    MultiplicationAssign,
    /// /
    Division,
    /// /=
    DivisionAssign,
    /// **
    Exponential,
    /// **=
    ExponentialAssign,
    /// %
    Mod,
    /// %=
    ModAssign,
    /// <<
    LeftShift,
    /// <<=
    LeftShiftAssign,
    /// >>
    RightShift,
    /// >>=
    RightShiftAssign,
    /// >>>
    ZeroFillRightShift,
    /// >>>=
    UnsignedRightShiftAssign,
    /// >
    GreaterThan,
    /// <
    LessThan,
    /// ==
    Equal,
    /// ===
    StrictEquality,
    /// !
    Not,
    /// !=
    NotEqual,
    /// >=
    GreaterThanEqual,
    /// <=
    LessThanEqual,
    /// &
    BitAnd,
    /// &=
    BitAndAssing,
    /// &&
    And,
    /// &&=
    AndAssign,
    /// |
    BitOr,
    /// |=
    BitOrAssign,
    /// ||
    Or,
    /// ||=
    OrAssign,
    /// ^
    BitXor,
    /// ^=
    BitXorAssign,
    /// ?.
    OptionalChain,
    /// ??
    NullishCoalescing,
    /// ??=
    NullishCoalescingAssign,
    /// ...
    Spread,
}

pub fn is_whitespace(token: &Token) -> bool {
    match token {
        Token::LineTerminator(_) => true,
        Token::WhiteSpace(_) => true,
        _ => false,
    }
}

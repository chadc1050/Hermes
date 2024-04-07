#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    Comment(Comment<'a>),
    Eof,
    Keyword(Keyword),
    LineTerminator(LineTerminator),
    Literal(Literal<'a>),
    Punc(Punc),
    WhiteSpace(WhiteSpace),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WhiteSpace {
    //
    Space,
    // \t
    HorizontalTabulation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineTerminator {
    // \n
    LineFeed,
    // \r
    CarridgeReturn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Boolean {
    // true
    True,
    // false
    False,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comment<'a> {
    SingleLine(&'a str),
    MultiLine(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'a> {
    BigIntSuffix(&'a str),
    Boolean(Boolean),
    Decimal(&'a str),
    DecimalBigInteger(&'a str),
    DecimalInteger(&'a str),
    NonDecimalInteger(NonDecimalIntegerLiteral<'a>),
    Null,
    Numeric(i64),
    StringLiteral(&'a str),
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punc {
    Brace(Brace),
    Bracket(Bracket),
    Op(Op),
    Parentheses(Parentheses),
    SemiColon,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parentheses {
    // (
    Left,
    // )
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bracket {
    // [
    Left,
    // ]
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Brace {
    // {
    Left,
    // }
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    // +
    Addition,
    // +=
    AdditonAssign,
    // =
    Assignment,
    // -
    Subtraction,
    // -=
    SubtractionAssign,
    // *
    Multiplication,
    // *=
    MultiplicationAssign,
    // /
    Division,
    // /=
    DivisionAssign,
    // **
    Exponential,
    // **=
    ExponentialAssign,
    // %
    Mod,
    // %=
    ModAssign,
    // <<
    LeftShift,
    // <<=
    LeftShiftAssign,
    // >>
    RightShift,
    // >>=
    RightShiftAssign,
    // >>>
    ZeroFillRightShift,
    // >>>=
    UnsignedRightShiftAssign,
    // >
    GreaterThan,
    // <
    LessThan,
    // ==
    Equal,
    // ===
    StrictEquality,
    // !
    Not,
    // !=
    NotEqual,
    // >=
    GreaterThanEqual,
    // <=
    LessThanEqual,
    // &&
    And,
    // &&=
    AndAssign,
    // ||
    Or,
    // ||=
    OrAssign,
    // ??
    NullishCoalescingAssign,
    // ...
    Spread,
}

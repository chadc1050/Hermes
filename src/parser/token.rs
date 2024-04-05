#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Comment(Comment),
    Eof,
    Keyword(Keyword),
    LineTerminator(LineTerminator),
    Literal(Literal),
    Punctuation(Punctuation),
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
    True,
    False,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comment {
    SingleLine,
    MultiLine,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    BigIntSuffix,
    Boolean(Boolean),
    Decimal,
    DecimalBigInteger,
    DecimalInteger,
    NonDecimalInteger(NonDecimalIntegerLiteral),
    Null,
    Numeric,
    StringLiteral,
    RegEx,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NonDecimalIntegerLiteral {
    BigInteger,
    OctalInteger,
    HexInteger,
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
pub enum Punctuation {
    Brace(Brace),
    Bracket(Bracket),
    Operator(Operator),
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
pub enum Operator {
    // +
    Addition,
    // +=
    AdditonAssignment,
    // =
    Assignment,
    // -
    Subtraction,
    // -=
    SubtractionAssignment,
    // *
    Multiplication,
    // *=
    MultiplicationAssignment,
    // /
    Division,
    // /=
    DivisionAssignment,
    // **
    Exponential,
    // **=
    ExponentialAssignment,
    // %
    Mod,
    // %=
    ModAssignment,
    // <<
    LeftShift,
    // <<=
    LeftShiftAssignment,
    // >>
    RightShift,
    // >>=
    RightShiftAssignment,
    // >>>
    ZeroFillRightShift,
    // >>>=
    UnsignedRightShiftAssignment,
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
    AndAssignment,
    // ||
    Or,
    // ||=
    OrAssignment,
    // ??
    NullishCoalescingAssignment,
    // ...
    Spread,
}

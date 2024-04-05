pub enum Token {
    Comment(Comment),
    Eof,
    Keyword(Keyword),
    LineTerminator(LineTerminator),
    Literal(Literal),
    Punctuation(Punctuation),
    WhiteSpace(WhiteSpace),
}

pub enum WhiteSpace {
    Space = 0x20,
    NoBreakSpace = 0xA0,
    CharacterTabulation = 0x09,
    LineTabulation = 0x0B,
    FormFeed = 0x0C,
}

pub enum LineTerminator {
    LineFeed = 0x000A,
    CarridgeReturn = 0x000D,
}

pub enum Boolean {
    True,
    False,
}

pub enum Comment {
    SingleLine,
    MultiLine,
}

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

pub enum NonDecimalIntegerLiteral {
    BigInteger,
    OctalInteger,
    HexInteger,
}

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

pub enum Punctuation {
    Brace(Brace),
    Bracket(Bracket),
    Operator(Operator),
    Parentheses(Parentheses),
    SemiColon,
}

pub enum Parentheses {
    Left = 0x28,
    Right = 0x29,
}

pub enum Bracket {
    Left = 0x5B,
    Right = 0x5D,
}

pub enum Brace {
    Left = 0x7B,
    Right = 0x7D,
}

pub enum Operator {
    Addition,
    AdditonAssignment,
    Assignment,
    Subtraction,
    SubtractionAssignment,
    Multiplication,
    MultiplicationAssignment,
    Division,
    DivisionAssignment,
    ExponentialAssignment,
    Mod,
    ModAssignment,
    LeftShift,
    LeftShiftAssignment,
    RightShift,
    RightShiftAssignment,
    ZeroFillRightShift,
    UnsignedRightShiftAssignment,
    GreaterThan,
    LessThan,
    Equal,
    Not,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    And,
    AndAssignment,
    Or,
    OrAssignment,
    NullishCoalescingAssignment,
    Spread,
}

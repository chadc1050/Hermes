#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

impl Token {
    pub fn new(kind: TokenKind, pos: usize) -> Self {
        Self { kind, pos }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Comment(CommentKind),
    /// End of file
    Eof,
    /// Alphabetic tokens that are not string literals, boolean literals, or keywords.
    Id(String),
    /// Language keywords
    Keyword(KeywordKind),
    LineTerminator(LineTerminatorKind),
    /// Value literals
    Lit(LitKind),
    Punc(PuncKind),
    WhiteSpace(WhiteSpaceKind),
    /// Any unknown characters that we are unable to identify
    Unicode(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WhiteSpaceKind {
    ///
    Space,
    /// \t
    HorizontalTabulation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineTerminatorKind {
    /// \n
    LineFeed,
    /// \r
    CarriageReturn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BooleanKind {
    /// true
    True,
    /// false
    False,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentKind {
    SingleLine(String),
    MultiLine(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LitKind {
    BigIntSuffix(String),
    Bool(BooleanKind),
    Dec(String),
    DecimalBigInteger(String),
    DecimalInteger(String),
    NonDecimalInteger(NonDecimalIntegerLiteralKind),
    Null,
    Num(i64),
    String(String),
    RegEx(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NonDecimalIntegerLiteralKind {
    BigInteger(String),
    OctalInteger(String),
    HexInteger(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordKind {
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

pub fn map_keyword(keyword_str: &str) -> Option<KeywordKind> {
    match keyword_str {
        "await" => Some(KeywordKind::Await),
        "break" => Some(KeywordKind::Break),
        "case" => Some(KeywordKind::Case),
        "catch" => Some(KeywordKind::Catch),
        "class" => Some(KeywordKind::Class),
        "const" => Some(KeywordKind::Const),
        "continue" => Some(KeywordKind::Continue),
        "debugger" => Some(KeywordKind::Debugger),
        "default" => Some(KeywordKind::Default),
        "delete" => Some(KeywordKind::Delete),
        "do" => Some(KeywordKind::Do),
        "else" => Some(KeywordKind::Else),
        "export" => Some(KeywordKind::Export),
        "extends" => Some(KeywordKind::Extends),
        "finally" => Some(KeywordKind::Finally),
        "for" => Some(KeywordKind::For),
        "function" => Some(KeywordKind::Function),
        "if" => Some(KeywordKind::If),
        "import" => Some(KeywordKind::Import),
        "in" => Some(KeywordKind::In),
        "instanceof" => Some(KeywordKind::InstanceOf),
        "let" => Some(KeywordKind::Let),
        "new" => Some(KeywordKind::New),
        "return" => Some(KeywordKind::Return),
        "static" => Some(KeywordKind::Static),
        "super" => Some(KeywordKind::Super),
        "switch" => Some(KeywordKind::Switch),
        "this" => Some(KeywordKind::This),
        "throw" => Some(KeywordKind::Throw),
        "try" => Some(KeywordKind::Try),
        "typeof" => Some(KeywordKind::TypeOf),
        "var" => Some(KeywordKind::Var),
        "void" => Some(KeywordKind::Void),
        "while" => Some(KeywordKind::While),
        "with" => Some(KeywordKind::With),
        "yield" => Some(KeywordKind::Yield),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PuncKind {
    Brace(BraceKind),
    Bracket(BracketKind),
    /// .
    Dot,
    Op(OpKind),
    Parentheses(ParenthesesKind),
    /// ;
    SemiColon,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParenthesesKind {
    /// (
    Left,
    /// )
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BracketKind {
    /// [
    Left,
    /// ]
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BraceKind {
    /// {
    Left,
    /// }
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpKind {
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

pub fn is_removable(token: &TokenKind) -> bool {
    match token {
        TokenKind::WhiteSpace(_) => true,
        TokenKind::Comment(_) => true,
        _ => false,
    }
}

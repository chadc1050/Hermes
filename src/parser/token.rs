pub enum Token {
    Comment(Comment),
    Eof,
    Operator(Operator),
    WhiteSpace(WhiteSpace),
}

pub enum WhiteSpace {
    Space,
    Tab,
    NewLine,
    FormFeed,
    CarridgeReturn,
}

pub enum Comment {
    SingleLine,
    MultiLine,
}

pub enum Operator {
    Plus,
    Subtract,
    Multiply,
    Divide,
}

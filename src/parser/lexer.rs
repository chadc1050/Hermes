use std::cell::RefCell;
use std::rc::Rc;

use super::reader::Reader;

use super::token::{
    is_removable, map_keyword, BooleanKind, BraceKind, BracketKind, LineTerminatorKind, LiteralKind, OpKind, ParenthesesKind, PuncKind, TokenKind,
    WhiteSpaceKind,
};

pub struct Lexer {
    reader: Rc<RefCell<Reader<char>>>,
}

impl Lexer {
    pub fn init(source: &str) -> Self {
        Lexer { reader: Rc::new(RefCell::new(Reader::init(source.chars().collect::<Vec<char>>()))) }
    }

    pub fn tokenize(&mut self) -> Vec<TokenKind> {
        let mut tokens = Vec::new();

        loop {
            match self.lex() {
                Ok(token) => {
                    if !is_removable(&token) {
                        tokens.push(token.clone());
                    }
                    if token == TokenKind::Eof {
                        break;
                    }
                }
                Err(err) => panic!("Error occurred parsing! Error: {err}"),
            }
        }

        tokens
    }

    fn lex(&self) -> Result<TokenKind, &str> {
        let mut reader = self.reader.borrow_mut();
        match reader.next_single() {
            Some(first) => {
                if first.is_digit(10) {
                    return self.lex_numeric(&mut reader, first);
                }

                if first.is_alphabetic() {
                    return self.lex_identifier(&mut reader, first);
                }

                match first {
                    '"' => self.lex_string_literal(&mut reader),
                    '\t' => Ok(TokenKind::WhiteSpace(WhiteSpaceKind::HorizontalTabulation)),
                    '\n' => Ok(TokenKind::LineTerminator(LineTerminatorKind::LineFeed)),
                    '\r' => Ok(TokenKind::LineTerminator(LineTerminatorKind::CarriageReturn)),
                    ' ' => Ok(TokenKind::WhiteSpace(WhiteSpaceKind::Space)),
                    '(' => Ok(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left))),
                    ')' => Ok(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right))),
                    '=' => match reader.peek_single() {
                        Some(second) => match second {
                            '=' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(TokenKind::Punc(PuncKind::Op(OpKind::StrictEquality)))
                                        }
                                        _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Equal))),
                                    },
                                    None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Equal))),
                                }
                            }
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Assign))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Assign))),
                    },
                    '*' => match reader.peek_single() {
                        Some(second) => match second {
                            '*' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(TokenKind::Punc(PuncKind::Op(OpKind::ExponentialAssign)))
                                        }
                                        _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Exponential))),
                                    },
                                    None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Exponential))),
                                }
                            }
                            '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::MultiplicationAssign))),
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Multiplication))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Multiplication))),
                    },
                    '&' => match reader.peek_single() {
                        Some(second) => match second {
                            '&' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(TokenKind::Punc(PuncKind::Op(OpKind::AndAssign)))
                                        }
                                        _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::And))),
                                    },
                                    None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::And))),
                                }
                            }
                            '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::BitAndAssing))),
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::BitAnd))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::BitAnd))),
                    },
                    '|' => match reader.peek_single() {
                        Some(second) => match second {
                            '|' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(TokenKind::Punc(PuncKind::Op(OpKind::OrAssign)))
                                        }
                                        _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Or))),
                                    },
                                    None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Or))),
                                }
                            }
                            '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::BitOrAssign))),
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::BitOr))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::BitOr))),
                    },
                    '^' => self.lex_assignable_operator(&mut reader, OpKind::BitXor, OpKind::BitXorAssign),
                    '+' => match reader.peek_single() {
                        Some(second) => match second {
                            '+' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Increment))),
                            '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::AdditonAssign))),
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Addition))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Addition))),
                    },
                    '-' => match reader.peek_single() {
                        Some(second) => match second {
                            '-' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Decrement))),
                            '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::SubtractionAssign))),
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Subtraction))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Subtraction))),
                    },
                    '/' => self.lex_assignable_operator(&mut reader, OpKind::Division, OpKind::DivisionAssign),
                    '%' => self.lex_assignable_operator(&mut reader, OpKind::Mod, OpKind::ModAssign),
                    '>' => match reader.peek_single() {
                        Some(second) => match second {
                            '>' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '>' => {
                                            reader.bump();
                                            match reader.peek_single() {
                                                Some(fourth) => match fourth {
                                                    '=' => {
                                                        reader.bump();
                                                        Ok(TokenKind::Punc(PuncKind::Op(OpKind::UnsignedRightShiftAssign)))
                                                    }
                                                    _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::ZeroFillRightShift))),
                                                },
                                                None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::ZeroFillRightShift))),
                                            }
                                        }
                                        '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::RightShiftAssign))),
                                        _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::RightShift))),
                                    },
                                    None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::RightShift))),
                                }
                            }
                            '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::GreaterThanEqual))),
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::GreaterThan))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::GreaterThan))),
                    },
                    '<' => match reader.peek_single() {
                        Some(second) => match second {
                            '<' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(TokenKind::Punc(PuncKind::Op(OpKind::LeftShiftAssign)))
                                        }
                                        _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::LeftShift))),
                                    },
                                    None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::LeftShift))),
                                }
                            }
                            '=' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::LessThanEqual))),
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::LessThan))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::LessThan))),
                    },
                    '!' => match reader.peek_single() {
                        Some(second) => match second {
                            '=' => {
                                reader.bump();
                                Ok(TokenKind::Punc(PuncKind::Op(OpKind::NotEqual)))
                            }
                            _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Not))),
                        },
                        None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::Not))),
                    },
                    '?' => match reader.peek_single() {
                        Some(second) => match second {
                            '?' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(TokenKind::Punc(PuncKind::Op(OpKind::NullishCoalescingAssign)))
                                        }
                                        _ => Ok(TokenKind::Punc(PuncKind::Op(OpKind::NullishCoalescing))),
                                    },
                                    None => Ok(TokenKind::Punc(PuncKind::Op(OpKind::OptionalChain))),
                                }
                            }
                            '.' => Ok(TokenKind::Punc(PuncKind::Op(OpKind::OptionalChain))),
                            _ => Err("Invalid Token"),
                        },
                        None => Err("Invalid Token"),
                    },
                    '.' => Ok(TokenKind::Punc(PuncKind::Dot)),
                    ';' => Ok(TokenKind::Punc(PuncKind::SemiColon)),
                    '[' => Ok(TokenKind::Punc(PuncKind::Bracket(BracketKind::Left))),
                    ']' => Ok(TokenKind::Punc(PuncKind::Bracket(BracketKind::Right))),
                    '{' => Ok(TokenKind::Punc(PuncKind::Brace(BraceKind::Left))),
                    '}' => Ok(TokenKind::Punc(PuncKind::Brace(BraceKind::Right))),
                    _ => Ok(TokenKind::Unicode(first.to_string())),
                }
            }
            None => return Ok(TokenKind::Eof),
        }
    }

    /// Handles alphabetic tokens encapsulated by
    fn lex_string_literal(&self, reader: &mut Reader<char>) -> Result<TokenKind, &str> {
        let mut word = String::new();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_alphabetic() {
                        reader.bump();
                        word.push(peek);
                    } else if peek == '"' {
                        reader.bump();
                        return Ok(TokenKind::Literal(LiteralKind::StringLiteral(word)));
                    }
                }
                None => {
                    return Err("Unexpected end of string literal!");
                }
            }
        }
    }

    /// Handles all alphabetic tokens not encapsulated by quotations (non-string literals)
    fn lex_identifier(&self, reader: &mut Reader<char>, char: char) -> Result<TokenKind, &str> {
        let mut word = char.to_string();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_alphabetic() {
                        word.push(peek);
                        reader.bump();
                    } else {
                        if let Some(keyword) = map_keyword(&word) {
                            return Ok(TokenKind::Keyword(keyword));
                        } else if word == "true" {
                            return Ok(TokenKind::Literal(LiteralKind::Boolean(BooleanKind::True)));
                        } else if word == "false" {
                            return Ok(TokenKind::Literal(LiteralKind::Boolean(BooleanKind::False)));
                        } else {
                            return Ok(TokenKind::Identifier(word));
                        }
                    }
                }
                None => {
                    if let Some(keyword) = map_keyword(&word) {
                        return Ok(TokenKind::Keyword(keyword));
                    } else if word == "true" {
                        return Ok(TokenKind::Literal(LiteralKind::Boolean(BooleanKind::True)));
                    } else if word == "false" {
                        return Ok(TokenKind::Literal(LiteralKind::Boolean(BooleanKind::False)));
                    } else {
                        return Ok(TokenKind::Identifier(word));
                    }
                }
            }
        }
    }

    /// Given a numeric character, parses the rest of the numeric and determines numeric variant.
    /// TODO: Need to check for decimals and non-decimal number types.
    fn lex_numeric(&self, reader: &mut Reader<char>, char: char) -> Result<TokenKind, &str> {
        let mut val = char.to_string();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_digit(10) {
                        val.push(peek);
                        reader.bump();
                    } else {
                        return Ok(TokenKind::Literal(LiteralKind::Numeric(val.parse().unwrap())));
                    }
                }
                None => return Ok(TokenKind::Literal(LiteralKind::Numeric(val.parse().unwrap()))),
            }
        }
    }

    fn lex_assignable_operator(&self, reader: &mut Reader<char>, operator: OpKind, assign: OpKind) -> Result<TokenKind, &str> {
        match reader.peek_single() {
            Some(second) => match second {
                '=' => {
                    reader.bump();
                    Ok(TokenKind::Punc(PuncKind::Op(assign)))
                }
                _ => Ok(TokenKind::Punc(PuncKind::Op(operator))),
            },
            None => Ok(TokenKind::Punc(PuncKind::Op(operator))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::{BooleanKind, KeywordKind, LiteralKind, OpKind, ParenthesesKind, PuncKind, TokenKind};

    use super::Lexer;

    #[test]
    fn test_tokenize() {
        let mut lexer = Lexer::init("testing 123");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Identifier("testing".into()), res[0]);
        assert_eq!(TokenKind::Literal(LiteralKind::Numeric(123)), res[1]);
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::init(" ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Eof, res[0]);

        let mut lexer = Lexer::init("");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Eof, res[0]);
    }

    #[test]
    fn test_punctuation() {
        let mut lexer = Lexer::init(";");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::SemiColon), res[0]);
    }

    #[test]
    fn test_boolean() {
        let mut lexer = Lexer::init("true false");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Literal(LiteralKind::Boolean(BooleanKind::True)), res[0]);
        assert_eq!(TokenKind::Literal(LiteralKind::Boolean(BooleanKind::False)), res[1]);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::init("\"true\"");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Literal(LiteralKind::StringLiteral("true".into())), res[0]);
    }

    #[test]
    fn test_string_identifier() {
        let mut lexer = Lexer::init("let test = new Tokenizer(\"debugger\");");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Keyword(KeywordKind::Let), res[0]);
        assert_eq!(TokenKind::Identifier("test".into()), res[1]);
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::Assign)), res[2]);
        assert_eq!(TokenKind::Keyword(KeywordKind::New), res[3]);
        assert_eq!(TokenKind::Identifier("Tokenizer".into()), res[4]);
        assert_eq!(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)), res[5]);
        assert_eq!(TokenKind::Literal(LiteralKind::StringLiteral("debugger".into())), res[6]);
        assert_eq!(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)), res[7]);
        assert_eq!(TokenKind::Punc(PuncKind::SemiColon), res[8]);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::init("+= ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::AdditonAssign)), res[0]);

        let mut lexer = Lexer::init("*= 3");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::MultiplicationAssign)), res[0]);

        let mut lexer = Lexer::init("**= 3");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::ExponentialAssign)), res[0]);

        let mut lexer = Lexer::init("** 3");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::Exponential)), res[0]);

        let mut lexer = Lexer::init("& ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::BitAnd)), res[0]);

        let mut lexer = Lexer::init("&&= ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::AndAssign)), res[0]);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::init("await yield");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Keyword(KeywordKind::Await), res[0]);
        assert_eq!(TokenKind::Keyword(KeywordKind::Yield), res[1]);

        let mut lexer = Lexer::init("let x = await y;");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Keyword(KeywordKind::Let), res[0]);
        assert_eq!(TokenKind::Identifier("x".into()), res[1]);
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::Assign)), res[2]);
        assert_eq!(TokenKind::Keyword(KeywordKind::Await), res[3]);
        assert_eq!(TokenKind::Identifier("y".into()), res[4]);
    }

    #[test]
    fn test_numerics() {
        let mut lexer = Lexer::init("356 ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Literal(LiteralKind::Numeric(356)), res[0]);
    }
}

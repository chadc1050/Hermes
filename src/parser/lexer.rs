use std::cell::RefCell;
use std::rc::Rc;

use super::reader::Reader;

use super::token::{
    is_whitespace, map_keyword, Boolean, Brace, Bracket, LineTerminator, Literal, Op, Parentheses, Punc, Token,
    WhiteSpace,
};

pub struct Lexer {
    reader: Rc<RefCell<Reader>>,
}

impl Lexer {
    pub fn init(source: &str) -> Self {
        Lexer { reader: Rc::new(RefCell::new(Reader::init(source))) }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.lex() {
                Ok(token) => {
                    if !is_whitespace(&token) {
                        tokens.push(token.clone());
                    }
                    if token == Token::Eof {
                        break;
                    }
                }
                Err(err) => panic!("Error occurred parsing! Error: {err}"),
            }
        }

        tokens
    }

    fn lex(&self) -> Result<Token, &str> {
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
                    '\t' => Ok(Token::WhiteSpace(WhiteSpace::HorizontalTabulation)),
                    '\n' => Ok(Token::LineTerminator(LineTerminator::LineFeed)),
                    '\r' => Ok(Token::LineTerminator(LineTerminator::CarridgeReturn)),
                    ' ' => Ok(Token::WhiteSpace(WhiteSpace::Space)),
                    '(' => Ok(Token::Punc(Punc::Parentheses(Parentheses::Left))),
                    ')' => Ok(Token::Punc(Punc::Parentheses(Parentheses::Right))),
                    '=' => match reader.peek_single() {
                        Some(second) => match second {
                            '=' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::Punc(Punc::Op(Op::StrictEquality)))
                                        }
                                        _ => Ok(Token::Punc(Punc::Op(Op::Equal))),
                                    },
                                    None => Ok(Token::Punc(Punc::Op(Op::Equal))),
                                }
                            }
                            _ => Ok(Token::Punc(Punc::Op(Op::Assign))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::Assign))),
                    },
                    '*' => match reader.peek_single() {
                        Some(second) => match second {
                            '*' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::Punc(Punc::Op(Op::ExponentialAssign)))
                                        }
                                        _ => Ok(Token::Punc(Punc::Op(Op::Exponential))),
                                    },
                                    None => Ok(Token::Punc(Punc::Op(Op::Exponential))),
                                }
                            }
                            '=' => Ok(Token::Punc(Punc::Op(Op::MultiplicationAssign))),
                            _ => Ok(Token::Punc(Punc::Op(Op::Multiplication))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::Multiplication))),
                    },
                    '&' => match reader.peek_single() {
                        Some(second) => match second {
                            '&' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::Punc(Punc::Op(Op::AndAssign)))
                                        }
                                        _ => Ok(Token::Punc(Punc::Op(Op::And))),
                                    },
                                    None => Ok(Token::Punc(Punc::Op(Op::And))),
                                }
                            }
                            '=' => Ok(Token::Punc(Punc::Op(Op::BitAndAssing))),
                            _ => Ok(Token::Punc(Punc::Op(Op::BitAnd))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::BitAnd))),
                    },
                    '|' => match reader.peek_single() {
                        Some(second) => match second {
                            '|' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::Punc(Punc::Op(Op::OrAssign)))
                                        }
                                        _ => Ok(Token::Punc(Punc::Op(Op::Or))),
                                    },
                                    None => Ok(Token::Punc(Punc::Op(Op::Or))),
                                }
                            }
                            '=' => Ok(Token::Punc(Punc::Op(Op::BitOrAssign))),
                            _ => Ok(Token::Punc(Punc::Op(Op::BitOr))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::BitOr))),
                    },
                    '^' => self.lex_assignable_operator(&mut reader, Op::BitXor, Op::BitXorAssign),
                    '+' => match reader.peek_single() {
                        Some(second) => match second {
                            '+' => Ok(Token::Punc(Punc::Op(Op::Increment))),
                            '=' => Ok(Token::Punc(Punc::Op(Op::AdditonAssign))),
                            _ => Ok(Token::Punc(Punc::Op(Op::Addition))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::Addition))),
                    },
                    '-' => match reader.peek_single() {
                        Some(second) => match second {
                            '-' => Ok(Token::Punc(Punc::Op(Op::Decrement))),
                            '=' => Ok(Token::Punc(Punc::Op(Op::SubtractionAssign))),
                            _ => Ok(Token::Punc(Punc::Op(Op::Subtraction))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::Subtraction))),
                    },
                    '/' => self.lex_assignable_operator(&mut reader, Op::Division, Op::DivisionAssign),
                    '%' => self.lex_assignable_operator(&mut reader, Op::Mod, Op::ModAssign),
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
                                                        Ok(Token::Punc(Punc::Op(Op::UnsignedRightShiftAssign)))
                                                    }
                                                    _ => Ok(Token::Punc(Punc::Op(Op::ZeroFillRightShift))),
                                                },
                                                None => Ok(Token::Punc(Punc::Op(Op::ZeroFillRightShift))),
                                            }
                                        }
                                        '=' => Ok(Token::Punc(Punc::Op(Op::RightShiftAssign))),
                                        _ => Ok(Token::Punc(Punc::Op(Op::RightShift))),
                                    },
                                    None => Ok(Token::Punc(Punc::Op(Op::RightShift))),
                                }
                            }
                            '=' => Ok(Token::Punc(Punc::Op(Op::GreaterThanEqual))),
                            _ => Ok(Token::Punc(Punc::Op(Op::GreaterThan))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::GreaterThan))),
                    },
                    '<' => match reader.peek_single() {
                        Some(second) => match second {
                            '<' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::Punc(Punc::Op(Op::LeftShiftAssign)))
                                        }
                                        _ => Ok(Token::Punc(Punc::Op(Op::LeftShift))),
                                    },
                                    None => Ok(Token::Punc(Punc::Op(Op::LeftShift))),
                                }
                            }
                            '=' => Ok(Token::Punc(Punc::Op(Op::LessThanEqual))),
                            _ => Ok(Token::Punc(Punc::Op(Op::LessThan))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::LessThan))),
                    },
                    '!' => match reader.peek_single() {
                        Some(second) => match second {
                            '=' => {
                                reader.bump();
                                Ok(Token::Punc(Punc::Op(Op::NotEqual)))
                            }
                            _ => Ok(Token::Punc(Punc::Op(Op::Not))),
                        },
                        None => Ok(Token::Punc(Punc::Op(Op::Not))),
                    },
                    '?' => match reader.peek_single() {
                        Some(second) => match second {
                            '?' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::Punc(Punc::Op(Op::NullishCoalescingAssign)))
                                        }
                                        _ => Ok(Token::Punc(Punc::Op(Op::NullishCoalescing))),
                                    },
                                    None => Ok(Token::Punc(Punc::Op(Op::OptionalChain))),
                                }
                            }
                            '.' => Ok(Token::Punc(Punc::Op(Op::OptionalChain))),
                            _ => Err("Invalid Token"),
                        },
                        None => Err("Invalid Token"),
                    },
                    '.' => Ok(Token::Punc(Punc::Dot)),
                    ';' => Ok(Token::Punc(Punc::SemiColon)),
                    '[' => Ok(Token::Punc(Punc::Bracket(Bracket::Left))),
                    ']' => Ok(Token::Punc(Punc::Bracket(Bracket::Right))),
                    '{' => Ok(Token::Punc(Punc::Brace(Brace::Left))),
                    '}' => Ok(Token::Punc(Punc::Brace(Brace::Right))),
                    _ => Ok(Token::Unicode(first.to_string())),
                }
            }
            None => return Ok(Token::Eof),
        }
    }

    /// Handles alphabetic tokens encapsulated by
    fn lex_string_literal(&self, reader: &mut Reader) -> Result<Token, &str> {
        let mut word = String::new();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_alphabetic() {
                        reader.bump();
                        word.push(peek);
                    } else if peek == '"' {
                        reader.bump();
                        return Ok(Token::Literal(Literal::StringLiteral(word)));
                    }
                }
                None => {
                    return Err("Unexpected end of string literal!");
                }
            }
        }
    }

    /// Handles all alphabetic tokens not encapsulated by quotations (non-string literals)
    fn lex_identifier(&self, reader: &mut Reader, char: char) -> Result<Token, &str> {
        let mut word = char.to_string();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_alphabetic() {
                        word.push(peek);
                        reader.bump();
                    } else {
                        if let Some(keyword) = map_keyword(&word) {
                            return Ok(Token::Keyword(keyword));
                        } else if word == "true" {
                            return Ok(Token::Literal(Literal::Boolean(Boolean::True)));
                        } else if word == "false" {
                            return Ok(Token::Literal(Literal::Boolean(Boolean::False)));
                        } else {
                            return Ok(Token::Identifier(word));
                        }
                    }
                }
                None => {
                    if let Some(keyword) = map_keyword(&word) {
                        return Ok(Token::Keyword(keyword));
                    } else if word == "true" {
                        return Ok(Token::Literal(Literal::Boolean(Boolean::True)));
                    } else if word == "false" {
                        return Ok(Token::Literal(Literal::Boolean(Boolean::False)));
                    } else {
                        return Ok(Token::Identifier(word));
                    }
                }
            }
        }
    }

    /// Given a numeric character, parses the rest of the numeric and determines numeric variant.
    /// TODO: Need to check for decimals and non-decimal number types.
    fn lex_numeric(&self, reader: &mut Reader, char: char) -> Result<Token, &str> {
        let mut val = char.to_string();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_digit(10) {
                        val.push(peek);
                        reader.bump();
                    } else {
                        return Ok(Token::Literal(Literal::Numeric(val.parse().unwrap())));
                    }
                }
                None => return Ok(Token::Literal(Literal::Numeric(val.parse().unwrap()))),
            }
        }
    }

    fn lex_assignable_operator(&self, reader: &mut Reader, operator: Op, assign: Op) -> Result<Token, &str> {
        match reader.peek_single() {
            Some(second) => match second {
                '=' => {
                    reader.bump();
                    Ok(Token::Punc(Punc::Op(assign)))
                }
                _ => Ok(Token::Punc(Punc::Op(operator))),
            },
            None => Ok(Token::Punc(Punc::Op(operator))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::{Boolean, Keyword, Literal, Op, Parentheses, Punc, Token};

    use super::Lexer;

    #[test]
    fn test_tokenize() {
        let mut lexer = Lexer::init("testing 123");
        let res = lexer.tokenize();
        assert_eq!(Token::Identifier("testing".into()), res[0]);
        assert_eq!(Token::Literal(Literal::Numeric(123)), res[1]);
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::init(" ");
        let res = lexer.tokenize();
        assert_eq!(Token::Eof, res[0]);

        let mut lexer = Lexer::init("");
        let res = lexer.tokenize();
        assert_eq!(Token::Eof, res[0]);
    }

    #[test]
    fn test_punctuation() {
        let mut lexer = Lexer::init(";");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::SemiColon), res[0]);
    }

    #[test]
    fn test_boolean() {
        let mut lexer = Lexer::init("true false");
        let res = lexer.tokenize();
        assert_eq!(Token::Literal(Literal::Boolean(Boolean::True)), res[0]);
        assert_eq!(Token::Literal(Literal::Boolean(Boolean::False)), res[1]);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::init("\"true\"");
        let res = lexer.tokenize();
        assert_eq!(Token::Literal(Literal::StringLiteral("true".into())), res[0]);
    }

    #[test]
    fn test_string_identifier() {
        let mut lexer = Lexer::init("let test = new Tokenizer(\"debugger\");");
        let res = lexer.tokenize();
        assert_eq!(Token::Keyword(Keyword::Let), res[0]);
        assert_eq!(Token::Identifier("test".into()), res[1]);
        assert_eq!(Token::Punc(Punc::Op(Op::Assign)), res[2]);
        assert_eq!(Token::Keyword(Keyword::New), res[3]);
        assert_eq!(Token::Identifier("Tokenizer".into()), res[4]);
        assert_eq!(Token::Punc(Punc::Parentheses(Parentheses::Left)), res[5]);
        assert_eq!(Token::Literal(Literal::StringLiteral("debugger".into())), res[6]);
        assert_eq!(Token::Punc(Punc::Parentheses(Parentheses::Right)), res[7]);
        assert_eq!(Token::Punc(Punc::SemiColon), res[8]);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::init("+= ");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::AdditonAssign)), res[0]);

        let mut lexer = Lexer::init("*= 3");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::MultiplicationAssign)), res[0]);

        let mut lexer = Lexer::init("**= 3");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::ExponentialAssign)), res[0]);

        let mut lexer = Lexer::init("** 3");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::Exponential)), res[0]);

        let mut lexer = Lexer::init("& ");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::BitAnd)), res[0]);

        let mut lexer = Lexer::init("&&= ");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::AndAssign)), res[0]);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::init("await yield");
        let res = lexer.tokenize();
        assert_eq!(Token::Keyword(Keyword::Await), res[0]);
        assert_eq!(Token::Keyword(Keyword::Yield), res[1]);

        let mut lexer = Lexer::init("let x = await y;");
        let res = lexer.tokenize();
        assert_eq!(Token::Keyword(Keyword::Let), res[0]);
        assert_eq!(Token::Identifier("x".into()), res[1]);
        assert_eq!(Token::Punc(Punc::Op(Op::Assign)), res[2]);
        assert_eq!(Token::Keyword(Keyword::Await), res[3]);
        assert_eq!(Token::Identifier("y".into()), res[4]);
    }

    #[test]
    fn test_numerics() {
        let mut lexer = Lexer::init("356 ");
        let res = lexer.tokenize();
        assert_eq!(Token::Literal(Literal::Numeric(356)), res[0]);
    }
}

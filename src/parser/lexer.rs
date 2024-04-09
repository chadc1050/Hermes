use std::cell::RefCell;
use std::rc::Rc;

use super::reader::{self, Reader};

use super::token::{Brace, Bracket, LineTerminator, Literal, Op, Parentheses, Punc, Token, WhiteSpace};

pub struct Lexer<'a> {
    reader: Rc<RefCell<Reader>>,
    last: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn init(source: &str) -> Self {
        Lexer { reader: Rc::new(RefCell::new(Reader::init(source))), last: None }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.lex() {
                Ok(token) => {
                    tokens.push(token.clone());
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
                    return self.lex_alphabetic(&mut reader, first);
                }

                match first {
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
                                    None => Err("Unexpected end."),
                                }
                            }
                            _ => Ok(Token::Punc(Punc::Op(Op::Assignment))),
                        },
                        None => Err("Unexpected end."),
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
                                    None => Err("Unexpected end."),
                                }
                            }
                            '=' => Ok(Token::Punc(Punc::Op(Op::MultiplicationAssign))),
                            _ => Ok(Token::Punc(Punc::Op(Op::Multiplication))),
                        },
                        None => Err("Unexpected end."),
                    },
                    '+' => self.lex_assignable_operator(&mut reader, Op::Addition, Op::AdditonAssign),
                    '-' => self.lex_assignable_operator(&mut reader, Op::Subtraction, Op::SubtractionAssign),
                    '/' => self.lex_assignable_operator(&mut reader, Op::Division, Op::DivisionAssign),
                    '%' => self.lex_assignable_operator(&mut reader, Op::Mod, Op::ModAssign),
                    ';' => Ok(Token::Punc(Punc::SemiColon)),
                    '[' => Ok(Token::Punc(Punc::Bracket(Bracket::Left))),
                    ']' => Ok(Token::Punc(Punc::Bracket(Bracket::Right))),
                    '{' => Ok(Token::Punc(Punc::Brace(Brace::Left))),
                    '}' => Ok(Token::Punc(Punc::Brace(Brace::Right))),
                    _ => Err("Unexpected char"),
                }
            }
            None => return Ok(Token::Eof),
        }
    }

    fn lex_alphabetic(&self, reader: &mut Reader, char: char) -> Result<Token, &str> {
        let mut word = char.to_string();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_alphabetic() {
                        word.push(peek);
                        reader.bump();
                    } else {
                        return Ok(Token::Literal(Literal::StringLiteral(word)));
                    }
                }
                None => return Ok(Token::Literal(Literal::StringLiteral(word))),
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
        // Check keywords
        match reader.peek_single() {
            Some(second) => match second {
                '=' => {
                    reader.bump();
                    Ok(Token::Punc(Punc::Op(assign)))
                }
                _ => Ok(Token::Punc(Punc::Op(operator))),
            },
            None => Err("Unexpected end."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::{Literal, Op, Punc, Token, WhiteSpace};

    use super::Lexer;

    #[test]
    fn test_tokenize() {
        let mut lexer = Lexer::init(" ");
        let res = lexer.tokenize();
        assert_eq!(Token::WhiteSpace(WhiteSpace::Space), res[0]);

        let mut lexer = Lexer::init("");
        let res = lexer.tokenize();
        assert_eq!(Token::Eof, res[0]);

        let mut lexer = Lexer::init(";");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::SemiColon), res[0]);

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

        let mut lexer = Lexer::init("356 ");
        let res = lexer.tokenize();
        assert_eq!(Token::Literal(Literal::Numeric(356)), res[0]);

        let mut lexer = Lexer::init("testing 123");
        let res = lexer.tokenize();
        assert_eq!(Token::Literal(Literal::StringLiteral("testing".into())), res[0]);
        assert_eq!(Token::Literal(Literal::Numeric(123)), res[2]);
    }
}

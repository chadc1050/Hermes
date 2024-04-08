use std::cell::RefCell;
use std::rc::Rc;

use super::reader::Reader;

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
                    tokens.push(token);
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
                    return self.lex_numeric(&reader, first);
                }

                if first.is_alphabetic() {
                    return self.lex_alphabetic(&reader, first);
                }

                match first {
                    '\t' => Ok(Token::WhiteSpace(WhiteSpace::HorizontalTabulation)),
                    '\n' => Ok(Token::LineTerminator(LineTerminator::LineFeed)),
                    '\r' => Ok(Token::LineTerminator(LineTerminator::CarridgeReturn)),
                    ' ' => Ok(Token::WhiteSpace(WhiteSpace::Space)),
                    '(' => Ok(Token::Punc(Punc::Parentheses(Parentheses::Left))),
                    ')' => Ok(Token::Punc(Punc::Parentheses(Parentheses::Right))),
                    '=' => match reader.peek(2) {
                        Some(second) => match second[1] {
                            '=' => match reader.peek(3) {
                                Some(third) => match third[2] {
                                    '=' => Ok(Token::Punc(Punc::Op(Op::StrictEquality))),
                                    _ => Ok(Token::Punc(Punc::Op(Op::Equal))),
                                },
                                None => Err("Unexpected end."),
                            },
                            _ => Ok(Token::Punc(Punc::Op(Op::Assignment))),
                        },
                        None => Err("Unexpected end."),
                    },
                    '*' => match reader.peek(2) {
                        Some(second) => match second[1] {
                            '*' => match reader.peek(3) {
                                Some(third) => match third[2] {
                                    '=' => Ok(Token::Punc(Punc::Op(Op::ExponentialAssign))),
                                    _ => Ok(Token::Punc(Punc::Op(Op::Exponential))),
                                },
                                None => Err("Unexpected end."),
                            },
                            '=' => Ok(Token::Punc(Punc::Op(Op::MultiplicationAssign))),
                            _ => Ok(Token::Punc(Punc::Op(Op::Multiplication))),
                        },
                        None => Err("Unexpected end."),
                    },
                    '+' => self.lex_assignable_operator(&reader, Op::Addition, Op::AdditonAssign),
                    '-' => self.lex_assignable_operator(&reader, Op::Subtraction, Op::SubtractionAssign),
                    '/' => self.lex_assignable_operator(&reader, Op::Division, Op::DivisionAssign),
                    '%' => self.lex_assignable_operator(&reader, Op::Mod, Op::ModAssign),
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

    fn lex_alphabetic(&self, reader: &Reader, char: char) -> Result<Token, &str> {
        let mut idx = 1;
        let alpha = char.to_string();
        loop {
            idx += 1;
            let peek = reader.peek(idx);
            match peek {
                Some(word) => match &word.into_iter().collect::<String>() {
                    _ => continue,
                },
                None => return Err("Unexpected end."),
            }
        }
    }

    fn lex_numeric(&self, reader: &Reader, char: char) -> Result<Token, &str> {
        let val = char.to_string();
        let mut idx = 1;
        loop {
            idx += 1;
            match reader.peek(idx) {
                Some(number) => {
                    if !number[idx - 1].is_digit(10) {
                        match number.into_iter().collect::<String>().parse::<i64>() {
                            Ok(parsed) => return Ok(Token::Literal(Literal::Numeric(parsed))),
                            Err(_) => return Err("Invalid numeric!"),
                        }
                    }
                }
                None => return Err("Unexpected end."),
            }
        }
    }

    fn lex_assignable_operator(&self, reader: &Reader, operator: Op, assign: Op) -> Result<Token, &str> {
        // Check keywords
        match reader.peek(2) {
            Some(second) => match second[1] {
                '=' => Ok(Token::Punc(Punc::Op(assign))),
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
    fn test_peek() {
        let mut lexer = Lexer::init(" ");
        let res = lexer.tokenize();
        assert_eq!(Token::WhiteSpace(WhiteSpace::Space), res[0]);

        let mut lexer = Lexer::init("");
        let res = lexer.tokenize();
        assert_eq!(Token::Eof, res[0]);

        let mut lexer = Lexer::init(";");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::SemiColon), res[0]);

        let mut lexer = Lexer::init("+= 2");
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
    }
}

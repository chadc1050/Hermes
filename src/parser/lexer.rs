use super::reader::Reader;

use super::token::{Brace, Bracket, LineTerminator, Literal, Operator, Parentheses, Punctuation, Token, WhiteSpace};

pub struct Lexer {
    reader: Reader,
}

impl Lexer {
    pub fn init(source: &str) -> Self {
        Lexer { reader: Reader::init(source) }
    }

    pub fn peek(&self) -> Token {
        let mut buffer_size = 1;
        loop {
            match self.reader.peek(buffer_size) {
                Some(chars) => {
                    let res = self.lex(chars);
                    match res {
                        Ok(val) => match val {
                            Some(token) => return token,
                            None => buffer_size += 1,
                        },
                        Err(err) => panic!("Error occurred parsing! Error: {err}"),
                    }
                }
                None => return Token::Eof,
            }
        }
    }

    pub fn consume(&mut self) -> Token {
        todo!("Implement consume")
    }

    fn lex(&self, buffer: &[char]) -> Result<Option<Token>, &str> {
        let char = buffer[0];

        if char.is_digit(10) {
            let mut idx = 1;
            loop {
                idx += 1;
                match self.reader.peek(idx) {
                    Some(number) => {
                        if !number[idx - 1].is_digit(10) {
                            return Ok(Some(Token::Literal(Literal::Numeric)));
                        }
                    }
                    None => return Err("Unexpected end."),
                }
            }
        }

        if char.is_alphabetic() {}

        match char {
            '\t' => Ok(Some(Token::WhiteSpace(WhiteSpace::HorizontalTabulation))),
            '\n' => Ok(Some(Token::LineTerminator(LineTerminator::LineFeed))),
            '\r' => Ok(Some(Token::LineTerminator(LineTerminator::CarridgeReturn))),
            ' ' => Ok(Some(Token::WhiteSpace(WhiteSpace::Space))),
            '(' => Ok(Some(Token::Punctuation(Punctuation::Parentheses(Parentheses::Left)))),
            ')' => Ok(Some(Token::Punctuation(Punctuation::Parentheses(Parentheses::Right)))),
            '=' => match self.reader.peek(2) {
                Some(second) => match second[1] {
                    '=' => match self.reader.peek(3) {
                        Some(third) => match third[2] {
                            '=' => Ok(Some(Token::Punctuation(Punctuation::Operator(Operator::StrictEquality)))),
                            _ => Ok(Some(Token::Punctuation(Punctuation::Operator(Operator::Equal)))),
                        },
                        None => Err("Unexpected end."),
                    },
                    _ => Ok(Some(Token::Punctuation(Punctuation::Operator(Operator::Assignment)))),
                },
                None => Err("Unexpected end."),
            },
            '*' => match self.reader.peek(2) {
                Some(second) => match second[1] {
                    '*' => match self.reader.peek(3) {
                        Some(third) => match third[2] {
                            '=' => Ok(Some(Token::Punctuation(Punctuation::Operator(Operator::ExponentialAssignment)))),
                            _ => Ok(Some(Token::Punctuation(Punctuation::Operator(Operator::Exponential)))),
                        },
                        None => Err("Unexpected end."),
                    },
                    '=' => Ok(Some(Token::Punctuation(Punctuation::Operator(Operator::MultiplicationAssignment)))),
                    _ => Ok(Some(Token::Punctuation(Punctuation::Operator(Operator::Multiplication)))),
                },
                None => Err("Unexpected end."),
            },
            '+' => self.handle_assignable_operator(Operator::Addition, Operator::AdditonAssignment),
            '-' => self.handle_assignable_operator(Operator::Subtraction, Operator::SubtractionAssignment),
            '/' => self.handle_assignable_operator(Operator::Division, Operator::DivisionAssignment),
            '%' => self.handle_assignable_operator(Operator::Mod, Operator::ModAssignment),
            ';' => Ok(Some(Token::Punctuation(Punctuation::SemiColon))),
            '[' => Ok(Some(Token::Punctuation(Punctuation::Bracket(Bracket::Left)))),
            ']' => Ok(Some(Token::Punctuation(Punctuation::Bracket(Bracket::Right)))),
            '{' => Ok(Some(Token::Punctuation(Punctuation::Brace(Brace::Left)))),
            '}' => Ok(Some(Token::Punctuation(Punctuation::Brace(Brace::Right)))),
            _ => Ok(None),
        }
    }

    fn handle_assignable_operator(&self, operator: Operator, assign: Operator) -> Result<Option<Token>, &str> {
        match self.reader.peek(2) {
            Some(second) => match second[1] {
                '=' => Ok(Some(Token::Punctuation(Punctuation::Operator(assign)))),
                _ => Ok(Some(Token::Punctuation(Punctuation::Operator(operator)))),
            },
            None => Err("Unexpected end."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::{Literal, Operator, Punctuation, Token, WhiteSpace};

    use super::Lexer;

    #[test]
    fn test_peek() {
        let lexer = Lexer::init(" ");
        let res = lexer.peek();
        assert_eq!(Token::WhiteSpace(WhiteSpace::Space), res);

        let lexer = Lexer::init("");
        let res = lexer.peek();
        assert_eq!(Token::Eof, res);

        let lexer = Lexer::init(";");
        let res = lexer.peek();
        assert_eq!(Token::Punctuation(Punctuation::SemiColon), res);

        let lexer = Lexer::init("+= 2");
        let res = lexer.peek();
        assert_eq!(Token::Punctuation(Punctuation::Operator(Operator::AdditonAssignment)), res);

        let lexer = Lexer::init("*= 3");
        let res = lexer.peek();
        assert_eq!(Token::Punctuation(Punctuation::Operator(Operator::MultiplicationAssignment)), res);

        let lexer = Lexer::init("**= 3");
        let res = lexer.peek();
        assert_eq!(Token::Punctuation(Punctuation::Operator(Operator::ExponentialAssignment)), res);

        let lexer = Lexer::init("** 3");
        let res = lexer.peek();
        assert_eq!(Token::Punctuation(Punctuation::Operator(Operator::Exponential)), res);

        let lexer = Lexer::init("356 ");
        let res = lexer.peek();
        assert_eq!(Token::Literal(Literal::Numeric), res);
    }
}

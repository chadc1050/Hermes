use super::reader::Reader;

use super::token::{Brace, Bracket, LineTerminator, Literal, Op, Parentheses, Punc, Token, WhiteSpace};

pub struct Lexer<'a> {
    reader: Reader,
    last: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn init(source: &str) -> Self {
        Lexer { reader: Reader::init(source), last: None }
    }

    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.reader.peek_single() {
                Some(char) => match self.lex(char.clone()) {
                    Ok(token) => tokens.push(token),
                    Err(err) => panic!("Error occurred parsing! Error: {err}"),
                },
                None => {
                    tokens.push(Token::Eof);
                    break;
                }
            }
        }

        tokens
    }

    fn lex(&self, char: char) -> Result<Token, &str> {
        if char.is_digit(10) {
            return self.lex_numeric(char);
        }

        if char.is_alphabetic() {
            return self.lex_alphabetic(char);
        }

        match char {
            '\t' => Ok(Token::WhiteSpace(WhiteSpace::HorizontalTabulation)),
            '\n' => Ok(Token::LineTerminator(LineTerminator::LineFeed)),
            '\r' => Ok(Token::LineTerminator(LineTerminator::CarridgeReturn)),
            ' ' => Ok(Token::WhiteSpace(WhiteSpace::Space)),
            '(' => Ok(Token::Punc(Punc::Parentheses(Parentheses::Left))),
            ')' => Ok(Token::Punc(Punc::Parentheses(Parentheses::Right))),
            '=' => match self.reader.peek(2) {
                Some(second) => match second[1] {
                    '=' => match self.reader.peek(3) {
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
            '*' => match self.reader.peek(2) {
                Some(second) => match second[1] {
                    '*' => match self.reader.peek(3) {
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
            '+' => self.lex_assignable_operator(Op::Addition, Op::AdditonAssign),
            '-' => self.lex_assignable_operator(Op::Subtraction, Op::SubtractionAssign),
            '/' => self.lex_assignable_operator(Op::Division, Op::DivisionAssign),
            '%' => self.lex_assignable_operator(Op::Mod, Op::ModAssign),
            ';' => Ok(Token::Punc(Punc::SemiColon)),
            '[' => Ok(Token::Punc(Punc::Bracket(Bracket::Left))),
            ']' => Ok(Token::Punc(Punc::Bracket(Bracket::Right))),
            '{' => Ok(Token::Punc(Punc::Brace(Brace::Left))),
            '}' => Ok(Token::Punc(Punc::Brace(Brace::Right))),
            _ => Err("Unexpected char"),
        }
    }

    fn lex_alphabetic(&self, char: char) -> Result<Token, &str> {
        let mut idx = 1;
        let alpha = char.to_string();
        loop {
            idx += 1;
            let peek = self.reader.peek(idx);
            match peek {
                Some(word) => match &word.into_iter().collect::<String>() {
                    _ => continue,
                },
                None => return Err("Unexpected end."),
            }
        }
    }

    fn lex_numeric(&self, char: char) -> Result<Token, &str> {
        let val = char.to_string();
        let mut idx = 1;
        loop {
            idx += 1;
            match self.reader.peek(idx) {
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

    fn lex_assignable_operator(&self, operator: Op, assign: Op) -> Result<Token, &str> {
        // Check keywords
        match self.reader.peek(2) {
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
        let lexer = Lexer::init(" ");
        let res = lexer.tokenize();
        assert_eq!(Token::WhiteSpace(WhiteSpace::Space), res[0]);

        let lexer = Lexer::init("");
        let res = lexer.tokenize();
        assert_eq!(Token::Eof, res[0]);

        let lexer = Lexer::init(";");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::SemiColon), res[0]);

        let lexer = Lexer::init("+= 2");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::AdditonAssign)), res[0]);

        let lexer = Lexer::init("*= 3");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::MultiplicationAssign)), res[0]);

        let lexer = Lexer::init("**= 3");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::ExponentialAssign)), res[0]);

        let lexer = Lexer::init("** 3");
        let res = lexer.tokenize();
        assert_eq!(Token::Punc(Punc::Op(Op::Exponential)), res[0]);

        let lexer = Lexer::init("356 ");
        let res = lexer.tokenize();
        assert_eq!(Token::Literal(Literal::Numeric(356)), res[0]);
    }
}

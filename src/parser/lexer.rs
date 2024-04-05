use super::reader::Reader;

use super::token::{Brace, Bracket, LineTerminator, Parentheses, Punctuation, Token, WhiteSpace};

pub struct Lexer<'a> {
    reader: Reader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn init(source: &'a str) -> Self {
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

    fn lex(&self, buffer: &[u8]) -> Result<Option<Token>, &str> {
        if buffer.len() == 1 {
            let char = buffer[0];
            match char {
                0x07 => Ok(Some(Token::WhiteSpace(WhiteSpace::LineTabulation))),
                0x09 => Ok(Some(Token::WhiteSpace(WhiteSpace::CharacterTabulation))),
                0x0A => Ok(Some(Token::LineTerminator(LineTerminator::LineFeed))),
                0x0C => Ok(Some(Token::WhiteSpace(WhiteSpace::FormFeed))),
                0x0D => Ok(Some(Token::LineTerminator(LineTerminator::CarridgeReturn))),
                0x20 => Ok(Some(Token::WhiteSpace(WhiteSpace::Space))),
                0xA0 => Ok(Some(Token::WhiteSpace(WhiteSpace::NoBreakSpace))),
                0x28 => Ok(Some(Token::Punctuation(Punctuation::Parentheses(Parentheses::Left)))),
                0x29 => Ok(Some(Token::Punctuation(Punctuation::Parentheses(Parentheses::Right)))),
                0x3B => Ok(Some(Token::Punctuation(Punctuation::SemiColon))),
                0x5B => Ok(Some(Token::Punctuation(Punctuation::Bracket(Bracket::Left)))),
                0x5D => Ok(Some(Token::Punctuation(Punctuation::Bracket(Bracket::Right)))),
                0x7B => Ok(Some(Token::Punctuation(Punctuation::Brace(Brace::Left)))),
                0x7D => Ok(Some(Token::Punctuation(Punctuation::Brace(Brace::Right)))),
                _ => Ok(None),
            }
        } else if buffer.len() == 2 {
            let chars = String::from_utf8(buffer.to_vec());
            match &chars {
                _ => Ok(None),
            }
        } else {
            Err("Unimplemented")
        }
    }
}

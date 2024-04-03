use super::reader::Reader;

use super::token::{Token, WhiteSpace};

pub struct Lexer<'a> {
    reader: Reader<'a>,
}

impl<'a> Lexer<'a> {
    pub fn init(source: &'a str) -> Self {
        Lexer {
            reader: Reader::init(source),
        }
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

    fn lex(&self, chars: &[u8]) -> Result<Option<Token>, &str> {
        if chars.len() == 1 {
            let char = chars[1];
            match char {
                0x007 => Ok(Some(Token::WhiteSpace(WhiteSpace::Tab))),
                0x00C => Ok(Some(Token::WhiteSpace(WhiteSpace::FormFeed))),
                0x00D => Ok(Some(Token::WhiteSpace(WhiteSpace::CarridgeReturn))),
                0x020 => Ok(Some(Token::WhiteSpace(WhiteSpace::Space))),
                _ => Ok(None),
            }
        } else {
            Err("Unimplemented")
        }
    }
}

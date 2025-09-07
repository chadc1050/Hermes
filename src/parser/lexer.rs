use std::cell::RefCell;
use std::rc::Rc;

use super::reader::Reader;

use super::token::{is_removable, map_keyword, BooleanKind, BraceKind, BracketKind, LineTerminatorKind, LitKind, OpKind, ParenthesesKind, PuncKind, Token, TokenKind, WhiteSpaceKind};

pub struct Lexer {
    reader: Rc<RefCell<Reader<char>>>,
}

impl Lexer {
    pub fn init(source: &str) -> Self {
        Lexer { reader: Rc::new(RefCell::new(Reader::init(source.chars().collect::<Vec<char>>()))) }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.lex() {
                Ok(token) => {
                    if !is_removable(&token.kind) {
                        tokens.push(token.clone());
                    }
                    if token.kind == TokenKind::Eof {
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
        let pos = reader.get_pos();
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
                    '\t' => Ok(Token::new(TokenKind::WhiteSpace(WhiteSpaceKind::HorizontalTabulation), pos)),
                    '\n' => Ok(Token::new(TokenKind::LineTerminator(LineTerminatorKind::LineFeed), pos)),
                    '\r' => Ok(Token::new(TokenKind::LineTerminator(LineTerminatorKind::CarriageReturn), pos)),
                    ' ' => Ok(Token::new(TokenKind::WhiteSpace(WhiteSpaceKind::Space), pos)),
                    '(' => Ok(Token::new(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)), pos)),
                    ')' => Ok(Token::new(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)), pos)),
                    '=' => match reader.peek_single() {
                        Some(second) => match second {
                            '=' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::StrictEquality)), pos))
                                        }
                                        _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Equal)), pos)),
                                    },
                                    None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Equal)), pos)),
                                }
                            }
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Assign)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Assign)), pos)),
                    },
                    '*' => match reader.peek_single() {
                        Some(second) => match second {
                            '*' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::ExponentialAssign)), pos))
                                        }
                                        _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Exponential)), pos)),
                                    },
                                    None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Exponential)), pos)),
                                }
                            }
                            '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::MultiplicationAssign)), pos)),
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Multiplication)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Multiplication)), pos)),
                    },
                    '&' => match reader.peek_single() {
                        Some(second) => match second {
                            '&' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::AndAssign)), pos))
                                        }
                                        _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::And)), pos)),
                                    },
                                    None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::And)), pos)),
                                }
                            }
                            '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::BitAndAssing)), pos)),
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::BitAnd)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::BitAnd)), pos)),
                    },
                    '|' => match reader.peek_single() {
                        Some(second) => match second {
                            '|' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::OrAssign)), pos))
                                        }
                                        _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Or)), pos)),
                                    },
                                    None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Or)), pos)),
                                }
                            }
                            '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::BitOrAssign)), pos)),
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::BitOr)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::BitOr)), pos)),
                    },
                    '^' => self.lex_assignable_operator(&mut reader, OpKind::BitXor, OpKind::BitXorAssign),
                    '+' => match reader.peek_single() {
                        Some(second) => match second {
                            '+' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Increment)), pos)),
                            '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::AdditonAssign)), pos)),
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Addition)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Addition)), pos)),
                    },
                    '-' => match reader.peek_single() {
                        Some(second) => match second {
                            '-' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Decrement)), pos)),
                            '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::SubtractionAssign)), pos)),
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Subtraction)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Subtraction)), pos)),
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
                                                        Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::UnsignedRightShiftAssign)), pos))
                                                    }
                                                    _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::ZeroFillRightShift)), pos)),
                                                },
                                                None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::ZeroFillRightShift)), pos)),
                                            }
                                        }
                                        '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::RightShiftAssign)), pos)),
                                        _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::RightShift)), pos)),
                                    },
                                    None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::RightShift)), pos)),
                                }
                            }
                            '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::GreaterThanEqual)), pos)),
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::GreaterThan)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::GreaterThan)), pos)),
                    },
                    '<' => match reader.peek_single() {
                        Some(second) => match second {
                            '<' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::LeftShiftAssign)), pos))
                                        }
                                        _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::LeftShift)), pos)),
                                    },
                                    None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::LeftShift)), pos)),
                                }
                            }
                            '=' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::LessThanEqual)), pos)),
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::LessThan)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::LessThan)), pos)),
                    },
                    '!' => match reader.peek_single() {
                        Some(second) => match second {
                            '=' => {
                                reader.bump();
                                Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::NotEqual)), pos))
                            }
                            _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Not)), pos)),
                        },
                        None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::Not)), pos)),
                    },
                    '?' => match reader.peek_single() {
                        Some(second) => match second {
                            '?' => {
                                reader.bump();
                                match reader.peek_single() {
                                    Some(third) => match third {
                                        '=' => {
                                            reader.bump();
                                            Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::NullishCoalescingAssign)), pos))
                                        }
                                        _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::NullishCoalescing)), pos)),
                                    },
                                    None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::OptionalChain)), pos)),
                                }
                            }
                            '.' => Ok(Token::new(TokenKind::Punc(PuncKind::Op(OpKind::OptionalChain)), pos)),
                            _ => Err("Invalid Token"),
                        },
                        None => Err("Invalid Token"),
                    },
                    '.' => Ok(Token::new(TokenKind::Punc(PuncKind::Dot), pos)),
                    ';' => Ok(Token::new(TokenKind::Punc(PuncKind::SemiColon), pos)),
                    '[' => Ok(Token::new(TokenKind::Punc(PuncKind::Bracket(BracketKind::Left)), pos)),
                    ']' => Ok(Token::new(TokenKind::Punc(PuncKind::Bracket(BracketKind::Right)), pos)),
                    '{' => Ok(Token::new(TokenKind::Punc(PuncKind::Brace(BraceKind::Left)), pos)),
                    '}' => Ok(Token::new(TokenKind::Punc(PuncKind::Brace(BraceKind::Right)), pos)),
                    _ => Ok(Token::new(TokenKind::Unicode(first.to_string()), pos)),
                }
            }
            None => Ok(Token::new(TokenKind::Eof, pos)),
        }
    }

    /// Handles alphabetic tokens encapsulated by
    fn lex_string_literal(&self, reader: &mut Reader<char>) -> Result<Token, &str> {
        let mut word = String::new();
        let pos = reader.get_pos();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_alphabetic() {
                        reader.bump();
                        word.push(peek);
                    } else if peek == '"' {
                        reader.bump();
                        return Ok(Token::new(TokenKind::Lit(LitKind::String(word)), pos));
                    }
                }
                None => {
                    return Err("Unexpected end of string literal!");
                }
            }
        }
    }

    /// Handles all alphabetic tokens not encapsulated by quotations (non-string literals)
    fn lex_identifier(&self, reader: &mut Reader<char>, char: char) -> Result<Token, &str> {
        let mut word = char.to_string();
        let pos = reader.get_pos();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_alphabetic() {
                        word.push(peek);
                        reader.bump();
                    } else {
                        return if let Some(keyword) = map_keyword(&word) {
                            Ok(Token::new(TokenKind::Keyword(keyword), pos))
                        } else if word == "true" {
                            Ok(Token::new(TokenKind::Lit(LitKind::Bool(BooleanKind::True)), pos))
                        } else if word == "false" {
                            Ok(Token::new(TokenKind::Lit(LitKind::Bool(BooleanKind::False)), pos))
                        } else {
                            Ok(Token::new(TokenKind::Id(word), pos))
                        }
                    }
                }
                None => {
                    return if let Some(keyword) = map_keyword(&word) {
                        Ok(Token::new(TokenKind::Keyword(keyword), pos))
                    } else if word == "true" {
                        Ok(Token::new(TokenKind::Lit(LitKind::Bool(BooleanKind::True)), pos))
                    } else if word == "false" {
                        Ok(Token::new(TokenKind::Lit(LitKind::Bool(BooleanKind::False)), pos))
                    } else {
                        Ok(Token::new(TokenKind::Id(word), pos))
                    }
                }
            }
        }
    }

    /// Given a numeric character, parses the rest of the numeric and determines numeric variant.
    /// TODO: Need to check for decimals and non-decimal number types.
    fn lex_numeric(&self, reader: &mut Reader<char>, char: char) -> Result<Token, &str> {
        let mut val = char.to_string();
        let pos = reader.get_pos();
        loop {
            match reader.peek_single() {
                Some(peek) => {
                    if peek.is_digit(10) {
                        val.push(peek);
                        reader.bump();
                    } else {
                        return Ok(Token::new(TokenKind::Lit(LitKind::Num(val.parse().unwrap())), pos));
                    }
                }
                None => return Ok(Token::new(TokenKind::Lit(LitKind::Num(val.parse().unwrap())), pos)),
            }
        }
    }

    fn lex_assignable_operator(&self, reader: &mut Reader<char>, operator: OpKind, assign: OpKind) -> Result<Token, &str> {
        let pos = reader.get_pos();
        match reader.peek_single() {
            Some(second) => match second {
                '=' => {
                    reader.bump();
                    Ok(Token::new(TokenKind::Punc(PuncKind::Op(assign)), pos))
                }
                _ => Ok(Token::new(TokenKind::Punc(PuncKind::Op(operator)), pos)),
            },
            None => Ok(Token::new(TokenKind::Punc(PuncKind::Op(operator)), pos)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::{BooleanKind, KeywordKind, LitKind, OpKind, ParenthesesKind, PuncKind, TokenKind};

    use super::Lexer;

    #[test]
    fn test_tokenize() {
        let mut lexer = Lexer::init("testing 123");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Id("testing".into()), res[0].kind);
        assert_eq!(TokenKind::Lit(LitKind::Num(123)), res[1].kind);
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::init(" ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Eof, res[0].kind);

        let mut lexer = Lexer::init("");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Eof, res[0].kind);
    }

    #[test]
    fn test_punctuation() {
        let mut lexer = Lexer::init(";");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::SemiColon), res[0].kind);
    }

    #[test]
    fn test_boolean() {
        let mut lexer = Lexer::init("true false");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Lit(LitKind::Bool(BooleanKind::True)), res[0].kind);
        assert_eq!(TokenKind::Lit(LitKind::Bool(BooleanKind::False)), res[1].kind);
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::init("\"true\"");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Lit(LitKind::String("true".into())), res[0].kind);
    }

    #[test]
    fn test_string_identifier() {
        let mut lexer = Lexer::init("let test = new Tokenizer(\"debugger\");");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Keyword(KeywordKind::Let), res[0].kind);
        assert_eq!(TokenKind::Id("test".into()), res[1].kind);
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::Assign)), res[2].kind);
        assert_eq!(TokenKind::Keyword(KeywordKind::New), res[3].kind);
        assert_eq!(TokenKind::Id("Tokenizer".into()), res[4].kind);
        assert_eq!(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)), res[5].kind);
        assert_eq!(TokenKind::Lit(LitKind::String("debugger".into())), res[6].kind);
        assert_eq!(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)), res[7].kind);
        assert_eq!(TokenKind::Punc(PuncKind::SemiColon), res[8].kind);
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::init("+= ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::AdditonAssign)), res[0].kind);

        let mut lexer = Lexer::init("*= 3");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::MultiplicationAssign)), res[0].kind);

        let mut lexer = Lexer::init("**= 3");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::ExponentialAssign)), res[0].kind);

        let mut lexer = Lexer::init("** 3");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::Exponential)), res[0].kind);

        let mut lexer = Lexer::init("& ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::BitAnd)), res[0].kind);

        let mut lexer = Lexer::init("&&= ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::AndAssign)), res[0].kind);
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::init("await yield");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Keyword(KeywordKind::Await), res[0].kind);
        assert_eq!(TokenKind::Keyword(KeywordKind::Yield), res[1].kind);

        let mut lexer = Lexer::init("let x = await y;");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Keyword(KeywordKind::Let), res[0].kind);
        assert_eq!(TokenKind::Id("x".into()), res[1].kind);
        assert_eq!(TokenKind::Punc(PuncKind::Op(OpKind::Assign)), res[2].kind);
        assert_eq!(TokenKind::Keyword(KeywordKind::Await), res[3].kind);
        assert_eq!(TokenKind::Id("y".into()), res[4].kind);
    }

    #[test]
    fn test_numerics() {
        let mut lexer = Lexer::init("356 ");
        let res = lexer.tokenize();
        assert_eq!(TokenKind::Lit(LitKind::Num(356)), res[0].kind);
    }
}

use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::parser::ast::{AdditiveExpression, ExpressionKind, LetDeclaration, Node, NodeKind, PrimaryExpressionKind, AST};
use crate::parser::ast::DeclarationKind::Lexical;
use crate::parser::ast::LexicalKind::Let;
use crate::parser::ast::PrimaryExpressionKind::{Identifier, Literal, RegExLiteral};
use crate::parser::reader::Reader;
use crate::parser::token::{KeywordKind, LiteralKind, OpKind, PuncKind, TokenKind};
use self::lexer::Lexer;

mod reader;

mod lexer;

mod token;
mod ast;

#[derive(Debug)]
pub enum ParseErrorKind {
    UnexpectedToken,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
}

/// Parses source code to AST based on [ECMAScript Lexical Grammar](https://262.ecma-international.org/#sec-intro).
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn init(source: &str) -> Self {
        Parser { lexer: Lexer::init(source) }
    }

    pub fn parse(&mut self) -> Result<AST, ParseError> {
        let tokens = self.lexer.tokenize();

        let ts = Rc::new(RefCell::new(Reader::init(tokens)));

        let mut ast = AST::new();

        let mut in_statement = false;
        let curr_node: &mut Node = ast.get_root();

        loop {
            let mut ref_ts = ts.borrow_mut();
            match ref_ts.peek_single() {
                Some(token) => {
                    match token {
                        TokenKind::Eof => {
                            break;
                        }
                        TokenKind::Identifier(_) => {}
                        TokenKind::Keyword(k) => {
                            match k {
                                KeywordKind::Let | KeywordKind::Const => {
                                    if in_statement {
                                        return Err(ParseError{kind: ParseErrorKind::UnexpectedToken })
                                    }

                                    let assign = ref_ts.next(3).ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

                                    let id;
                                    match assign[1].clone() {
                                        TokenKind::Identifier(val) => {id = val }
                                        _ => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
                                    }

                                    if assign[2] != TokenKind::Punc(PuncKind::Op(OpKind::Assign)) {
                                        return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
                                    }

                                    match Parser::parse_expression(ref_ts) {
                                        Some(expr) => {
                                            let let_decl = LetDeclaration{ identifier: id, expression: expr };
                                            curr_node.add_child(Node::new(NodeKind::Declaration(Lexical(Let(let_decl)))));
                                        }
                                        None => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
                                    }
                                }
                                _ => {}
                            }
                        }
                        TokenKind::LineTerminator(_) => {
                            ref_ts.bump();
                            in_statement = false;
                        }
                        TokenKind::Literal(_) => {}
                        TokenKind::Punc(punc) => {
                            match punc {
                                PuncKind::SemiColon => {
                                    ref_ts.bump();
                                    in_statement = false;
                                }
                                _ => todo!()
                            }
                        }
                        TokenKind::Unicode(_) => {}
                        _ => return Err(ParseError{kind: ParseErrorKind::UnexpectedToken}),
                    }
                },
                None => {
                    break;
                }
            }
        }

        Ok(ast)
    }

    fn parse_expression(mut ts: RefMut<Reader<TokenKind>>) -> Option<ExpressionKind> {
        let first = ts.next_single();
        match first {
            Some(token) => match token {
                TokenKind::Identifier(first_id) => {
                    let second = ts.next_single();
                    match second {
                        Some(second_token) => {
                            match second_token {
                                TokenKind::Identifier(_) => todo!(),
                                TokenKind::Keyword(_) => todo!(),
                                TokenKind::Literal(_) => todo!(),
                                TokenKind::Punc(punc) => {
                                    match punc {
                                        PuncKind::Brace(_) => todo!(),
                                        PuncKind::Bracket(_) => todo!(),
                                        PuncKind::Dot => todo!(),
                                        PuncKind::Op(op) => {
                                            match op {
                                                OpKind::Addition => {
                                                    let third = ts.next_single();
                                                    match third {
                                                        Some(third_token) => {
                                                            match third_token {
                                                                TokenKind::Identifier(second_id) => {
                                                                    Some(ExpressionKind::Additive(Box::new(AdditiveExpression{lhs: ExpressionKind::Primary(Identifier(first_id)), rhs: ExpressionKind::Primary(Identifier(second_id))})))
                                                                },
                                                                TokenKind::Keyword(_) => todo!(),
                                                                TokenKind::LineTerminator(_) => todo!(),
                                                                TokenKind::Literal(_) => todo!(),
                                                                TokenKind::Punc(_) => todo!(),
                                                                _ => None
                                                            }
                                                        }
                                                        None => None
                                                    }
                                                }
                                                OpKind::Multiplication => todo!(),
                                                _ => todo!()
                                            }
                                        }
                                        PuncKind::Parentheses(_) => todo!(),
                                        PuncKind::SemiColon => todo!(),
                                    }
                                }
                                _ => None
                            }
                        }
                        None => None
                    }
                }
                TokenKind::Keyword(kw) => match kw {
                    KeywordKind::This => {
                        Some(ExpressionKind::Primary(PrimaryExpressionKind::This))
                    }
                    _ => None
                }
                TokenKind::Literal(literal) => {
                    ts.bump();
                    match literal {
                        LiteralKind::BigIntSuffix(bis) => Some(ExpressionKind::Primary(Literal(LiteralKind::BigIntSuffix(bis)))),
                        LiteralKind::Boolean(bool) => Some(ExpressionKind::Primary(Literal(LiteralKind::Boolean(bool)))),
                        LiteralKind::Decimal(dec) => Some(ExpressionKind::Primary(Literal(LiteralKind::Decimal(dec)))),
                        LiteralKind::DecimalBigInteger(dbi) => Some(ExpressionKind::Primary(Literal(LiteralKind::DecimalBigInteger(dbi)))),
                        LiteralKind::DecimalInteger(di) => Some(ExpressionKind::Primary(Literal(LiteralKind::DecimalInteger(di)))),
                        LiteralKind::NonDecimalInteger(ndi) => Some(ExpressionKind::Primary(Literal(LiteralKind::NonDecimalInteger(ndi)))),
                        LiteralKind::Null => Some(ExpressionKind::Primary(Literal(LiteralKind::Null))),
                        LiteralKind::Numeric(num) => Some(ExpressionKind::Primary(Literal(LiteralKind::Numeric(num)))),
                        LiteralKind::StringLiteral(str) => Some(ExpressionKind::Primary(Literal(LiteralKind::StringLiteral(str)))),
                        LiteralKind::RegEx(regex) => Some(ExpressionKind::Primary(RegExLiteral(regex)))
                    }
                }
                TokenKind::Punc(_) => todo!(),
                _ => None
            }
            None => {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use super::ast::NodeKind;

    #[test]
    fn test_parser() {
        let mut parser = Parser::init("let five = 5;\n let six = 6\n let added = five + six");

        let res = parser.parse();

        assert!(res.is_ok());

        let mut ast = res.unwrap();

        let root = ast.get_root();

        assert_eq!(root.node_kind, NodeKind::Module);

        assert_eq!(root.get_children().len(), 3);
    }
}
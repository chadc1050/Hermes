use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::parser::ast::{AdditiveExpression, ConstDeclaration, DeclarationKind, ExpressionKind, IfStatement, LetDeclaration, LexicalKind, Node, NodeKind, PrimaryExpressionKind, StatementKind, AST};
use crate::parser::ast::DeclarationKind::Lexical;
use crate::parser::ast::LexicalKind::{Const, Let};
use crate::parser::ast::PrimaryExpressionKind::{Identifier, Literal, RegExLiteral};
use crate::parser::reader::Reader;
use crate::parser::token::{KeywordKind, LiteralKind, OpKind, ParenthesesKind, PuncKind, TokenKind};
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
                                KeywordKind::Const => curr_node.add_child(Node::new(NodeKind::Declaration(Lexical(Const(Self::parse_const_decl(ref_ts)?))))),
                                KeywordKind::Let => curr_node.add_child(Node::new(NodeKind::Declaration(Lexical(Let(Self::parse_let_decl(ref_ts)?))))),
                                KeywordKind::If => curr_node.add_child(Node::new(NodeKind::Statement(StatementKind::If(Self::parse_if_statement(ref_ts)?)))),
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

    fn parse_let_decl(mut ts: RefMut<Reader<TokenKind>>) -> Result<LetDeclaration, ParseError> {

        let statement = ts.next(3).ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        let id;
        match statement[1].clone() {
            TokenKind::Identifier(val) => {id = val }
            _ => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if statement[2] != TokenKind::Punc(PuncKind::Op(OpKind::Assign)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        match Parser::parse_expression(ts) {
            Some(expr) => Ok(LetDeclaration{ identifier: id, expression: expr }),
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }

    fn parse_const_decl(mut ts: RefMut<Reader<TokenKind>>) -> Result<ConstDeclaration, ParseError> {

        let statement = ts.next(3).ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        let id;
        match statement[1].clone() {
            TokenKind::Identifier(val) => {id = val }
            _ => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if statement[2] != TokenKind::Punc(PuncKind::Op(OpKind::Assign)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        match Parser::parse_expression(ts) {
            Some(expr) => Ok(ConstDeclaration{ identifier: id, expression: expr }),
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }

    fn parse_if_statement(mut ts: RefMut<Reader<TokenKind>>) -> Result<IfStatement, ParseError> {
        let open = ts.next_single().ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;
        if open != TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        let sub_ts = RefCell::new(
            Reader::init(
                ts.collect_until(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)))
            )
        );

        let expr = Self::parse_expression(sub_ts.borrow_mut());

        match expr {
            Some(expr) => {
                let close = ts.next_single().ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;
                if close != TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)) {
                    return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
                }
                ts.bump();

                Ok(IfStatement { condition: expr, body: vec![] })
            }
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use super::ast::NodeKind;

    #[test]
    fn test_decl() {
        let mut parser = Parser::init("let five = 5;\n let six = 6\n let added = five + six");
        let res = parser.parse();
        assert!(res.is_ok());
        let mut ast = res.unwrap();
        let root = ast.get_root();
        assert_eq!(root.node_kind, NodeKind::Module);
        assert_eq!(root.get_children().len(), 3);
    }

    #[test]
    fn test_if() {
        let mut parser = Parser::init("let y = 5;\nif (y == 5) {\nreturn;\n}");
        let res = parser.parse();
        assert!(res.is_ok());
        let mut ast = res.unwrap();
        let root = ast.get_root();
    }
}
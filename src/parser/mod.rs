use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::parser::ast::{AdditiveExpression, BlockStatement, ConstDeclaration, DeclarationKind, ExpressionKind, IfStatement, LetDeclaration, LexicalKind, Node, NodeKind, PrimaryExpressionKind, StatementKind, AST};
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
    ts: Rc<RefCell<Reader<TokenKind>>>
}

impl Parser {
    pub fn init(source: &str) -> Self {
        let tokens = Lexer::init(source).tokenize();
        let ts = Rc::new(RefCell::new(Reader::init(tokens)));
        Parser { ts }
    }

    pub fn parse(&mut self, file_name: String) -> Result<AST, ParseError> {

        let mut ast = AST::new(file_name);

        let mut in_statement = false;
        let curr_node: &mut Node = ast.get_root();

        loop {
            let initial_token = self.ts.borrow().peek_single();

            match initial_token {
                Some(token) => {
                    match token {
                        TokenKind::Eof => {
                            break;
                        }
                        TokenKind::Identifier(_) => {}
                        TokenKind::Keyword(k) => {
                            match k {
                                KeywordKind::Const => curr_node.add_child(Node::new(NodeKind::Declaration(Lexical(Const(self.parse_const_decl()?))))),
                                KeywordKind::Let => curr_node.add_child(Node::new(NodeKind::Declaration(Lexical(Let(self.parse_let_decl()?))))),
                                KeywordKind::If => curr_node.add_child(Node::new(NodeKind::Statement(StatementKind::If(self.parse_if_statement()?)))),
                                _ => {}
                            }
                        }
                        TokenKind::LineTerminator(_) => {
                            self.ts.borrow_mut().bump();
                            in_statement = false;
                        }
                        TokenKind::Literal(_) => {}
                        TokenKind::Punc(punc) => {
                            match punc {
                                PuncKind::SemiColon => {
                                    self.ts.borrow_mut().bump();
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

    fn parse_block(&mut self) -> Result<BlockStatement, ParseError> {
        Ok(BlockStatement{statements: vec![]})
    }

    fn parse_expression(&mut self) -> Option<ExpressionKind> {
        let mut ref_ts = self.ts.borrow_mut();
        let first = ref_ts.next_single();
        match first {
            Some(token) => match token {
                TokenKind::Identifier(first_id) => {
                    let second = ref_ts.next_single();
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
                                                    let third = ref_ts.next_single();
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
                    ref_ts.bump();
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

    fn parse_let_decl(&mut self) -> Result<LetDeclaration, ParseError> {

        let statement = self.ts.borrow_mut()
            .next(3)
            .ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        let id;
        match statement[1].clone() {
            TokenKind::Identifier(val) => {id = val }
            _ => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if statement[2] != TokenKind::Punc(PuncKind::Op(OpKind::Assign)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        match self.parse_expression() {
            Some(expr) => Ok(LetDeclaration{ identifier: id, expression: expr }),
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }

    fn parse_const_decl(&mut self) -> Result<ConstDeclaration, ParseError> {

        let statement = self.ts.borrow_mut().next(3).ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        let id;
        match statement[1].clone() {
            TokenKind::Identifier(val) => {id = val }
            _ => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if statement[2] != TokenKind::Punc(PuncKind::Op(OpKind::Assign)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        match self.parse_expression() {
            Some(expr) => Ok(ConstDeclaration{ identifier: id, expression: expr }),
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement, ParseError> {
        let if_start = self.ts.borrow_mut().next(2).ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        if if_start[0] != TokenKind::Keyword(KeywordKind::If) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if if_start[1] != TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        let expr = self.parse_expression();

        match expr {
            Some(expr) => {
                let close = self.ts.borrow_mut().next_single().ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;
                if close != TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)) {
                    return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
                }
                self.ts.borrow_mut().bump();

                Ok(IfStatement { condition: expr, body: self.parse_block()? })
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
        let module = "Test";
        let mut parser = Parser::init("let five = 5;\n let six = 6\n let added = five + six");
        let res = parser.parse(module.into());
        assert!(res.is_ok());
        let mut ast = res.unwrap();
        let root = ast.get_root();
        assert_eq!(root.node_kind, NodeKind::Module(module.into()));
        assert_eq!(root.get_children().len(), 3);
    }

    #[test]
    fn test_if() {
        let module = "Test";
        let mut parser = Parser::init("let y = 5;\nif (y == 5) {\nreturn;\n}");
        let res = parser.parse(module.into());
        assert!(res.is_ok());
        let mut ast = res.unwrap();
        let root = ast.get_root();
    }
}
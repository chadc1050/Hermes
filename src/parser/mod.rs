use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::parser::ast::{AdditiveExpr, BlockStmt, ConstDecl, ExprKind, IfStmt, LetDecl, Node, NodeKind, PrimaryExprKind, StmtKind, AST};
use crate::parser::ast::DeclKind::Lexical;
use crate::parser::ast::LexicalKind::{Const, Let};
use crate::parser::ast::PrimaryExprKind::{Id, Lit, RegExLiteral};
use crate::parser::reader::Reader;
use crate::parser::token::{KeywordKind, LitKind, OpKind, ParenthesesKind, PuncKind, Token, TokenKind};
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
    ts: Rc<RefCell<Reader<Token>>>
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

        let mut curr_node: &mut Node = ast.get_root();

        curr_node.add_child(Node::new(NodeKind::Stmt(StmtKind::Block)));

        curr_node = curr_node.get_children_mut().first_mut().unwrap();

        loop {
            let initial_token = self.ts.borrow().peek_single();

            match initial_token {
                Some(token) => {
                    match token.kind {
                        TokenKind::Eof => {
                            break;
                        }
                        TokenKind::Id(_) => {}
                        TokenKind::Keyword(k) => {
                            match k {
                                KeywordKind::Const => curr_node.add_child(Node::new(NodeKind::Stmt(StmtKind::Decl(Lexical(Const(self.parse_const_decl()?)))))),
                                KeywordKind::Let => curr_node.add_child(Node::new(NodeKind::Stmt(StmtKind::Decl(Lexical(Let(self.parse_let_decl()?)))))),
                                KeywordKind::If => curr_node.add_child(Node::new(NodeKind::Stmt(StmtKind::If(self.parse_if_statement()?)))),
                                _ => {}
                            }
                        }
                        TokenKind::LineTerminator(_) => {
                            self.ts.borrow_mut().bump();
                            in_statement = false;
                        }
                        TokenKind::Lit(_) => {}
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

    fn parse_block(&mut self) -> Result<BlockStmt, ParseError> {
        Ok(BlockStmt { stmts: vec![]})
    }

    #[inline]
    fn parse_expression(&mut self) -> Option<ExprKind> {
        let mut ref_ts = self.ts.borrow_mut();
        let first = ref_ts.next_single();
        match first {
            Some(token) => match token.kind {
                TokenKind::Id(first_id) => {
                    let second = ref_ts.next_single();
                    match second {
                        Some(second_token) => {
                            match second_token.kind {
                                TokenKind::Id(_) => todo!(),
                                TokenKind::Keyword(_) => todo!(),
                                TokenKind::Lit(_) => todo!(),
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
                                                            match third_token.kind {
                                                                TokenKind::Id(second_id) => {
                                                                    Some(ExprKind::Additive(Box::new(AdditiveExpr {lhs: ExprKind::Primary(Id(first_id)), rhs: ExprKind::Primary(Id(second_id))})))
                                                                },
                                                                TokenKind::Keyword(_) => todo!(),
                                                                TokenKind::LineTerminator(_) => todo!(),
                                                                TokenKind::Lit(_) => todo!(),
                                                                TokenKind::Punc(_) => todo!(),
                                                                _ => None
                                                            }
                                                        }
                                                        None => None
                                                    }
                                                }
                                                OpKind::Multiplication => todo!(),
                                                OpKind::Equal => {
                                                    let third = ref_ts.next_single();
                                                    match third {
                                                        Some(third_token) => match third_token.kind {
                                                            TokenKind::Id(_) => todo!(),
                                                            TokenKind::Keyword(_) => todo!(),
                                                            TokenKind::Lit(lit) => {
                                                                Some(todo!())
                                                            }
                                                            TokenKind::Punc(_) => todo!(),
                                                            _ => None
                                                        },
                                                        None => None
                                                    }
                                                }
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
                        Some(ExprKind::Primary(PrimaryExprKind::This))
                    }
                    _ => None
                }
                TokenKind::Lit(literal) => {
                    ref_ts.bump();
                    match literal {
                        LitKind::BigIntSuffix(bis) => Some(ExprKind::Primary(Lit(LitKind::BigIntSuffix(bis)))),
                        LitKind::Bool(bool) => Some(ExprKind::Primary(Lit(LitKind::Bool(bool)))),
                        LitKind::Dec(dec) => Some(ExprKind::Primary(Lit(LitKind::Dec(dec)))),
                        LitKind::DecimalBigInteger(dbi) => Some(ExprKind::Primary(Lit(LitKind::DecimalBigInteger(dbi)))),
                        LitKind::DecimalInteger(di) => Some(ExprKind::Primary(Lit(LitKind::DecimalInteger(di)))),
                        LitKind::NonDecimalInteger(ndi) => Some(ExprKind::Primary(Lit(LitKind::NonDecimalInteger(ndi)))),
                        LitKind::Null => Some(ExprKind::Primary(Lit(LitKind::Null))),
                        LitKind::Num(num) => Some(ExprKind::Primary(Lit(LitKind::Num(num)))),
                        LitKind::String(str) => Some(ExprKind::Primary(Lit(LitKind::String(str)))),
                        LitKind::RegEx(regex) => Some(ExprKind::Primary(RegExLiteral(regex)))
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

    #[inline]
    fn parse_let_decl(&mut self) -> Result<LetDecl, ParseError> {

        let statement = self.ts.borrow_mut()
            .next(3)
            .ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        let id;
        match statement[1].clone().kind {
            TokenKind::Id(val) => {id = val }
            _ => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if statement[2].kind != TokenKind::Punc(PuncKind::Op(OpKind::Assign)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        match self.parse_expression() {
            Some(expr) => Ok(LetDecl { identifier: id, expression: expr }),
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }

    #[inline]
    fn parse_const_decl(&mut self) -> Result<ConstDecl, ParseError> {

        let statement = self.ts.borrow_mut()
            .next(3)
            .ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        let id;
        match statement[1].clone().kind {
            TokenKind::Id(val) => {id = val }
            _ => return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if statement[2].kind != TokenKind::Punc(PuncKind::Op(OpKind::Assign)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        match self.parse_expression() {
            Some(expr) => Ok(ConstDecl { identifier: id, expression: expr }),
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }

    #[inline]
    fn parse_if_statement(&mut self) -> Result<IfStmt, ParseError> {
        let if_start = self.ts.borrow_mut().next(2).ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;

        if if_start[0].kind != TokenKind::Keyword(KeywordKind::If) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        if if_start[1].kind != TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)) {
            return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }

        let expr = self.parse_expression();

        match expr {
            Some(expr) => {
                let close = self.ts.borrow_mut().next_single().ok_or(ParseError { kind: ParseErrorKind::UnexpectedToken })?;
                if close.kind != TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)) {
                    return Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
                }
                self.ts.borrow_mut().bump();

                Ok(IfStmt { cond: expr, body: self.parse_block()? })
            }
            None => Err(ParseError { kind: ParseErrorKind::UnexpectedToken })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::LitKind;
    use super::Parser;
    use super::ast::{AdditiveExpr, DeclKind, ExprKind, LetDecl, LexicalKind, NodeKind, PrimaryExprKind, StmtKind};

    #[test]
    fn test_decl() {
        let module = "Test";
        let mut parser = Parser::init("let five = 5;\n let six = 6\n let added = five + six");
        let res = parser.parse(module.into());
        assert!(res.is_ok());
        let mut ast = res.unwrap();
        let root = ast.get_root();
        assert_eq!(root.node_kind, NodeKind::Mod(module.into()));
        let block = root.get_children();
        assert_eq!(block.len(), 1);
        let statements = block.first().unwrap();
        assert_eq!(statements.node_kind, NodeKind::Stmt(StmtKind::Block));
        assert_eq!(statements.get_children()[0].node_kind, NodeKind::Stmt(StmtKind::Decl(DeclKind::Lexical(LexicalKind::Let(LetDecl { identifier: "five".into(), expression: ExprKind::Primary(PrimaryExprKind::Lit(LitKind::Num(5))) })))));
        assert_eq!(statements.get_children()[1].node_kind, NodeKind::Stmt(StmtKind::Decl(DeclKind::Lexical(LexicalKind::Let(LetDecl { identifier: "six".into(), expression: ExprKind::Primary(PrimaryExprKind::Lit(LitKind::Num(6))) })))));
        assert_eq!(statements.get_children()[2].node_kind, NodeKind::Stmt(StmtKind::Decl(DeclKind::Lexical(LexicalKind::Let(LetDecl { identifier: "added".into(), expression: ExprKind::Additive(Box::new(AdditiveExpr {
            lhs: ExprKind::Primary(PrimaryExprKind::Id("five".into())),
            rhs: ExprKind::Primary(PrimaryExprKind::Id("six".into()))
        }))})))));
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
use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::ast::{AdditiveExpr, BlockStmt, ConstDecl, ExprKind, IfStmt, LetDecl, PrimaryExprKind, StmtKind, AST};
use crate::parser::ast::DeclKind::Lexical;
use crate::parser::ast::LexicalKind::{Const, Let};
use crate::parser::ast::PrimaryExprKind::{Id, Lit, RegExLiteral};
use crate::parser::lexer::LexerError;
use crate::parser::ParseErrorKind::UnexpectedToken;
use crate::parser::reader::Reader;
use crate::parser::token::{BraceKind, KeywordKind, LitKind, OpKind, ParenthesesKind, PuncKind, Token, TokenKind};
use crate::parser::token::TokenKind::Keyword;
use self::lexer::Lexer;

mod reader;
mod lexer;
mod token;
pub mod ast;
mod grammar;

#[derive(Clone, Debug)]
pub enum ParseErrorKind {
    UnexpectedToken(TokenKind),
}

#[derive(Clone, Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
}

pub struct ParseResult {
    pub ast: AST,
    pub errors: Vec<ParseError>,
}

/// Parses source code to AST based on [ECMAScript Lexical Grammar](https://262.ecma-international.org/#sec-intro).
pub struct Parser {
    ts: Rc<RefCell<Reader<Token>>>,
    fatal_error: Option<ParseError>,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn init(source: &str) -> Result<Self, LexerError> {
        match Lexer::init(source).tokenize() {
            Ok(tokens) => {
                let ts = Rc::new(RefCell::new(Reader::init(tokens)));
                Ok(Parser { ts, fatal_error: None, errors: Vec::new() })
            }
            Err(err) => Err(err),
        }
    }

    pub fn parse(&mut self, module: &str) -> Result<ParseResult, ParseError> {

        let mut ast = AST::new(module);

        loop {
            match self.parse_stmt() {
                Some(stmt) => ast.body.push(stmt),
                None => {
                    if self.is_end() {
                        break;
                    }
                },
            }
        }

        if self.fatal_error.is_some() {
            return Err(self.fatal_error.clone().unwrap())
        }

        Ok(ParseResult { ast, errors: self.errors.clone() })
    }

    fn expect(&mut self, expected: TokenKind) {
        self.expect_peek(expected);
        self.bump();
    }

    fn expect_peek(&mut self, expected: TokenKind) {
        let actual = self.ts.borrow_mut().peek_single().unwrap().kind;
        if actual != expected {
            self.set_fatal_error(UnexpectedToken(actual))
        }
    }

    fn bump(&mut self) {
        self.ts.borrow_mut().bump();
    }

    fn eat(&mut self, check: TokenKind) -> bool {
        if self.peek().kind == check {
            self.bump();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Token {
        self.ts.borrow().peek_single().unwrap()
    }

    fn next(&mut self) -> Token {
        self.ts.borrow_mut().next_single().unwrap()
    }

    fn push_error(&mut self, err: ParseErrorKind) {
        self.errors.push(ParseError { kind: err })
    }

    fn set_fatal_error(&mut self, err: ParseErrorKind) {
        self.fatal_error = Some(ParseError { kind: err });
        self.advance_to_end();

    }

    fn advance_to_end(&mut self) {
        self.ts.borrow_mut().end()
    }

    fn is_end(&self) -> bool {
        self.ts.borrow().has_next()
    }
}
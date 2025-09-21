use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::{AdditiveExpr, BlockStmt, ConstDecl, ExprKind, IfStmt, LetDecl, PrimaryExprKind, StmtKind, Module};
use crate::ast::DeclKind::Lexical;
use crate::ast::LexicalKind::{Const, Let};
use crate::ast::PrimaryExprKind::{Id, Lit, RegExLiteral};
use crate::lexer::LexerError;
use crate::ParseErrorKind::UnexpectedToken;
use crate::reader::Reader;
use crate::token::{BraceKind, KeywordKind, LitKind, OpKind, ParenthesesKind, PuncKind, Token, TokenKind};
use crate::token::TokenKind::Keyword;
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
    pub ast: Module,
    pub errors: Vec<ParseError>,
}

/// Parses source code to AST based on [ECMAScript Lexical Grammar](https://262.ecma-international.org/#sec-intro).
pub struct Parser {
    ts: Rc<RefCell<Reader<Token>>>,
    curr_token: Token,
    fatal_error: Option<ParseError>,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn init(source: &str) -> Result<Self, LexerError> {
        match Lexer::init(source).tokenize() {
            Ok(tokens) => {
                let ts = Rc::new(RefCell::new(Reader::init(tokens)));
                let curr_token = ts.borrow().peek_single().unwrap();
                Ok(Parser { ts, curr_token, fatal_error: None, errors: Vec::new() })
            }
            Err(err) => Err(err),
        }
    }

    pub fn parse(&mut self, module: &str) -> Result<ParseResult, ParseError> {

        let mut ast = Module::new(module);

        loop {
            match self.parse_stmt() {
                Some(stmt) => ast.body.push(stmt),
                None => {
                    if self.is_end() {
                        break;
                    }
                    continue;
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

    /// Checks the provided token against the next token in the stream. If they don't match, creates fatal error.
    fn expect_peek(&mut self, expected: TokenKind) {
        let actual = self.peek_kind();
        if actual != expected {
            self.set_fatal_error(UnexpectedToken(actual))
        }
    }

    /// Moves the cursor forward one position.
    fn bump(&mut self) {
        self.next();
    }

    /// Checks the provided token against the next token in the stream. If matching, bumps the cursor one position.
    fn eat(&mut self, check: TokenKind) -> bool {
        if self.is_end() {
            return false;
        }

        let peek = self.peek_kind();
        if peek == check {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Checks the provided token against the token at the current cursor position.
    fn at(&mut self, check: TokenKind) -> bool {
        let peek = self.peek_kind();
        peek == check
    }

    /// Peek the next token in the stream.
    fn peek(&self) -> Token {
        self.ts.borrow().peek_single().unwrap()
    }

    /// Peek the next token kind in the stream.
    fn peek_kind(&self) -> TokenKind {
        self.peek().kind
    }

    /// Poll the next token in the stream.
    fn next(&mut self) -> Token {
        let next = self.ts.borrow_mut().next_single().unwrap();
        self.curr_token = self.peek();
        next
    }

    /// Poll the next token kind in the stream.
    fn next_kind(&mut self) -> TokenKind {
        self.next().kind
    }

    /// Advances to the end of the token stream.
    fn advance_to_end(&mut self) {
        self.ts.borrow_mut().end();
        self.curr_token = self.ts.borrow().peek_single().unwrap();
    }

    fn push_error(&mut self, err: ParseErrorKind) {
        self.errors.push(ParseError { kind: err })
    }

    fn set_fatal_error(&mut self, err: ParseErrorKind) {
        self.fatal_error = Some(ParseError { kind: err });
        self.advance_to_end();

    }

    /// Checks if it is the end of the token stream.
    fn is_end(&self) -> bool {
        !self.ts.borrow().has_next() || self.curr_token.kind == TokenKind::Eof
    }
}
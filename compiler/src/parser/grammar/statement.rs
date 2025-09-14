use crate::parser::ast::DeclKind::Lexical;
use crate::parser::ast::LexicalKind::{Const, Let};
use crate::parser::ast::{BlockStmt, ConstDecl, IfStmt, LetDecl, StmtKind};
use crate::parser::ParseErrorKind::UnexpectedToken;
use crate::parser::Parser;
use crate::parser::token::{BraceKind, KeywordKind, OpKind, PuncKind, TokenKind};
use crate::parser::token::TokenKind::Keyword;

impl Parser {
    pub(crate) fn parse_stmt(&mut self) -> Option<StmtKind> {

        match self.peek().kind {
            TokenKind::Eof => {
                None
            }
            TokenKind::Id(_) => todo!(),
            TokenKind::Keyword(k) => {
                match k {
                    KeywordKind::Const => Some(StmtKind::Decl(Lexical(Const(self.parse_const_decl_stmt())))),
                    KeywordKind::Let => Some(StmtKind::Decl(Lexical(Let(self.parse_let_decl_stmt())))),
                    KeywordKind::If => Some(StmtKind::If(self.parse_if_stmt())),
                    _ => todo!(),
                }
            }
            TokenKind::LineTerminator(_) => {
                self.bump();
                None
            }
            TokenKind::Lit(_) => todo!(),
            TokenKind::Punc(punc) => {
                match punc {
                    PuncKind::SemiColon => {
                        self.bump();
                        None
                    }
                    _ => todo!()
                }
            }
            TokenKind::Unicode(_) => {
                self.bump();
                None
            }
            _ => {
                self.bump();
                None
            },
        }
    }

    pub(crate) fn parse_stmt_list(&mut self) -> StmtKind {
        match self.peek().kind {
            TokenKind::Keyword(k) => match k {
                KeywordKind::Const => StmtKind::Decl(Lexical(Const(self.parse_const_decl_stmt()))),
                KeywordKind::Let => StmtKind::Decl(Lexical(Let(self.parse_let_decl_stmt()))),
                KeywordKind::If => StmtKind::If(self.parse_if_stmt()),
                _ => todo!()
            }
            TokenKind::Punc(PuncKind::Brace(BraceKind::Left)) => StmtKind::Block(self.parse_block_stmt()),
            _ => todo!()
        }
    }

    fn parse_let_decl_stmt(&mut self) -> LetDecl {

        self.expect(Keyword(KeywordKind::Let));

        let mut id = String::new();
        match self.next().kind {
            TokenKind::Id(val) => {id = val }
            t => self.set_fatal_error(UnexpectedToken(t))
        }

        self.expect(TokenKind::Punc(PuncKind::Op(OpKind::Assign)));

        LetDecl {
            identifier: id,
            expression: self.parse_expr()
        }
    }

    fn parse_const_decl_stmt(&mut self) -> ConstDecl {

        self.expect(Keyword(KeywordKind::Const));

        let mut id = String::new();
        match self.next().kind {
            TokenKind::Id(val) => {id = val }
            t => self.push_error(UnexpectedToken(t))
        }

        self.expect(TokenKind::Punc(PuncKind::Op(OpKind::Assign)));

        ConstDecl {
            identifier: id,
            expression: self.parse_expr()
        }
    }

    fn parse_if_stmt(&mut self) -> IfStmt {

        self.expect(Keyword(KeywordKind::If));

        let cond = self.parse_paren_expr();

        let body = Box::new(self.parse_stmt_list());

        let mut alternative = None;
        if self.eat(Keyword(KeywordKind::Else)) {
            alternative = Some(Box::new(self.parse_stmt_list()));
        }

        IfStmt { cond, body, alternative }
    }

    pub(crate) fn parse_block_stmt(&mut self) -> BlockStmt {

        self.expect(TokenKind::Punc(PuncKind::Brace(BraceKind::Left)));

        let mut stmts = Vec::new();

        while self.peek().kind == TokenKind::Punc(PuncKind::Brace(BraceKind::Right)) {
            stmts.push(self.parse_stmt_list());
        }

        self.expect(TokenKind::Punc(PuncKind::Brace(BraceKind::Right)));

        BlockStmt { stmts}
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_decl() {
        let module = "Test";
        let mut parser = Parser::init("let five = 5;\n let six = 6\n let added = five + six").unwrap();
        let res = parser.parse(module.into());
        assert!(res.is_ok());
    }

    #[test]
    fn test_if() {
        let module = "Test";
        let mut parser = Parser::init("let y = 5;\nif (y == 5) {\nreturn;\n}").unwrap();
        let res = parser.parse(module.into());
        assert!(res.is_ok());
    }
}
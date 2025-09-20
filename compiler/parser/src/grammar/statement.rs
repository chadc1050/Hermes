use crate::ast::DeclKind::Lexical;
use crate::ast::LexicalKind::{Const, Let};
use crate::ast::{BlockStmt, BreakStmt, BreakableStmtKind, ConstDecl, ContinueStmt, DebugStmt, EmptyStmt, ExprKind, ExprStmt, IfStmt, LetDecl, ReturnStmt, StmtKind, SwitchStmt, ThrowStmt, TryStmt, WithStmt};
use crate::ParseErrorKind::UnexpectedToken;
use crate::Parser;
use crate::token::{BraceKind, KeywordKind, OpKind, PuncKind, TokenKind};
use crate::token::TokenKind::Keyword;

impl Parser {

    /// Section 14 [Statement](https://tc39.es/ecma262/#sec-ecmascript-language-statements-and-declarations)
    pub(crate) fn parse_stmt(&mut self) -> Option<StmtKind> {

        match self.peek().kind {
            TokenKind::Eof => None,
            TokenKind::Id(id) => todo!(),
            TokenKind::Keyword(k) => {
                match k {
                    KeywordKind::Break => Some(StmtKind::Break(self.parse_break_stmt())),
                    KeywordKind::Const => Some(StmtKind::Decl(Lexical(Const(self.parse_const_decl_stmt())))),
                    KeywordKind::Continue => Some(StmtKind::Continue(self.parse_continue_stmt())),
                    KeywordKind::Debugger => Some(StmtKind::Debugger(self.parse_debugger_stmt())),
                    KeywordKind::Let => Some(StmtKind::Decl(Lexical(Let(self.parse_let_decl_stmt())))),
                    KeywordKind::If => Some(StmtKind::If(self.parse_if_stmt())),
                    KeywordKind::Return => Some(StmtKind::Return(self.parse_return_stmt())),
                    KeywordKind::Switch => Some(StmtKind::Breakable(BreakableStmtKind::Switch(self.parse_switch_stmt()))),
                    KeywordKind::Throw => Some(StmtKind::Throw(self.parse_throw_stmt())),
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
                    PuncKind::SemiColon => Some(StmtKind::Empty(self.parse_empty_stmt())),
                    PuncKind::Brace(BraceKind::Left) => Some(StmtKind::Block(self.parse_block_stmt())),
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

    /// Section 14.2 [Block](https://tc39.es/ecma262/#sec-block)
    pub(crate) fn parse_block_stmt(&mut self) -> BlockStmt {

        self.expect(TokenKind::Punc(PuncKind::Brace(BraceKind::Left)));

        let mut stmts = Vec::new();

        while self.peek().kind == TokenKind::Punc(PuncKind::Brace(BraceKind::Right)) {
            stmts.push(self.parse_stmt_list());
        }

        self.expect(TokenKind::Punc(PuncKind::Brace(BraceKind::Right)));

        BlockStmt { stmts}
    }

    /// Section 14.3.1 [Let and Const Declarations](https://tc39.es/ecma262/#sec-let-and-const-declarations)
    pub(crate) fn parse_let_decl_stmt(&mut self) -> LetDecl {

        self.expect(Keyword(KeywordKind::Let));

        let mut id = String::new();
        match self.next_kind() {
            TokenKind::Id(val) => {id = val }
            t => self.set_fatal_error(UnexpectedToken(t))
        }

        self.expect(TokenKind::Punc(PuncKind::Op(OpKind::Assign)));

        let let_decl = LetDecl {
            identifier: id,
            expression: self.parse_expr()
        };

        self.eat(TokenKind::Punc(PuncKind::SemiColon));

        let_decl
    }

    /// Section 14.3.1 [Let and Const Declarations](https://tc39.es/ecma262/#sec-let-and-const-declarations)
    pub(crate) fn parse_const_decl_stmt(&mut self) -> ConstDecl {

        self.expect(Keyword(KeywordKind::Const));

        let mut id = String::new();
        match self.next().kind {
            TokenKind::Id(val) => {id = val }
            t => self.push_error(UnexpectedToken(t))
        }

        self.expect(TokenKind::Punc(PuncKind::Op(OpKind::Assign)));

        let const_decl = ConstDecl {
            identifier: id,
            expression: self.parse_expr()
        };

        self.eat(TokenKind::Punc(PuncKind::SemiColon));

        const_decl
    }

    /// Section 14.4 [Empty Statement](https://tc39.es/ecma262/#sec-empty-statement)
    pub(crate) fn parse_empty_stmt(&mut self) -> EmptyStmt {
        self.expect(TokenKind::Punc(PuncKind::SemiColon));
        EmptyStmt{}
    }

    /// Section 14.5 [Expression Statement](https://tc39.es/ecma262/#sec-expression-statement)
    pub(crate) fn parse_expr_stmt(&mut self) -> ExprStmt {
        let expr = self.parse_expr();
        self.eat(TokenKind::Punc(PuncKind::SemiColon));
        ExprStmt { expr }
    }

    /// Section 14.6 [If Statement](https://tc39.es/ecma262/#sec-if-statement)
    pub(crate) fn parse_if_stmt(&mut self) -> IfStmt {

        self.expect(Keyword(KeywordKind::If));

        let cond = self.parse_paren_expr();

        let body = Box::new(self.parse_stmt_list());

        let mut alternative = None;
        if self.eat(Keyword(KeywordKind::Else)) {
            alternative = Some(Box::new(self.parse_stmt_list()));
        }

        IfStmt { cond, body, alternative }
    }

    /// Section 14.8 [Continue Statement](https://tc39.es/ecma262/#sec-continue-statement)
    pub(crate) fn parse_continue_stmt(&mut self) -> ContinueStmt {
        self.expect(Keyword(KeywordKind::Continue));

        // TODO: Check for label
        let label = None;

        self.eat(TokenKind::Punc(PuncKind::SemiColon));

        ContinueStmt { label }
    }

    /// Section 14.9 [Break Statement](https://tc39.es/ecma262/#sec-break-statement)
    pub(crate) fn parse_break_stmt(&mut self) -> BreakStmt {
        self.expect(Keyword(KeywordKind::Break));

        // TODO: Check for label
        let label = None;

        self.eat(TokenKind::Punc(PuncKind::SemiColon));

        BreakStmt { label }
    }

    /// Section 14.10 [Return Statement](https://tc39.es/ecma262/#sec-return-statement)
    pub(crate) fn parse_return_stmt(&mut self) -> ReturnStmt {
        self.expect(Keyword(KeywordKind::Return));

        // TODO: Check for void return as well.
        let expr = self.parse_expr();

        ReturnStmt {
            return_value: Some(Box::new(expr))
        }
    }

    /// Section 14.11 [With Statement (LEGACY)](https://tc39.es/ecma262/#sec-with-statement)
    pub(crate) fn parse_with_stmt(&mut self) -> WithStmt {
        self.expect(Keyword(KeywordKind::With));
        let with_expr = self.parse_paren_expr();
        let with_block = Box::new(self.parse_stmt_list());
        WithStmt {
            with_expr,
            with_block,
        }
    }

    /// Section 14.12 [Switch Statement](https://tc39.es/ecma262/#sec-switch-statement)
    pub(crate) fn parse_switch_stmt(&mut self) -> SwitchStmt {
        self.expect(Keyword(KeywordKind::Switch));

        todo!()
    }

    /// Section 14.14 [Throw Statement](https://tc39.es/ecma262/#sec-throw-statement)
    pub(crate) fn parse_throw_stmt(&mut self) -> ThrowStmt {
        self.expect(Keyword(KeywordKind::Throw));
        let throws_expr = self.parse_expr();
        ThrowStmt {
            throws_expr,
        }
    }

    /// 14.15 [Try Statement](https://tc39.es/ecma262/#sec-try-statement)
    pub(crate) fn parse_try_stmt(&mut self) -> TryStmt {
        self.expect(Keyword(KeywordKind::Try));

        let try_block = Box::new(self.parse_block_stmt());

        let mut catch_block = None;

        if self.eat(Keyword(KeywordKind::Catch)) {
            // TODO: Handle catch
        }

        let mut finally_block = None;

        if self.eat(Keyword(KeywordKind::Finally)) {
            finally_block = Some(Box::new(self.parse_block_stmt()));
        }


        TryStmt {
            try_block,
            catch_block,
            finally_block,
        }
    }

    /// Section 14.16 [Debugger Statement](https://tc39.es/ecma262/#sec-debugger-statement)
    pub(crate) fn parse_debugger_stmt(&mut self) -> DebugStmt {
        self.expect(Keyword(KeywordKind::Debugger));

        self.eat(TokenKind::Punc(PuncKind::SemiColon));

        DebugStmt{}
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
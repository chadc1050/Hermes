use crate::ast::{CondExpr, ExprKind, PrimaryExprKind, SeqExpr, YieldExpr};
use crate::ast::ExprKind::Sequence;
use crate::Parser;
use crate::token::{BraceKind, BracketKind, KeywordKind, OpKind, ParenthesesKind, PuncKind, TokenKind};

impl Parser {
    pub(crate) fn parse_paren_expr(&mut self) -> ExprKind {
        self.expect(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)));
        let expr = self.parse_expr();
        self.expect(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)));
        expr
    }

    /// Section 13 [Expression](https://tc39.es/ecma262/#sec-ecmascript-language-expressions)
    pub(crate) fn parse_expr(&mut self) -> ExprKind {

        let first = self.parse_assign_expr();

        if !self.at(TokenKind::Punc(PuncKind::Comma)) {
            return first;
        }

        self.parse_sequence_expr(first);

        todo!()
    }

    /// Section 13.16 [Comma Operator](https://tc39.es/ecma262/#sec-comma-operator)
    pub(crate) fn parse_sequence_expr(&mut self, first: ExprKind) -> ExprKind {
        let mut expressions = vec![first];
        while self.eat(TokenKind::Punc(PuncKind::Comma)) {
            let expr = self.parse_assign_expr();
            expressions.push(expr)
        }
        Sequence(SeqExpr { exprs: expressions })
    }

    /// Section 13.15 [Assignment Operator](https://tc39.es/ecma262/#prod-AssignmentExpression)
    pub(crate) fn parse_assign_expr(&mut self) -> ExprKind {

        // [+Yield] YieldExpression[?In,?Await]
        if self.at(TokenKind::Keyword(KeywordKind::Yield)) {
            return self.parse_yield_expr();
        }

        // TODO: Arrow function

        let lhs = self.parse_lhs_expr();

        if self.is_assignment_op() {
            self.bump();
            return self.parse_assign_expr();
        }

        self.parse_conditional_expr_with_condition(lhs)
    }

    /// Section 13.14 [Conditional Operator](https://tc39.es/ecma262/#sec-conditional-operator)
    pub(crate) fn parse_conditional_expr_with_condition(&mut self, cond: ExprKind) -> ExprKind {
        if !self.eat(TokenKind::Punc(PuncKind::Question)) {
            return cond;
        }

        let consequent = Box::new(self.parse_assign_expr());

        self.expect(TokenKind::Punc(PuncKind::Colon));

        let alternate =  Box::new(self.parse_assign_expr());

        ExprKind::Conditional(CondExpr{ cond: Box::new(cond), consequent, alternate, })
    }

    /// Section 13.13 [Binary Logical Operator](https://tc39.es/ecma262/#sec-binary-logical-operators)
    pub(crate) fn parse_binary_logical_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.12 [Binary Bitwise Operator](https://tc39.es/ecma262/#sec-binary-bitwise-operators)
    pub(crate) fn parse_binary_bitwise_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.11 [Equality Operator](https://tc39.es/ecma262/#sec-equality-operators)
    pub(crate) fn parse_equality_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.10 [Relational Operator](https://tc39.es/ecma262/#sec-relational-operators)
    pub(crate) fn parse_relation_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.9 [Bitwise Shift Operator](https://tc39.es/ecma262/#sec-bitwise-shift-operators)
    pub(crate) fn parse_bitwise_shift_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.8 [Additive Operator](https://tc39.es/ecma262/#sec-additive-operators)
    pub(crate) fn parse_additive_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.7 [Multiplicative Operator](https://tc39.es/ecma262/#sec-multiplicative-operators)
    pub(crate) fn parse_multiplicative_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.6 [Exponentiation Operator](https://tc39.es/ecma262/#sec-exp-operator)
    pub(crate) fn parse_exponent_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.5 [Unary Operator](https://tc39.es/ecma262/#sec-unary-operators)
    pub(crate) fn parse_unary_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.4 [Update Operator](https://tc39.es/ecma262/#sec-update-expressions)
    pub(crate) fn parse_update_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.3 [Left-Hand-Side Expressions](https://tc39.es/ecma262/#sec-left-hand-side-expressions)
    pub(crate) fn parse_lhs_expr(&mut self) -> ExprKind {
        todo!()
    }

    /// Section 13.2 [Primary Expressions](https://tc39.es/ecma262/#sec-primary-expression)
    pub(crate) fn parse_primary_expr(&mut self) -> ExprKind {
        todo!()
    }

    pub(crate) fn parse_yield_expr(&mut self) -> ExprKind {
        self.expect(TokenKind::Keyword(KeywordKind::Yield));
        let mut delegate = false;
        if self.eat(TokenKind::Punc(PuncKind::Op(OpKind::Multiplication))) { // Might want to generalize lexer to star
            delegate = true;
        }

        let not_assign_expr = matches!(self.curr_token.kind,
                    TokenKind::Punc(PuncKind::SemiColon)
                    | TokenKind::Eof
                    | TokenKind::Punc(PuncKind::Bracket(BracketKind::Left))
                    | TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left))
                    | TokenKind::Punc(PuncKind::Brace(BraceKind::Left))
                    | TokenKind::Punc(PuncKind::Colon)
                    | TokenKind::Punc(PuncKind::Comma)
            );

        if not_assign_expr {
            return ExprKind::Yield(YieldExpr{ delegate, arg: None });
        }

        ExprKind::Yield(YieldExpr{ delegate, arg: Some(Box::new(self.parse_assign_expr())) })
    }

    pub(crate) fn parse_this_expr(&mut self) -> ExprKind {
        self.expect(TokenKind::Keyword(KeywordKind::This));
        ExprKind::Primary(PrimaryExprKind::This)
    }

    pub(crate) fn is_assignment_op(&self) -> bool {
        matches!(
            self.peek_kind(),
            TokenKind::Punc(PuncKind::Op(OpKind::Equal))
            | TokenKind::Punc(PuncKind::Op(OpKind::AdditonAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::SubtractionAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::MultiplicationAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::DivisionAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::ModAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::LeftShiftAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::RightShiftAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::UnsignedRightShiftAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::OrAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::AndAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::BitOrAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::BitXorAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::BitAndAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::NullishCoalescingAssign))
            | TokenKind::Punc(PuncKind::Op(OpKind::ExponentialAssign))
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ExprKind, ExprStmt, StmtKind, YieldExpr};
    use crate::ast::ExprKind::{Primary, Yield};
    use crate::ast::PrimaryExprKind::This;
    use super::Parser;

    #[test]
    fn test_this() {
        let mut p = Parser::init("this").unwrap();
        let res = p.parse("test").unwrap();
        assert!(res.errors.is_empty());
        assert_eq!(res.ast.body.len(), 1);
        assert_eq!(res.ast.body[0], StmtKind::Expression(ExprStmt(Primary(This))));
    }

    #[test]
    fn test_yield() {
        let mut p = Parser::init("yield;").unwrap();
        let res = p.parse("test").unwrap();
        assert!(res.errors.is_empty());
        assert_eq!(res.ast.body.len(), 1);
        assert_eq!(res.ast.body[0], StmtKind::Expression(ExprStmt(Yield(YieldExpr{ delegate: false, arg: None }))));
    }
}
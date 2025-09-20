use crate::ast::{AdditiveExpr, ExprKind, PrimaryExprKind};
use crate::ast::PrimaryExprKind::{Id, Lit, RegExLiteral};
use crate::Parser;
use crate::token::{KeywordKind, LitKind, OpKind, ParenthesesKind, PuncKind, TokenKind};

impl Parser {
    pub(crate) fn parse_paren_expr(&mut self) -> ExprKind {
        self.expect(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Left)));
        let expr = self.parse_expr();
        self.expect(TokenKind::Punc(PuncKind::Parentheses(ParenthesesKind::Right)));
        expr
    }

    /// Section 13 [Expression](https://tc39.es/ecma262/#sec-ecmascript-language-expressions)
    pub(crate) fn parse_expr(&mut self) -> ExprKind {
        match self.next_kind() {
            TokenKind::Id(first_id) => {
                match self.next_kind() {
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
                                        match self.next_kind() {
                                            TokenKind::Id(second_id) => {
                                                ExprKind::Additive(Box::new(AdditiveExpr {lhs: ExprKind::Primary(Id(first_id)), rhs: ExprKind::Primary(Id(second_id))}))
                                            },
                                            TokenKind::Keyword(_) => todo!(),
                                            TokenKind::LineTerminator(_) => todo!(),
                                            TokenKind::Lit(_) => todo!(),
                                            TokenKind::Punc(_) => todo!(),
                                            _ => todo!()
                                        }
                                    }
                                    OpKind::Multiplication => todo!(),
                                    OpKind::Equal => {
                                        match self.next_kind() {
                                            TokenKind::Id(_) => todo!(),
                                            TokenKind::Keyword(_) => todo!(),
                                            TokenKind::Lit(lit) => {
                                                todo!()
                                            }
                                            TokenKind::Punc(_) => todo!(),
                                            _ => todo!()
                                        }
                                    }
                                    _ => todo!()
                                }
                            }
                            PuncKind::Parentheses(_) => todo!(),
                            PuncKind::SemiColon => todo!(),
                        }
                    }
                    _ => todo!()
                }
            }
            TokenKind::Keyword(kw) => match kw {
                KeywordKind::This => {
                    ExprKind::Primary(PrimaryExprKind::This)
                }
                _ => todo!()
            }
            TokenKind::Lit(literal) => {
                match literal {
                    LitKind::BigIntSuffix(bis) => ExprKind::Primary(Lit(LitKind::BigIntSuffix(bis))),
                    LitKind::Bool(bool) => ExprKind::Primary(Lit(LitKind::Bool(bool))),
                    LitKind::Dec(dec) => ExprKind::Primary(Lit(LitKind::Dec(dec))),
                    LitKind::DecimalBigInteger(dbi) => ExprKind::Primary(Lit(LitKind::DecimalBigInteger(dbi))),
                    LitKind::DecimalInteger(di) => ExprKind::Primary(Lit(LitKind::DecimalInteger(di))),
                    LitKind::NonDecimalInteger(ndi) => ExprKind::Primary(Lit(LitKind::NonDecimalInteger(ndi))),
                    LitKind::Null => ExprKind::Primary(Lit(LitKind::Null)),
                    LitKind::Num(num) => ExprKind::Primary(Lit(LitKind::Num(num))),
                    LitKind::String(str) => ExprKind::Primary(Lit(LitKind::String(str))),
                    LitKind::RegEx(regex) => ExprKind::Primary(RegExLiteral(regex))
                }
            }
            TokenKind::Punc(_) => todo!(),
            _ => todo!()
        }
    }
}
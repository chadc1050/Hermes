use serde::Serialize;
use crate::token::{KeywordKind, LitKind};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Module {
    pub module_name: String,
    pub body: Vec<StmtKind>,
}

impl Module {
    pub fn new(module: &str) -> Self {
        Module {
            module_name: module.to_string(),
            body: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum StmtKind {
    Block(BlockStmt),
    Decl(DeclKind),
    Variable,
    Empty(EmptyStmt),
    Expression(ExprStmt),
    If(IfStmt),
    Breakable(BreakableStmtKind),
    Continue(ContinueStmt),
    Break(BreakStmt),
    Return(ReturnStmt),
    With(WithStmt),
    Throw(ThrowStmt),
    Try(TryStmt),
    Debugger(DebugStmt)
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BreakableStmtKind {
    Switch(SwitchStmt),
    Iter
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum DeclKind {
    Hoistable(HoistableDeclKind),
    Class,
    Lexical(LexicalKind)
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum LexicalKind {
    Let(LetDecl),
    Const(ConstDecl),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum HoistableDeclKind {
    Function,
    AsyncFunction,
    Generator,
    AsyncGenerator,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ExprKind {
    Primary(PrimaryExprKind),
    Additive(Box<AdditiveExpr>),
    Multiplicative,
    ConditionalExpr(Box<CondExpr>)
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PrimaryExprKind {
    This,
    Id(String),
    Lit(LitKind),
    ArrayLiteral,
    ObjectLiteral,
    FunctionExpr,
    AsyncFunctionExpr,
    ClassExpression,
    GeneratorExpr,
    AsyncGeneratorExpr,
    RegExLiteral(String),
    TemplateLiteral,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LetDecl {
    pub identifier: String,
    pub expression: ExprKind
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConstDecl {
    pub identifier: String,
    pub expression: ExprKind
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AdditiveExpr {
    pub lhs: ExprKind,
    pub rhs: ExprKind
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CondExpr {
    pub condition: ExprKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BlockStmt {
    pub stmts: Vec<StmtKind>
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BreakStmt {
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContinueStmt {
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DebugStmt;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EmptyStmt;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExprStmt {
    pub expr: ExprKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IfStmt {
    pub cond: ExprKind,
    pub body: Box<StmtKind>,
    pub alternative: Option<Box<StmtKind>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReturnStmt {
    pub return_value: Option<Box<ExprKind>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SwitchStmt {
    pub switch_expr: ExprKind,
    pub cases: Option<Vec<SwitchCase>>,
    pub default: Option<SwitchCase>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ThrowStmt {
    pub throws_expr: ExprKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TryStmt {
    pub try_block: Box<BlockStmt>,
    pub catch_block: Option<Box<CatchClause>>,
    pub finally_block: Option<Box<BlockStmt>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WithStmt {
    pub with_expr: ExprKind,
    pub with_block: Box<StmtKind>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CatchClause {
    pub catch_param: Option<Box<ExprKind>>,
    pub catch_block: Box<BlockStmt>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SwitchCase {
    pub test: Option<ExprKind>,
    pub body: Vec<Box<StmtKind>>,
}
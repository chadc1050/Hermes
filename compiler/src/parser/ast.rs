use serde::Serialize;
use crate::parser::token::{LitKind};

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
    Empty,
    Expression,
    If(IfStmt),
    Breakable(BreakableStmtKind),
    Continue,
    Break,
    Return,
    With,
    Throw,
    Try,
    Debugger
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum BreakableStmtKind {
    Switch,
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
pub struct IfStmt {
    pub cond: ExprKind,
    pub body: Box<StmtKind>,
    pub alternative: Option<Box<StmtKind>>,
}

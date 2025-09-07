use std::any::Any;
use crate::parser::token::{LitKind, TokenKind};

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Mod(String),
    Stmt(StmtKind),
    Expr(ExprKind),
}

#[derive(Debug, PartialEq)]
pub enum StmtKind {
    Block,
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

#[derive(Debug, PartialEq)]
pub enum BreakableStmtKind {
    Switch,
    Iter
}

#[derive(Debug, PartialEq)]
pub enum DeclKind {
    Hoistable(HoistableDeclKind),
    Class,
    Lexical(LexicalKind)
}

#[derive(Debug, PartialEq)]
pub enum LexicalKind {
    Let(LetDecl),
    Const(ConstDecl),
}

#[derive(Debug, PartialEq)]
pub enum HoistableDeclKind {
    Function,
    AsyncFunction,
    Generator,
    AsyncGenerator,
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    Primary(PrimaryExprKind),
    Additive(Box<AdditiveExpr>),
    Multiplicative,
    ConditionalExpr(Box<CondExpr>)
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct LetDecl {
    pub identifier: String,
    pub expression: ExprKind
}

#[derive(Debug, PartialEq)]
pub struct ConstDecl {
    pub identifier: String,
    pub expression: ExprKind
}

#[derive(Debug, PartialEq)]
pub struct AdditiveExpr {
    pub lhs: ExprKind,
    pub rhs: ExprKind
}

#[derive(Debug, PartialEq)]
pub struct CondExpr {
    pub condition: ExprKind,
}

#[derive(Debug, PartialEq)]
pub struct BlockStmt {
    pub stmts: Vec<StmtKind>
}

#[derive(Debug, PartialEq)]
pub struct IfStmt {
    pub cond: ExprKind,
    pub body: BlockStmt,
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_kind: NodeKind,
    children: Vec<Node>,
}

impl Node { 
    pub fn new(node_kind: NodeKind) -> Self {
        Node {
            node_kind,
            children: Vec::new(),
        }
    }

    pub fn get_children(&self) -> &Vec<Node> {
        &self.children
    }

    pub fn get_children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn clear_children(&mut self) {
        self.children.clear();
    }
}


pub struct AST {
    root: Node,
}

impl AST {
    pub fn new(module: String) -> Self {
        AST {
            root: Node::new(NodeKind::Mod(module)),
        }
    }

    pub fn get_root(&mut self) -> &mut Node {
        &mut self.root
    }
}
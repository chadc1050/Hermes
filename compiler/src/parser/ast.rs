use serde::Serialize;
use crate::parser::token::{LitKind};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum NodeKind {
    Mod(String),
    Stmt(StmtKind),
    Expr(ExprKind),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
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
    pub body: BlockStmt,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
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


#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AST {
    root: Node,
}

impl AST {
    pub fn new(module: &str) -> Self {
        AST {
            root: Node::new(NodeKind::Mod(module.into())),
        }
    }

    pub fn get_root(&mut self) -> &mut Node {
        &mut self.root
    }
}
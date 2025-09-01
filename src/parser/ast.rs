use std::any::Any;
use crate::parser::token::{LiteralKind, TokenKind};

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Module,
    Statement(StatementKind),
    Declaration(DeclarationKind),
    Expression(ExpressionKind),
}

#[derive(Debug, PartialEq)]
pub enum StatementKind {
    Block,
    Variable,
    Empty,
    Expression,
    If(IfStatement),
    Breakable(BreakableStatementKind),
    Continue,
    Break,
    Return,
    With,
    Throw,
    Try,
    Debugger
}

#[derive(Debug, PartialEq)]
pub enum BreakableStatementKind {
    Switch,
    Iteration
}

#[derive(Debug, PartialEq)]
pub enum DeclarationKind {
    Hoistable(HoistableDeclarationKind),
    Class,
    Lexical(LexicalKind)
}

#[derive(Debug, PartialEq)]
pub enum LexicalKind {
    Let(LetDeclaration),
    Const(ConstDeclaration),
}

#[derive(Debug, PartialEq)]
pub enum HoistableDeclarationKind {
    Function,
    AsyncFunction,
    Generator,
    AsyncGenerator,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    Primary(PrimaryExpressionKind),
    Additive(Box<AdditiveExpression>),
    Multiplicative,
    ConditionalExpression
}

#[derive(Debug, PartialEq)]
pub enum PrimaryExpressionKind {
    This,
    Identifier(String),
    Literal(LiteralKind),
    ArrayLiteral,
    ObjectLiteral,
    FunctionExpression,
    AsyncFunctionExpression,
    ClassExpression,
    GeneratorExpression,
    AsyncGeneratorExpression,
    RegExLiteral(String),
    TemplateLiteral,
}

#[derive(Debug, PartialEq)]
pub struct LetDeclaration {
    pub identifier: String,
    pub expression: ExpressionKind
}

#[derive(Debug, PartialEq)]
pub struct ConstDeclaration {
    pub identifier: String,
    pub expression: ExpressionKind
}

#[derive(Debug, PartialEq)]
pub struct AdditiveExpression {
    pub lhs: ExpressionKind,
    pub rhs: ExpressionKind
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: ExpressionKind,
    pub body: Vec<Node>
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
    pub fn new() -> Self {
        AST {
            root: Node::new(NodeKind::Module),
        }
    }

    pub fn get_root(&mut self) -> &mut Node {
        &mut self.root
    }
}
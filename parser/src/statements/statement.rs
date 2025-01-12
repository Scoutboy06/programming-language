use super::{
    BlockStatement, ExpressionStatement, ForStatement, FunctionDeclaration, IfStatement,
    ReturnStatement, Shebang, VariableDeclaration, WhileStatement,
};
use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    EmptyStatement(Box<EmptyStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
    BlockStatement(Box<BlockStatement>),
    IfStatement(Box<IfStatement>),
    SwitchStatement(Box<()>),
    WhileStatement(Box<WhileStatement>),
    DoWhileStatement(Box<()>),
    ForStatement(Box<ForStatement>),
    ForInStatement(Box<()>),
    ForOfStatement(Box<()>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    VariableDeclaration(Box<VariableDeclaration>),
    ReturnStatement(Box<ReturnStatement>),
    BreakStatement(Box<()>),
    ContinueStatement(Box<()>),
    ThrowStatement(Box<()>),
    TryStatement(Box<()>),
    DebuggerStatement(Box<()>),
    LabeledStatement(Box<()>),
    WithStatement(Box<()>), // Deprecated
    ClassDeclaration(Box<()>),
    ImportDeclaration(Box<()>),
    ExportDeclaration(Box<()>),
    Shebang(Box<Shebang>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EmptyStatement {
    node: Node,
}

impl Statement {
    pub fn node(&self) -> &Node {
        match self {
            Self::EmptyStatement(s) => &s.node,
            Self::ExpressionStatement(s) => &s.node,
            Self::BlockStatement(s) => &s.node,
            Self::IfStatement(s) => &s.node,
            Self::FunctionDeclaration(s) => &s.node,
            Self::VariableDeclaration(s) => &s.node,
            Self::ReturnStatement(s) => &s.node,
            Self::Shebang(s) => &s.node,
            _ => todo!("Statement::node()"),
        }
    }
}

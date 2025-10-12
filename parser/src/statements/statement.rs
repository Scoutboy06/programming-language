use crate::nodes::Node;
use crate::statements::{
    BlockStatement, BreakStatement, ContinueStatement, EnumStatement, ExpressionStatement,
    ForStatement, FunctionDeclaration, IfStatement, ReturnStatement, Shebang, ThrowStatement,
    VariableDeclaration, WhileStatement,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    EmptyStatement(Box<EmptyStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
    EnumStatement(Box<EnumStatement>),
    BlockStatement(Box<BlockStatement>),
    IfStatement(Box<IfStatement>),
    SwitchStatement(Box<()>),
    WhileStatement(Box<WhileStatement>),
    DoWhileStatement(Box<()>),
    ForStatement(Box<ForStatement>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    VariableDeclaration(Box<VariableDeclaration>),
    ReturnStatement(Box<ReturnStatement>),
    BreakStatement(Box<BreakStatement>),
    ContinueStatement(Box<ContinueStatement>),
    ThrowStatement(Box<ThrowStatement>),
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
            Self::ThrowStatement(s) => &s.node,
            Self::ForStatement(s) => &s.node(),
            Self::ContinueStatement(s) => &s.node,
            Self::BreakStatement(s) => &s.node,
            _ => todo!("Statement::node()"),
        }
    }
}

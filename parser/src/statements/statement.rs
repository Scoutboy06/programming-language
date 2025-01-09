use super::{
    BlockStatement, ExpressionStatement, FunctionDeclaration, IfStatement, ReturnStatement,
    Shebang, VariableDeclaration,
};
use crate::nodes::Node;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    EmptyStatement(Box<EmptyStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
    BlockStatement(Box<BlockStatement>),
    IfStatement(Box<IfStatement>),
    SwitchStatement(Box<()>),
    WhileStatement(Box<()>),
    DoWhileStatement(Box<()>),
    ForStatement(Box<()>),
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

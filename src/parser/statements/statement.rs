use super::{BlockStatement, FunctionDeclaration, Shebang, VariableDeclaration};
use crate::{
    lexer::Lexer,
    parser::{Expression, Node},
};

#[derive(Debug, PartialEq)]
pub enum Statement {
    EmptyStatement(Box<EmptyStatement>),
    ExpressionStatement(Box<Expression>),
    BlockStatement(Box<BlockStatement>),
    IfStatement(Box<()>),
    SwitchStatement(Box<()>),
    WhileStatement(Box<()>),
    DoWhileStatement(Box<()>),
    ForStatement(Box<()>),
    ForInStatement(Box<()>),
    ForOfStatement(Box<()>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    VariableDeclaration(Box<VariableDeclaration>),
    ReturnStatement(Box<()>),
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

#[derive(Debug, PartialEq)]
pub struct EmptyStatement {
    node: Node,
}

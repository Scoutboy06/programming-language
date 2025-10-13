use crate::nodes::Node;
use crate::statements::{
    BlockStatement, BreakStatement, ContinueStatement, DebuggerStatement, Directive,
    DoWhileStatement, EmptyStatement, EnumStatement, ExpressionStatement, ForInStatement,
    ForStatement, FunctionDeclaration, IfStatement, LabeledStatement, ReturnStatement,
    SwitchStatement, ThrowStatement, TryStatement, VariableDeclaration, WhileStatement,
    WithStatement,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    BlockStatement(Box<BlockStatement>),
    BreakStatement(Box<BreakStatement>),
    ContinueStatement(Box<ContinueStatement>),
    DebuggerStatement(Box<DebuggerStatement>),
    Directive(Box<Directive>),
    DoWhileStatement(Box<DoWhileStatement>),
    EmptyStatement(Box<EmptyStatement>),
    EnumStatement(Box<EnumStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
    ForInStatement(Box<ForInStatement>),
    ForStatement(Box<ForStatement>),
    FunctionDeclaration(Box<FunctionDeclaration>),
    IfStatement(Box<IfStatement>),
    LabeledStatement(Box<LabeledStatement>),
    ReturnStatement(Box<ReturnStatement>),
    SwitchStatement(Box<SwitchStatement>),
    ThrowStatement(Box<ThrowStatement>),
    TryStatement(Box<TryStatement>),
    VariableDeclaration(Box<VariableDeclaration>),
    WhileStatement(Box<WhileStatement>),
    WithStatement(Box<WithStatement>),
    // ClassDeclaration(Box<()>),
    // ImportDeclaration(Box<()>),
    // ExportDeclaration(Box<()>),
}

impl Statement {
    pub fn node(&self) -> &Node {
        match self {
            Self::BlockStatement(s) => &s.node,
            Self::BreakStatement(s) => &s.node,
            Self::ContinueStatement(s) => &s.node,
            Self::DebuggerStatement(s) => &s.node,
            Self::Directive(s) => &s.node,
            Self::DoWhileStatement(s) => &s.node,
            Self::EmptyStatement(s) => &s.node,
            Self::EnumStatement(s) => &s.node,
            Self::ExpressionStatement(s) => &s.node,
            Self::ForInStatement(s) => &s.node,
            Self::ForStatement(s) => &s.node,
            Self::FunctionDeclaration(s) => &s.node,
            Self::IfStatement(s) => &s.node,
            Self::LabeledStatement(s) => &s.node,
            Self::ReturnStatement(s) => &s.node,
            Self::SwitchStatement(s) => &s.node,
            Self::ThrowStatement(s) => &s.node,
            Self::TryStatement(s) => &s.node,
            Self::VariableDeclaration(s) => &s.node,
            Self::WhileStatement(s) => &s.node,
            Self::WithStatement(s) => &s.node,
        }
    }
}

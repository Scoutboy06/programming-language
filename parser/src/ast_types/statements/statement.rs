use parser_derive::FromVariants;

use crate::ast_types::{
    declarations::declaration::Declaration,
    node_objects::Node,
    statements::{
        BlockStatement, BreakStatement, ContinueStatement, DebuggerStatement, Directive,
        DoWhileStatement, EmptyStatement, ExpressionStatement, ForInStatement, ForOfStatement,
        ForStatement, IfStatement, LabeledStatement, ReturnStatement, SwitchStatement,
        ThrowStatement, TryStatement, WhileStatement, WithStatement,
    },
};

#[derive(Debug, PartialEq, Clone, FromVariants)]
pub enum Statement {
    BlockStatement(Box<BlockStatement>),
    BreakStatement(Box<BreakStatement>),
    ContinueStatement(Box<ContinueStatement>),
    DebuggerStatement(Box<DebuggerStatement>),
    Declaration(Box<Declaration>),
    Directive(Box<Directive>),
    DoWhileStatement(Box<DoWhileStatement>),
    EmptyStatement(Box<EmptyStatement>),
    ExpressionStatement(Box<ExpressionStatement>),
    ForInStatement(Box<ForInStatement>),
    ForOfStatement(Box<ForOfStatement>),
    ForStatement(Box<ForStatement>),
    IfStatement(Box<IfStatement>),
    LabeledStatement(Box<LabeledStatement>),
    ReturnStatement(Box<ReturnStatement>),
    SwitchStatement(Box<SwitchStatement>),
    ThrowStatement(Box<ThrowStatement>),
    TryStatement(Box<TryStatement>),
    WhileStatement(Box<WhileStatement>),
    WithStatement(Box<WithStatement>),
}

impl Statement {
    pub fn node(&self) -> &Node {
        match self {
            Self::BlockStatement(s) => &s.node,
            Self::BreakStatement(s) => &s.node,
            Self::ContinueStatement(s) => &s.node,
            Self::DebuggerStatement(s) => &s.node,
            Self::Declaration(s) => &s.node(),
            Self::Directive(s) => &s.node,
            Self::DoWhileStatement(s) => &s.node,
            Self::EmptyStatement(s) => &s.node,
            Self::ExpressionStatement(s) => &s.node,
            Self::ForInStatement(s) => &s.node,
            Self::ForOfStatement(s) => &s.node,
            Self::ForStatement(s) => &s.node,
            Self::IfStatement(s) => &s.node,
            Self::LabeledStatement(s) => &s.node,
            Self::ReturnStatement(s) => &s.node,
            Self::SwitchStatement(s) => &s.node,
            Self::ThrowStatement(s) => &s.node,
            Self::TryStatement(s) => &s.node,
            Self::WhileStatement(s) => &s.node,
            Self::WithStatement(s) => &s.node,
        }
    }
}

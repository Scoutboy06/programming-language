use super::{
    ArrayExpression, ArrowFunctionExpression, AssignmentExpression, BinaryExpression,
    CallExpression, FunctionExpression, Literal, MemberExpression, ObjectExpression,
    ParenthesisExpression, UnaryExpression, UpdateExpression,
};
use crate::{
    expressions::{
        ConditionalExpression, LogicalExpression, NewExpression, SequenceExpression,
        TernaryExpression, ThisExpression, TypeofExpression,
    },
    nodes::Node,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    ArrayExpression(Box<ArrayExpression>),
    ArrowFunctionExpression(Box<ArrowFunctionExpression>),
    AssignmentExpression(Box<AssignmentExpression>),
    BinaryExpression(Box<BinaryExpression>),
    CallExpression(Box<CallExpression>),
    ConditionalExpression(Box<ConditionalExpression>),
    FunctionExpression(Box<FunctionExpression>),
    Literal(Box<Literal>),
    LogicalExpression(Box<LogicalExpression>),
    MemberExpression(Box<MemberExpression>),
    NewExpression(Box<NewExpression>),
    ObjectExpression(Box<ObjectExpression>),
    ParenthesisExpression(Box<ParenthesisExpression>),
    SequenceExpression(Box<SequenceExpression>),
    TernaryExpression(Box<TernaryExpression>),
    ThisExpression(Box<ThisExpression>),
    TypeofExpression(Box<TypeofExpression>),
    UnaryExpression(Box<UnaryExpression>),
    UpdateExpression(Box<UpdateExpression>),
}

impl Expression {
    pub fn node(&self) -> &Node {
        match self {
            Self::ArrayExpression(e) => &e.node,
            Self::ArrowFunctionExpression(e) => &e.node,
            Self::AssignmentExpression(e) => &e.node,
            Self::BinaryExpression(e) => &e.node,
            Self::CallExpression(e) => &e.node,
            Self::ConditionalExpression(e) => &e.node,
            Self::FunctionExpression(e) => &e.node,
            Self::Literal(e) => &e.node(),
            Self::LogicalExpression(e) => &e.node,
            Self::MemberExpression(e) => &e.node,
            Self::NewExpression(e) => &e.node,
            Self::ObjectExpression(e) => &e.node,
            Self::ParenthesisExpression(e) => &e.node,
            Self::SequenceExpression(e) => &e.node,
            Self::TernaryExpression(e) => &e.node,
            Self::ThisExpression(e) => &e.node,
            Self::TypeofExpression(e) => &e.node,
            Self::UnaryExpression(e) => &e.node,
            Self::UpdateExpression(e) => &e.node,
        }
    }
}

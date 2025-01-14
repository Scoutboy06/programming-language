use super::{
    ArrayExpression, AssignmentExpression, BinaryExpression, CallExpression, FunctionExpression,
    Literal, MemberExpression, ObjectExpression, ParenthesisExpression, UnaryExpression,
    UpdateExpression,
};
use crate::{nodes::Node, statements::Identifier};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    ParenthesisExpression(Box<ParenthesisExpression>),
    Literal(Box<Literal>),
    Identifier(Box<Identifier>),
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    AssignmentExpression(Box<AssignmentExpression>),
    LogicalExpression(Box<()>),
    ConditionalExpression(Box<()>),
    CallExpression(Box<CallExpression>),
    MemberExpression(Box<MemberExpression>),
    FunctionExpression(Box<FunctionExpression>),
    ArrowFunctionExpression(Box<()>),
    ObjectExpression(Box<ObjectExpression>),
    ArrayExpression(Box<ArrayExpression>),
    NewExpression(Box<()>),
    SequenceExpression(Box<()>),
    UpdateExpression(Box<UpdateExpression>),
    ThisExpression(Box<()>),
    SuperExpression(Box<()>),
    ClassExpression(Box<()>),
    TemplateLiteral(Box<()>),
    TaggedTemplateExpression(Box<()>),
    SpreadElement(Box<()>),
    YieldExpression(Box<()>),
    AwaitExpression(Box<()>),
    ImportExpression(Box<()>),
}

impl Expression {
    pub fn node(&self) -> &Node {
        match self {
            Self::ParenthesisExpression(e) => &e.node,
            Self::Literal(lit) => lit.node(),
            Self::Identifier(i) => &i.node,
            Self::BinaryExpression(b) => &b.node,
            Self::UnaryExpression(u) => &u.node,
            Self::AssignmentExpression(a) => &a.node,
            Self::LogicalExpression(_) => todo!("LogicalExpression"),
            Self::ConditionalExpression(_) => todo!("ConditionalExpression"),
            Self::CallExpression(c) => &c.node,
            Self::MemberExpression(m) => &m.node,
            Self::FunctionExpression(f) => &f.node,
            Self::ArrowFunctionExpression(_) => todo!("ArrowFunctionExpression"),
            Self::ObjectExpression(o) => &o.node,
            Self::ArrayExpression(a) => &a.node,
            Self::NewExpression(_) => todo!("NewExpression"),
            Self::SequenceExpression(_) => todo!("SequenceExpression"),
            Self::UpdateExpression(e) => &e.node,
            Self::ThisExpression(_) => todo!("ThisExpression"),
            Self::SuperExpression(_) => todo!("SuperExpression"),
            Self::ClassExpression(_) => todo!("ClassExpression"),
            Self::TemplateLiteral(_) => todo!("TemplateLiteral"),
            Self::TaggedTemplateExpression(_) => todo!("TaggedTemplateExpression"),
            Self::SpreadElement(_) => todo!("SpreadElement"),
            Self::YieldExpression(_) => todo!("YieldExpression"),
            Self::AwaitExpression(_) => todo!("AwaitExpression"),
            Self::ImportExpression(_) => todo!("ImportExpression"),
        }
    }
}

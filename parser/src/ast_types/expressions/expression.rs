use parser_derive::FromVariants;

use crate::ast_types::{
    classes::meta_property::MetaProperty,
    expressions::{
        ArrayExpression, ArrowFunctionExpression, AssignmentExpression, BinaryExpression,
        CallExpression, ConditionalExpression, FunctionExpression, LogicalExpression,
        MemberExpression, NewExpression, ObjectExpression, SequenceExpression, Super,
        TaggedTemplateExpression, TemplateLiteral, ThisExpression, UnaryExpression,
        UpdateExpression, YieldExpression,
    },
    identifier::Identifier,
    literal::Literal,
    node_objects::Node,
};

#[derive(Debug, PartialEq, Clone, FromVariants)]
pub enum Expression {
    ArrayExpression(Box<ArrayExpression>),
    ArrowFunctionExpression(Box<ArrowFunctionExpression>),
    AssignmentExpression(Box<AssignmentExpression>),
    BinaryExpression(Box<BinaryExpression>),
    CallExpression(Box<CallExpression>),
    ConditionalExpression(Box<ConditionalExpression>),
    FunctionExpression(Box<FunctionExpression>),
    Identifier(Box<Identifier>),
    Literal(Box<Literal>),
    LogicalExpression(Box<LogicalExpression>),
    MemberExpression(Box<MemberExpression>),
    MetaProperty(Box<MetaProperty>),
    NewExpression(Box<NewExpression>),
    ObjectExpression(Box<ObjectExpression>),
    SequenceExpression(Box<SequenceExpression>),
    Super(Box<Super>),
    TaggedTemplateExpression(Box<TaggedTemplateExpression>),
    TemplateLiteral(Box<TemplateLiteral>),
    ThisExpression(Box<ThisExpression>),
    UnaryExpression(Box<UnaryExpression>),
    UpdateExpression(Box<UpdateExpression>),
    YieldExpression(Box<YieldExpression>),
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
            Self::Identifier(e) => &e.node,
            Self::Literal(e) => &e.node,
            Self::LogicalExpression(e) => &e.node,
            Self::MemberExpression(e) => &e.node,
            Self::MetaProperty(e) => &e.node,
            Self::NewExpression(e) => &e.node,
            Self::ObjectExpression(e) => &e.node,
            Self::SequenceExpression(e) => &e.node,
            Self::Super(e) => &e.node,
            Self::TaggedTemplateExpression(e) => &e.node,
            Self::TemplateLiteral(e) => &e.node,
            Self::ThisExpression(e) => &e.node,
            Self::UnaryExpression(e) => &e.node,
            Self::UpdateExpression(e) => &e.node,
            Self::YieldExpression(e) => &e.node,
        }
    }
}

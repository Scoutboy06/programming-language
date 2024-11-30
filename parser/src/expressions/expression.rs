use super::{
    AssignmentExpression, BinaryExpression, CallExpression, Literal, MemberExpression,
    UnaryExpression,
};
use crate::statements::Identifier;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Box<Literal>),
    Identifier(Box<Identifier>),
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    AssignmentExpression(Box<AssignmentExpression>),
    LogicalExpression(Box<()>),
    ConditionalExpression(Box<()>),
    CallExpression(Box<CallExpression>),
    MemberExpression(Box<MemberExpression>),
    FunctionExpression(Box<()>),
    ArrowFunctionExpression(Box<()>),
    ObjectExpression(Box<()>),
    ArrayExpression(Box<()>),
    NewExpression(Box<()>),
    SequenceExpression(Box<()>),
    UpdateExpression(Box<()>),
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

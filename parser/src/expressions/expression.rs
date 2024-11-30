use super::{AssignmentExpression, BinaryExpression, Literal, UnaryExpression};
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
    CallExpression(Box<()>),
    MemberExpression(Box<()>),
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

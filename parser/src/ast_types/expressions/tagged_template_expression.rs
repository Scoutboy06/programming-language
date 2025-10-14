use super::Expression;
use crate::{expressions::TemplateLiteral, nodes::Node};
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct TaggedTemplateExpression {
    pub node: Node,
    pub tag: Expression,
    pub quasi: TemplateLiteral,
}

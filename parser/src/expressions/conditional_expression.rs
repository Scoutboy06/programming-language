use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct ConditionalExpression {
    pub node: Node,
    pub test: Expression,
    pub alternate: Expression,
    pub consequent: Expression,
}

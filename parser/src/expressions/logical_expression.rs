use super::Expression;
use crate::nodes::Node;
use lexer::LogicalOperator;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct LogicalExpression {
    pub node: Node,
    pub operator: LogicalOperator,
    pub left: Expression,
    pub right: Expression,
}

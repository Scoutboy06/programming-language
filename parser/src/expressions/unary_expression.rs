use super::Expression;
use crate::nodes::Node;
use lexer::UnaryOperator;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct UnaryExpression {
    pub node: Node,
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Expression,
}

use super::Expression;
use crate::nodes::Node;
use lexer::UpdateOperator;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct UpdateExpression {
    pub node: Node,
    pub operator: UpdateOperator,
    pub argument: Expression,
    pub prefix: bool,
}

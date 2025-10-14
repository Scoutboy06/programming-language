use crate::ast_types::node_objects::Node;

use super::Expression;
use lexer::LogicalOperator;
use parser_derive::Expr;

// es5
// interface LogicalExpression <: Expression {
//     type: "LogicalExpression";
//     operator: LogicalOperator;
//     left: Expression;
//     right: Expression;
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct LogicalExpression {
    pub node: Node,
    pub operator: LogicalOperator,
    pub left: Expression,
    pub right: Expression,
}

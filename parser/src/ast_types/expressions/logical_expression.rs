use crate::ast_types::{expressions::Expression, node_objects::Node, operators::LogicalOperator};

// es5
// interface LogicalExpression <: Expression {
//     type: "LogicalExpression";
//     operator: LogicalOperator;
//     left: Expression;
//     right: Expression;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct LogicalExpression {
    pub node: Node,
    pub operator: LogicalOperator,
    pub left: Expression,
    pub right: Expression,
}

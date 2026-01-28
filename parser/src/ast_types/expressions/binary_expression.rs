use crate::ast_types::{node_objects::Node, operators::BinaryOperator};

use super::Expression;

// es5
// interface BinaryExpression <: Expression {
//     type: "BinaryExpression";
//     operator: BinaryOperator;
//     left: Expression;
//     right: Expression;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression {
    pub node: Node,
    pub operator: BinaryOperator,
    pub left: Expression,
    pub right: Expression,
}

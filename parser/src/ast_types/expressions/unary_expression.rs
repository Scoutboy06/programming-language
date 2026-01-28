use super::Expression;
use crate::ast_types::{node_objects::Node, operators::UnaryOperator};

// es5
// interface UnaryExpression <: Expression {
//     type: "UnaryExpression";
//     operator: UnaryOperator;
//     prefix: boolean;
//     argument: Expression;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression {
    pub node: Node,
    pub operator: UnaryOperator,
    pub prefix: bool,
    pub argument: Expression,
}

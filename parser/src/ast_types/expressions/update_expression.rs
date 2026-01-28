use crate::ast_types::{node_objects::Node, operators::UpdateOperator};

use super::Expression;

// es5
// interface UpdateExpression <: Expression {
//     type: "UpdateExpression";
//     operator: UpdateOperator;
//     argument: Expression;
//     prefix: boolean;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateExpression {
    pub node: Node,
    pub operator: UpdateOperator,
    pub argument: Expression,
    pub prefix: bool,
}

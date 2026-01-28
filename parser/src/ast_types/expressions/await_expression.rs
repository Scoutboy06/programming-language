use crate::ast_types::{expressions::Expression, node_objects::Node};

// es2017
// interface AwaitExpression <: Expression {
//     type: "AwaitExpression";
//     argument: Expression;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct AwaitExpression {
    pub node: Node,
    pub argument: Expression,
}

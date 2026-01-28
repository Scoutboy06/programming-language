use crate::ast_types::{expressions::Expression, node_objects::Node};

// es5
// interface SequenceExpression <: Expression {
//     type: "SequenceExpression";
//     expressions: [ Expression ];
// }
#[derive(Debug, PartialEq, Clone)]
pub struct SequenceExpression {
    pub node: Node,
    pub expressions: Vec<Expression>,
}

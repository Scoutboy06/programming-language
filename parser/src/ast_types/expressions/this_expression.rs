use crate::ast_types::node_objects::Node;

// es5
// interface ThisExpression <: Expression {
//     type: "ThisExpression";
// }
#[derive(Debug, PartialEq, Clone)]
pub struct ThisExpression {
    pub node: Node,
}

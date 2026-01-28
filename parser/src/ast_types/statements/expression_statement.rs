use crate::ast_types::{expressions::Expression, node_objects::Node};

// es5
// interface ExpressionStatement <: Statement {
//     type: "ExpressionStatement";
//     expression: Expression;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub node: Node,
    pub expression: Expression,
}

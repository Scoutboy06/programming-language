use crate::ast_types::{expressions::Expression, node_objects::Node};

// es2020
// interface ImportExpression <: Expression {
//   type: "ImportExpression";
//   source: Expression;
// }
pub struct ImportExpression {
    pub node: Node,
    pub source: Expression,
}

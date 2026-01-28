use crate::ast_types::{expressions::Expression, node_objects::Node};

// es5
// interface ReturnStatement <: Statement {
//     type: "ReturnStatement";
//     argument: Expression | null;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub node: Node,
    pub argument: Option<Expression>,
}

use crate::ast_types::{expressions::Expression, node_objects::Node, statements::Statement};

// es5
// interface WithStatement <: Statement {
//     type: "WithStatement";
//     object: Expression;
//     body: Statement;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct WithStatement {
    pub node: Node,
    pub object: Expression,
    pub body: Statement,
}

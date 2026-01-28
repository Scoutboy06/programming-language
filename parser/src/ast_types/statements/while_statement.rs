use crate::ast_types::{expressions::Expression, node_objects::Node, statements::Statement};

// es5
// interface WhileStatement <: Statement {
//     type: "WhileStatement";
//     test: Expression;
//     body: Statement;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub node: Node,
    pub test: Expression,
    pub body: Statement,
}

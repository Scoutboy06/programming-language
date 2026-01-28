use crate::ast_types::{expressions::Expression, node_objects::Node, statements::Statement};

// es5
// interface DoWhileStatement <: Statement {
//     type: "DoWhileStatement";
//     body: Statement;
//     test: Expression;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct DoWhileStatement {
    pub node: Node,
    pub body: Statement,
    pub test: Expression,
}

use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface WhileStatement <: Statement {
//     type: "WhileStatement";
//     test: Expression;
//     body: Statement;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct WhileStatement {
    pub node: Node,
    pub test: Expression,
    pub body: Statement,
}

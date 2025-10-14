use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface DoWhileStatement <: Statement {
//     type: "DoWhileStatement";
//     body: Statement;
//     test: Expression;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct DoWhileStatement {
    pub node: Node,
    pub body: Statement,
    pub test: Expression,
}

use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface EmptyStatement <: Statement {
//     type: "EmptyStatement";
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct EmptyStatement {
    pub node: Node,
}

use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface DebuggerStatement <: Statement {
//     type: "DebuggerStatement";
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct DebuggerStatement {
    pub node: Node,
}

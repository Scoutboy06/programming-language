use crate::ast_types::{identifier::Identifier, node_objects::Node};

use super::Statement;
use parser_derive::Stmt;

// es5
// interface BreakStatement <: Statement {
//     type: "BreakStatement";
//     label: Identifier | null;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct BreakStatement {
    pub node: Node,
    pub label: Option<Identifier>,
}

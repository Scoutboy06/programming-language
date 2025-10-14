use crate::ast_types::{identifier::Identifier, node_objects::Node};

use super::Statement;
use parser_derive::Stmt;

// es5
// interface ContinueStatement <: Statement {
//     type: "ContinueStatement";
//     label: Identifier | null;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ContinueStatement {
    pub node: Node,
    pub label: Option<Identifier>,
}

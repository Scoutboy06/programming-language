use crate::ast_types::{identifier::Identifier, node_objects::Node};

use super::Statement;
use parser_derive::Stmt;

// es5
// interface LabeledStatement <: Statement {
//     type: "LabeledStatement";
//     label: Identifier;
//     body: Statement;
// }
#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct LabeledStatement {
    pub node: Node,
    pub label: Identifier,
    pub body: Statement,
}

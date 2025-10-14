use crate::ast_types::{expressions::Expression, node_objects::Node, statements::ForInOrOfLeft};

use super::Statement;
use parser_derive::Stmt;

// es2015
// interface ForOfStatement <: ForInStatement {
//     type: "ForOfStatement";
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForOfStatement {
    pub node: Node,
    pub left: ForInOrOfLeft,
    pub right: Expression,
    pub body: Statement,
}

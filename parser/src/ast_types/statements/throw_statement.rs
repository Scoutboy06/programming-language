use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface ThrowStatement <: Statement {
//     type: "ThrowStatement";
//     argument: Expression;
// }
#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct ThrowStatement {
    pub node: Node,
    pub argument: Expression,
}

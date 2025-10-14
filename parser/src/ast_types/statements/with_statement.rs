use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface WithStatement <: Statement {
//     type: "WithStatement";
//     object: Expression;
//     body: Statement;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct WithStatement {
    pub node: Node,
    pub object: Expression,
    pub body: Statement,
}

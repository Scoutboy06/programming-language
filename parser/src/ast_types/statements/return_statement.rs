use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface ReturnStatement <: Statement {
//     type: "ReturnStatement";
//     argument: Expression | null;
// }
#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct ReturnStatement {
    pub node: Node,
    pub argument: Option<Expression>,
}

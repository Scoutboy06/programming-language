use super::Statement;
use crate::ast_types::node_objects::Node;
use parser_derive::Stmt;

// es5
// interface ExpressionStatement <: Statement {
//     type: "ExpressionStatement";
//     expression: Expression;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ExpressionStatement {
    pub node: Node,
    pub expression: Expression,
}

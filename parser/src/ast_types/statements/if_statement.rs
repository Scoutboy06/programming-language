use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface IfStatement <: Statement {
//     type: "IfStatement";
//     test: Expression;
//     consequent: Statement;
//     alternate: Statement | null;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct IfStatement {
    pub node: Node,
    pub test: Expression,
    pub consequent: Statement,
    pub alternate: Option<Statement>,
}

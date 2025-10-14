use crate::ast_types::node_objects::Node;

use super::Statement;
use parser_derive::Stmt;

// es5
// interface SwitchStatement <: Statement {
//     type: "SwitchStatement";
//     discriminant: Expression;
//     cases: [ SwitchCase ];
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct SwitchStatement {
    pub node: Node,
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    node: Node,
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

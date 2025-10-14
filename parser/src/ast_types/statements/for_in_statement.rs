use parser_derive::Stmt;

use super::Statement;
use crate::ast_types::node_objects::Node;

// es5
// interface ForInStatement <: Statement {
//     type: "ForInStatement";
//     left: VariableDeclaration |  Pattern;
//     right: Expression;
//     body: Statement;
// }
#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForInStatement {
    pub node: Node,
    pub left: ForInOrOfLeft,
    pub right: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInOrOfLeft {
    VariableDeclaration(VariableDeclaration),
    Pattern(Pattern),
}

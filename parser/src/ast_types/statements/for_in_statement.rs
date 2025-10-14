use super::Statement;
use parser_derive::Stmt;

use crate::ast_types::{
    declarations::variable_declaration::VariableDeclaration, expressions::Expression,
    node_objects::Node, patterns::pattern::Pattern,
};

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

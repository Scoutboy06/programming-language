use parser_derive::Stmt;

use super::Statement;
use crate::{
    expressions::Expression,
    impl_from,
    nodes::{Node, Pattern},
    statements::VariableDeclaration,
};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForInStatement {
    pub node: Node,
    pub left: ForInLeft,
    pub right: Expression,
    pub body: Statement,
}

pub enum ForInLeft {
    VariableDeclaration(VariableDeclaration),
    Pattern(Pattern),
}

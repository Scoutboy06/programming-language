use parser_derive::Stmt;

use super::Statement;
use crate::{
    expressions::{Expression, Identifier},
    nodes::Node,
    statements::VariableDeclaration,
};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForInStatement {
    pub node: Node,
    pub variable: ForInVariable,
    pub object: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInVariable {
    VariableDeclaration(VariableDeclaration),
    Identifier(Identifier),
}

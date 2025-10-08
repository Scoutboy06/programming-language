use parser_derive::Stmt;

use super::Statement;
use crate::{
    expressions::{Expression, Identifier},
    nodes::Node,
    statements::VariableDeclaration,
};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForOfStatement {
    pub node: Node,
    pub variable: ForOfVariable,
    pub iterable: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForOfVariable {
    VariableDeclaration(VariableDeclaration),
    Identifier(Identifier),
}

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
    pub left: ForInVariable,
    pub right: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInVariable {
    VariableDeclaration(VariableDeclaration),
    Identifier(Identifier),
}

impl From<VariableDeclaration> for ForInVariable {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}

impl From<Identifier> for ForInVariable {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}

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
    pub left: ForOfVariable,
    pub right: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForOfVariable {
    VariableDeclaration(VariableDeclaration),
    Identifier(Identifier),
}

impl From<VariableDeclaration> for ForOfVariable {
    fn from(value: VariableDeclaration) -> Self {
        Self::VariableDeclaration(value)
    }
}
impl From<Identifier> for ForOfVariable {
    fn from(value: Identifier) -> Self {
        Self::Identifier(value)
    }
}

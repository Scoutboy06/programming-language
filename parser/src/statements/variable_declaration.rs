use crate::expressions::Expression;
use crate::nodes::Node;
use string_cache::DefaultAtom as Atom;

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

impl From<VariableDeclaration> for Statement {
    fn from(value: VariableDeclaration) -> Self {
        Statement::VariableDeclaration(Box::new(value))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarator {
    pub node: Node,
    pub id: Identifier,
    pub init: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub node: Node,
    pub name: Atom,
}

impl From<Identifier> for Expression {
    fn from(value: Identifier) -> Self {
        Expression::Identifier(Box::new(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

use string_cache::DefaultAtom as Atom;

use crate::{
    lexer::{Keyword, Kind, Span},
    parser::{Expression, Node},
};

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

#[derive(Debug, PartialEq)]
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

impl Identifier {
    pub fn new(name: Atom, start: usize, end: usize) -> Self {
        Self {
            node: Node::new(start, end),
            name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

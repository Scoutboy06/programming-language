use crate::expressions::types::TypeAnnotation;
use crate::expressions::{Expression, Identifier, VariableKind};
use crate::nodes::{Node, Pattern};
use parser_derive::Stmt;

use super::Statement;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct VariableDeclaration {
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
    pub kind: VariableKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclarator {
    pub node: Node,
    pub id: Pattern,
    pub type_annotation: Option<TypeAnnotation>,
    pub init: Option<Expression>,
}

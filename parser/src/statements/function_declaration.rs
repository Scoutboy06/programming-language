use super::Statement;
use crate::{
    expressions::{
        types::{TypeAnnotation, TypeParameterDeclaration},
        Identifier,
    },
    nodes::{Node, Pattern},
    statements::FunctionBody,
};
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct FunctionDeclaration {
    pub node: Node,
    pub id: Identifier,
    pub type_parameters: Option<TypeParameterDeclaration>,
    pub params: Vec<Pattern>,
    pub return_type: Option<TypeAnnotation>,
    pub body: FunctionBody,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub node: Node,
    pub identifier: Identifier,
    pub type_annotation: Option<TypeAnnotation>,
    pub optional: bool,
}

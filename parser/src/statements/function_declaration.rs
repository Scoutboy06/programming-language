use crate::{expressions::TypeAnnotation, nodes::Node};

use super::{BlockStatement, Identifier};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: BlockStatement,
    pub is_expression: bool,
    pub is_generator: bool,
    pub is_async: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub node: Node,
    pub identifier: Identifier,
    pub type_annotation: TypeAnnotation,
    pub optional: bool,
}

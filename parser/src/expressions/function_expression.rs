use super::{types::TypeAnnotation, Expression, Identifier};
use crate::{
    nodes::{Node, Pattern},
    statements::{BlockStatement, FunctionBody, Parameter},
};
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct FunctionExpression {
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub return_type: Option<TypeAnnotation>,
    pub body: FunctionBody,
}

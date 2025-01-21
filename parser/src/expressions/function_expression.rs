use super::{types::TypeAnnotation, Expression, Identifier};
use crate::{
    nodes::Node,
    statements::{BlockStatement, Parameter},
};
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct FunctionExpression {
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: BlockStatement,
    pub is_generator: bool,
    pub is_async: bool,
}

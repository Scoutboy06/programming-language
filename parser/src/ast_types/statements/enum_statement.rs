use super::Statement;
use crate::{
    expressions::{Expression, Identifier},
    nodes::Node,
};
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct EnumStatement {
    pub node: Node,
    pub is_declare: bool,
    pub is_const: bool,
    pub id: Identifier,
    pub members: Vec<EnumMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumMember {
    pub node: Node,
    pub id: Identifier,
    pub init: Option<Expression>,
}

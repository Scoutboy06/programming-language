use super::{Identifier, Statement};
use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumStatement {
    pub node: Node,
    pub is_declare: bool,
    pub is_const: bool,
    pub id: Identifier,
    pub members: Vec<EnumMember>,
}

impl From<EnumStatement> for Statement {
    fn from(value: EnumStatement) -> Self {
        Statement::EnumStatement(Box::new(value))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumMember {
    pub node: Node,
    pub id: Identifier,
    pub init: Option<Expression>,
}

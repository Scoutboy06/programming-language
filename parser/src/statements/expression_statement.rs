use super::Statement;
use crate::{expressions::Expression, nodes::Node};
use parser_derive::Stmt;

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ExpressionStatement {
    pub node: Node,
    pub expression: Expression,
}

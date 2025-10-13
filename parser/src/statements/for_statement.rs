use parser_derive::Stmt;

use super::Statement;
use crate::{expressions::Expression, nodes::Node, statements::VariableDeclaration};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForStatement {
    pub node: Node,
    pub init: Option<ForInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

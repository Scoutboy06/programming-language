use parser_derive::Stmt;

use super::Statement;
use crate::{expressions::Expression, nodes::Node};

#[derive(Debug, Clone, PartialEq, Stmt)]
pub struct ForStatement {
    pub node: Node,
    pub initializer: Option<Statement>,
    pub condition: Option<Expression>,
    pub update: Option<Statement>,
    pub body: Statement,
}

use super::Statement;
use crate::{
    nodes::{Node, Pattern},
    statements::BlockStatement,
};
use parser_derive::Stmt;

#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct TryStatement {
    pub node: Node,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CatchClause {
    pub node: Node,
    pub param: Pattern,
    pub body: BlockStatement,
}

use super::Statement;
use crate::ast_types::{
    node_objects::Node, patterns::pattern::Pattern, statements::BlockStatement,
};
use parser_derive::Stmt;

// es5
// interface TryStatement <: Statement {
//     type: "TryStatement";
//     block: BlockStatement;
//     handler: CatchClause | null;
//     finalizer: BlockStatement | null;
// }
#[derive(Debug, PartialEq, Clone, Stmt)]
pub struct TryStatement {
    pub node: Node,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

// es5
// interface CatchClause <: Node {
//     type: "CatchClause";
//     param: Pattern;
//     body: BlockStatement;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct CatchClause {
    pub node: Node,
    pub param: Pattern,
    pub body: BlockStatement,
}

use crate::ast_types::{identifier::Identifier, node_objects::Node};

// es5
// interface ContinueStatement <: Statement {
//     type: "ContinueStatement";
//     label: Identifier | null;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStatement {
    pub node: Node,
    pub label: Option<Identifier>,
}

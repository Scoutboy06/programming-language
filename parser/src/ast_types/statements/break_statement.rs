use crate::ast_types::{identifier::Identifier, node_objects::Node};

// es5
// interface BreakStatement <: Statement {
//     type: "BreakStatement";
//     label: Identifier | null;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement {
    pub node: Node,
    pub label: Option<Identifier>,
}

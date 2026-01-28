use crate::ast_types::node_objects::Node;

// es5
// interface EmptyStatement <: Statement {
//     type: "EmptyStatement";
// }
#[derive(Debug, Clone, PartialEq)]
pub struct EmptyStatement {
    pub node: Node,
}

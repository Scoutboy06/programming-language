use crate::ast_types::node_objects::Node;

// es5
// interface DebuggerStatement <: Statement {
//     type: "DebuggerStatement";
// }
#[derive(Debug, Clone, PartialEq)]
pub struct DebuggerStatement {
    pub node: Node,
}

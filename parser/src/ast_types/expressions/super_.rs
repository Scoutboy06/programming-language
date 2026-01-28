use crate::ast_types::node_objects::Node;

// es2015
// interface Super <: Node {
//     type: "Super";
// }
#[derive(Debug, PartialEq, Clone)]
pub struct Super {
    pub node: Node,
}

use crate::ast_types::{expressions::Expression, node_objects::Node};

// es2015
// interface SpreadElement <: Node {
//     type: "SpreadElement";
//     argument: Expression;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct SpreadElement {
    pub node: Node,
    pub argument: Expression,
}

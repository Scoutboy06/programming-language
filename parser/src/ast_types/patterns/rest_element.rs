use crate::ast_types::{node_objects::Node, patterns::pattern::Pattern};

// es2015
// interface RestElement <: Pattern {
//     type: "RestElement";
//     argument: Pattern;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct RestElement {
    pub node: Node,
    pub argument: Pattern,
}

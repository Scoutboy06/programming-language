use crate::ast_types::{node_objects::Node, patterns::pattern::Pattern};

// es2015
// interface ArrayPattern <: Pattern {
//     type: "ArrayPattern";
//     elements: [ Pattern | null ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPattern {
    pub node: Node,
    pub elements: Vec<Option<Pattern>>,
}

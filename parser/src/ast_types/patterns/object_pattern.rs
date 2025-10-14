use crate::ast_types::{expressions::Expression, node_objects::Node, patterns::pattern::Pattern};

// es2015
// interface ObjectPattern <: Pattern {
//     type: "ObjectPattern";
//     properties: [ AssignmentProperty ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPattern {
    pub properties: Vec<AssignmentProperty>,
}

// es2015
// interface AssignmentProperty <: Property {
//     type: "Property"; // inherited
//     value: Pattern;
//     kind: "init";
//     method: false;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentProperty {
    pub node: Node,
    pub key: Expression,
    pub value: Pattern,
    pub shorthand: bool,
    pub computed: bool,
}

use crate::ast_types::{
    expressions::Expression,
    node_objects::Node,
    patterns::{pattern::Pattern, rest_element::RestElement},
};

// es2015
// interface ObjectPattern <: Pattern {
//     type: "ObjectPattern";
//     properties: [ AssignmentProperty ];
// }
//
// es2018
// extend interface ObjectPattern {
//     properties: [ AssignmentProperty | RestElement ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPattern {
    pub node: Node,
    pub properties: Vec<AssignmentPropertyProperty>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentPropertyProperty {
    AssignmentProperty(AssignmentProperty),
    RestElement(RestElement),
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

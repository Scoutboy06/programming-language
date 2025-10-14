use crate::ast_types::{expressions::Expression, node_objects::Node};

// es5
// interface Property <: Node {
//     type: "Property";
//     key: Literal | Identifier;
//     value: Expression;
//     kind: "init" | "get" | "set";
// }
//
// es2015
// extend interface Property {
//     key: Expression;
//     method: boolean;
//     shorthand: boolean;
//     computed: boolean;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub node: Node,
    pub key: Expression,
    pub value: Expression,
    pub kind: PropertyKind,
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}

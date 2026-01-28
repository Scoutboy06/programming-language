use crate::ast_types::{identifier::Identifier, node_objects::Node};

// es2015
// interface MetaProperty <: Expression {
//     type: "MetaProperty";
//     meta: Identifier;
//     property: Identifier;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct MetaProperty {
    pub node: Node,
    pub meta: Identifier,
    pub property: Identifier,
}

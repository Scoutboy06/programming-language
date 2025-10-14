use crate::ast_types::{expressions::Expression, identifier::Identifier, node_objects::Node};
use parser_derive::Expr;

// es2015
// interface MetaProperty <: Expression {
//     type: "MetaProperty";
//     meta: Identifier;
//     property: Identifier;
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct MetaProperty {
    pub node: Node,
    pub meta: Identifier,
    pub property: Identifier,
}

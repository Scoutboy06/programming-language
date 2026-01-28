use parser_derive::FromVariants;

use crate::ast_types::{node_objects::Node, property::Property, spread_element::SpreadElement};

// es5
// interface ObjectExpression <: Expression {
//     type: "ObjectExpression";
//     properties: [ Property ];
// }
//
// es2018
// extend interface ObjectExpression {
//     properties: [ Property | SpreadElement ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExpression {
    pub node: Node,
    pub properties: Vec<ObjectExpressionProperty>,
}

#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum ObjectExpressionProperty {
    Property(Property),
    SpreadElement(SpreadElement),
}

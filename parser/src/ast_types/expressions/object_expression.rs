use parser_derive::Expr;

use crate::ast_types::{expressions::Expression, node_objects::Node, property::Property};

// es5
// interface ObjectExpression <: Expression {
//     type: "ObjectExpression";
//     properties: [ Property ];
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ObjectExpression {
    pub node: Node,
    pub properties: Vec<Property>,
}

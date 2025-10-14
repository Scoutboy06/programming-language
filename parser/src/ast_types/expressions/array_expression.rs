use crate::ast_types::{node_objects::Node, spread_element::SpreadElement};

use super::Expression;
use parser_derive::Expr;

// es5
// interface ArrayExpression <: Expression {
//     type: "ArrayExpression";
//     elements: [ Expression | null ];
// }
//
// es2015
// extend interface ArrayExpression {
//     elements: [ Expression | SpreadElement | null ];
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ArrayExpression {
    pub node: Node,
    pub elements: Vec<Option<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayElement {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

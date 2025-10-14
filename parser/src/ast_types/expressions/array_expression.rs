use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es5
// interface ArrayExpression <: Expression {
//     type: "ArrayExpression";
//     elements: [ Expression | null ];
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ArrayExpression {
    pub node: Node,
    pub elements: Vec<Option<Expression>>,
}

pub enum ArrayElement {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

pub struct ArrayPattern {
    pub node: Node,
    pub elements: Vec<Option<Pattern>>,
}

use crate::ast_types::{node_objects::Node, spread_element::SpreadElement};

use super::Expression;
use parser_derive::Expr;

// es5
// interface NewExpression <: Expression {
//     type: "NewExpression";
//     callee: Expression;
//     arguments: [ Expression ];
// }
//
// es2015
// extend interface NewExpression {
//     arguments: [ Expression | SpreadElement ];
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct NewExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Vec<NewExpressionArgument>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NewExpressionArgument {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

use crate::ast_types::{expressions::Super, node_objects::Node};

use super::Expression;
use parser_derive::Expr;

// es5
// interface CallExpression <: Expression {
//     type: "CallExpression";
//     callee: Expression;
//     arguments: [ Expression ];
// }
//
// es2015
// extend interface CallExpression {
//     callee: Expression | Super;
//     arguments: [ Expression | SpreadElement ];
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct CallExpression {
    pub node: Node,
    pub callee: CallExpressionCallee,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallExpressionCallee {
    Expression(Expression),
    Super(Super),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallExpressionArgument {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

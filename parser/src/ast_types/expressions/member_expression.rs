use crate::ast_types::{expressions::Super, node_objects::Node};

use super::Expression;
use parser_derive::Expr;

// es5
// interface MemberExpression <: Expression, Pattern {
//     type: "MemberExpression";
//     object: Expression;
//     property: Expression;
//     computed: boolean;
// }
//
// es2015
// extend interface MemberExpression {
//     object: Expression | Super;
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: Expression,
    pub computed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberExpressionObject {
    Expression(Expression),
    Super(Super),
}

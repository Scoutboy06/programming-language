use parser_derive::FromVariants;

use crate::ast_types::{
    expressions::{Expression, Super},
    node_objects::Node,
    spread_element::SpreadElement,
};

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
//
// es2020
// extend interface CallExpression <: ChainElement {}
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub node: Node,
    pub optional: bool,
    pub callee: CallExpressionCallee,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum CallExpressionCallee {
    Expression(Expression),
    Super(Super),
}

impl CallExpressionCallee {
    pub fn node(&self) -> &Node {
        match self {
            CallExpressionCallee::Expression(expr) => expr.node(),
            CallExpressionCallee::Super(sup) => &sup.node,
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum CallExpressionArgument {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

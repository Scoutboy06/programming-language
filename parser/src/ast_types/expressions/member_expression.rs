use parser_derive::FromVariants;

use crate::ast_types::{
    classes::class::PrivateIdentifier,
    expressions::{Expression, Super},
    node_objects::Node,
};

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
//
// es2020
// extend interface MemberExpression <: ChainElement {}
//
// es2022
// extend interface MemberExpression {
//     property: Expression | PrivateIdentifier;
// }
#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression {
    pub node: Node,
    pub optional: bool,
    pub object: Expression,
    pub property: MemberExpressionProperty,
    pub computed: bool,
}

#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum MemberExpressionObject {
    Expression(Expression),
    Super(Super),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberExpressionProperty {
    Expression(Expression),
    PrivateIdentifier(PrivateIdentifier),
}

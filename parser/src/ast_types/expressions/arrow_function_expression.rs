use parser_derive::FromVariants;

use crate::ast_types::{
    expressions::Expression, identifier::Identifier, node_objects::Node,
    patterns::pattern::Pattern, statements::FunctionBody, types::TypeAnnotation,
};

// es2015
// interface ArrowFunctionExpression <: Function, Expression {
//     type: "ArrowFunctionExpression";
//     body: FunctionBody | Expression;
//     expression: boolean;
//     generator: false;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ArrowFunctionExpression {
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub return_type: Option<TypeAnnotation>,
    pub body: ArrowFunctionExpressionBody,
    pub expression: bool,
}

#[derive(Debug, Clone, PartialEq, FromVariants)]
pub enum ArrowFunctionExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Expression),
}

impl ArrowFunctionExpressionBody {
    pub fn node(&self) -> &Node {
        match self {
            ArrowFunctionExpressionBody::FunctionBody(body) => &body.node,
            ArrowFunctionExpressionBody::Expression(expr) => expr.node(),
        }
    }
}

use parser_derive::Expr;

use crate::ast_types::{
    declarations::function_declaration::Parameter, expressions::Expression, identifier::Identifier,
    node_objects::Node, statements::FunctionBody,
};

// es2015
// interface ArrowFunctionExpression <: Function, Expression {
//     type: "ArrowFunctionExpression";
//     body: FunctionBody | Expression;
//     expression: boolean;
//     generator: false;
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct ArrowFunctionExpression {
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Parameter>,
    pub return_type: Option<TypeAnnotation>,
    pub body: ArrowFunctionExpressionBody,
    pub expression: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrowFunctionExpressionBody {
    FunctionBody(FunctionBody),
    Expression(Expression),
}

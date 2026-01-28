use crate::ast_types::{
    identifier::Identifier, node_objects::Node, patterns::pattern::Pattern,
    statements::FunctionBody, types::TypeAnnotation,
};

// es5
// interface FunctionExpression <: Function, Expression {
//     type: "FunctionExpression";
// }
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionExpression {
    pub node: Node,
    pub id: Option<Identifier>,
    pub generator: bool,
    pub params: Vec<Pattern>,
    pub return_type: Option<TypeAnnotation>,
    pub body: FunctionBody,
}

use crate::ast_types::{node_objects::Node, template_element::TemplateElement};

use super::Expression;
use parser_derive::Expr;

// es2015
// interface TemplateLiteral <: Expression {
//     type: "TemplateLiteral";
//     quasis: [ TemplateElement ];
//     expressions: [ Expression ];
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct TemplateLiteral {
    pub node: Node,
    pub quasis: Vec<TemplateElement>,
    pub expression: Vec<Expression>,
}

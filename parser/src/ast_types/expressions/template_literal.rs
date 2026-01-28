use crate::ast_types::{node_objects::Node, template_element::TemplateElement};

use super::Expression;

// es2015
// interface TemplateLiteral <: Expression {
//     type: "TemplateLiteral";
//     quasis: [ TemplateElement ];
//     expressions: [ Expression ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateLiteral {
    pub node: Node,
    pub quasis: Vec<TemplateElement>,
    pub expression: Vec<Expression>,
}

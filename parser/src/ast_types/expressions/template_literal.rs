use super::Expression;
use crate::nodes::Node;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct TemplateLiteral {
    pub node: Node,
    pub quasis: Vec<TemplateElement>,
    pub expression: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement {
    pub node: Node,
    pub tail: bool,
    pub value: TemplateElementValue,
}

#[derive(Debug, Clone, PartialEq)]
struct TemplateElementValue {
    pub cooked: String,
    pub raw: String,
}

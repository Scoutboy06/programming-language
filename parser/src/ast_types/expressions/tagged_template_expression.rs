use crate::ast_types::{expressions::TemplateLiteral, node_objects::Node};

use super::Expression;
use parser_derive::Expr;

// es2015
// interface TaggedTemplateExpression <: Expression {
//     type: "TaggedTemplateExpression";
//     tag: Expression;
//     quasi: TemplateLiteral;
// }
#[derive(Debug, Clone, PartialEq, Expr)]
pub struct TaggedTemplateExpression {
    pub node: Node,
    pub tag: Expression,
    pub quasi: TemplateLiteral,
}

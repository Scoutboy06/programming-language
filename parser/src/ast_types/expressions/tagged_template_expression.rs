use crate::ast_types::{
    expressions::{Expression, TemplateLiteral},
    node_objects::Node,
};

// es2015
// interface TaggedTemplateExpression <: Expression {
//     type: "TaggedTemplateExpression";
//     tag: Expression;
//     quasi: TemplateLiteral;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct TaggedTemplateExpression {
    pub node: Node,
    pub tag: Expression,
    pub quasi: TemplateLiteral,
}

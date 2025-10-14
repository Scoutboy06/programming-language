use crate::ast_types::{
    classes::class::ClassBody, expressions::Expression, identifier::Identifier, node_objects::Node,
};

// es2015
// interface ClassExpression <: Class, Expression {
//     type: "ClassExpression";
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ClassExpression {
    pub node: Node,
    pub id: Identifier,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
}

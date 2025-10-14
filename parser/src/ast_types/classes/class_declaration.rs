use crate::ast_types::{
    classes::class::ClassBody, expressions::Expression, identifier::Identifier, node_objects::Node,
};

// es2015
// interface ClassDeclaration <: Class, Declaration {
//     type: "ClassDeclaration";
//     id: Identifier;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    pub node: Node,
    pub id: Identifier,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
}

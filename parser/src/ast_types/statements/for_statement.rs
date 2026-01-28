use crate::ast_types::{
    declarations::variable_declaration::VariableDeclaration, expressions::Expression,
    node_objects::Node, statements::Statement,
};

// es5
// interface ForStatement <: Statement {
//     type: "ForStatement";
//     init: VariableDeclaration | Expression | null;
//     test: Expression | null;
//     update: Expression | null;
//     body: Statement;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub node: Node,
    pub init: Option<ForInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ForInit {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

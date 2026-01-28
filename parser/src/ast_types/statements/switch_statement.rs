use crate::ast_types::{expressions::Expression, node_objects::Node, statements::Statement};

// es5
// interface SwitchStatement <: Statement {
//     type: "SwitchStatement";
//     discriminant: Expression;
//     cases: [ SwitchCase ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchStatement {
    pub node: Node,
    pub discriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCase {
    node: Node,
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

use crate::ast_types::{
    expressions::Expression, node_objects::Node, spread_element::SpreadElement,
};

// es5
// interface NewExpression <: Expression {
//     type: "NewExpression";
//     callee: Expression;
//     arguments: [ Expression ];
// }
//
// es2015
// extend interface NewExpression {
//     arguments: [ Expression | SpreadElement ];
// }
#[derive(Debug, PartialEq, Clone)]
pub struct NewExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Vec<NewExpressionArgument>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum NewExpressionArgument {
    Expression(Expression),
    SpreadElement(SpreadElement),
}

use super::Expression;
use crate::nodes::{Node, Pattern};
use lexer::AssignmentOperator;
use parser_derive::Expr;

#[derive(Debug, PartialEq, Clone, Expr)]
pub struct AssignmentExpression {
    pub node: Node,
    pub operator: AssignmentOperator,
    pub left: AssignmentExpressionLeft,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssignmentExpressionLeft {
    Pattern(Pattern),
    Expression(Expression),
}

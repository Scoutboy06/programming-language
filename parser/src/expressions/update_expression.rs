use super::Expression;
use crate::nodes::Node;
use lexer::Operator;
use parser_derive::Expr;

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct UpdateExpression {
    pub node: Node,
    pub argument: Expression,
    pub operator: UpdateOperator,
    pub prefix: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UpdateOperator {
    Increment, // ++
    Decrement, // --
}

pub trait AsUpdateOperator {
    fn as_update_operator(&self) -> Option<UpdateOperator>;
}

impl AsUpdateOperator for Operator {
    fn as_update_operator(&self) -> Option<UpdateOperator> {
        match self {
            Operator::Increment => Some(UpdateOperator::Increment),
            Operator::Decrement => Some(UpdateOperator::Decrement),
            _ => None,
        }
    }
}

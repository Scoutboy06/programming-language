use super::{BinaryOperation, Literal};
use crate::statements::Identifier;

#[derive(Debug, PartialEq)]
pub enum BinaryExpression {
    Literal(Literal),
    Identifier(Identifier),
    BinaryOperation(BinaryOperation),
}

use super::{BinaryExpression, Expression};
use crate::nodes::Node;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(StringLiteral),
    Boolean(BooleanLiteral),
    Null(NullLiteral),
    Number(NumberLiteral),
    BigInt(BigIntLiteral),
    Regex(RegexLiteral),
    JSXText(JSXTextLiteral),
}

impl Literal {
    pub fn as_expression(value: Self) -> Expression {
        Expression::Literal(Box::new(value))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub node: Node,
    pub value: Atom,
}

impl StringLiteral {
    pub fn as_expression(node: Node, value: Atom) -> Expression {
        Expression::Literal(Box::new(Literal::String(Self { node, value })))
    }

    pub fn as_bin_expression(node: Node, value: Atom) -> BinaryExpression {
        BinaryExpression::Literal(Literal::String(Self { node, value }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub node: Node,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn as_expression(node: Node, value: bool) -> Expression {
        Expression::Literal(Box::new(Literal::Boolean(Self { node, value })))
    }

    pub fn as_bin_expression(node: Node, value: bool) -> BinaryExpression {
        BinaryExpression::Literal(Literal::Boolean(Self { node, value }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullLiteral {
    pub node: Node,
}

impl NullLiteral {
    pub fn as_expression(node: Node) -> Expression {
        Expression::Literal(Box::new(Literal::Null(Self { node })))
    }

    pub fn as_bin_expression(node: Node) -> BinaryExpression {
        BinaryExpression::Literal(Literal::Null(Self { node }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub node: Node,
    pub value: f64,
}

impl NumberLiteral {
    pub fn as_expression(node: Node, value: f64) -> Expression {
        Expression::Literal(Box::new(Literal::Number(Self { node, value })))
    }

    pub fn as_bin_expression(node: Node, value: f64) -> BinaryExpression {
        BinaryExpression::Literal(Literal::Number(Self { node, value }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BigIntLiteral {
    pub node: Node,
}

impl BigIntLiteral {
    pub fn as_expression(node: Node) -> Expression {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegexLiteral {
    pub node: Node,
}

impl RegexLiteral {
    pub fn as_expression(node: Node) -> Expression {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JSXTextLiteral {
    pub node: Node,
}

impl JSXTextLiteral {
    pub fn as_expression(node: Node) -> Expression {
        todo!()
    }
}

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

macro_rules! init_literal {
    ($literal_type:ident, $variant:path) => {
        impl From<$literal_type> for Expression {
            fn from(value: $literal_type) -> Self {
                Expression::Literal(Box::new($variant(value)))
            }
        }

        impl From<$literal_type> for BinaryExpression {
            fn from(value: $literal_type) -> Self {
                BinaryExpression::Literal($variant(value))
            }
        }

        impl From<$literal_type> for Box<BinaryExpression> {
            fn from(value: $literal_type) -> Self {
                Box::new(value.into())
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub node: Node,
    pub value: Atom,
}
init_literal!(StringLiteral, Literal::String);

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub node: Node,
    pub value: bool,
}
init_literal!(BooleanLiteral, Literal::Boolean);

#[derive(Debug, Clone, PartialEq)]
pub struct NullLiteral {
    pub node: Node,
}
init_literal!(NullLiteral, Literal::Null);

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub node: Node,
    pub value: f64,
}
init_literal!(NumberLiteral, Literal::Number);

#[derive(Debug, Clone, PartialEq)]
pub struct BigIntLiteral {
    pub node: Node,
}
init_literal!(BigIntLiteral, Literal::BigInt);

#[derive(Debug, Clone, PartialEq)]
pub struct RegexLiteral {
    pub node: Node,
}
init_literal!(RegexLiteral, Literal::Regex);

#[derive(Debug, Clone, PartialEq)]
pub struct JSXTextLiteral {
    pub node: Node,
}
init_literal!(JSXTextLiteral, Literal::JSXText);

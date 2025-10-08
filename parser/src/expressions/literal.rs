use super::Expression;
use crate::nodes::Node;
use lexer::Keyword;
use parser_derive::Expr;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, PartialEq, Expr)]
pub enum Literal {
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    NullLiteral(NullLiteral),
    NumberLiteral(NumberLiteral),
}

macro_rules! init_literal {
    ($variant:ident) => {
        impl From<$variant> for Expression {
            fn from(value: $variant) -> Self {
                Expression::Literal(Box::new(Literal::$variant(value)))
            }
        }

        impl From<$variant> for Literal {
            fn from(value: $variant) -> Self {
                Literal::$variant(value)
            }
        }
    };
}

impl Literal {
    pub fn node(&self) -> &Node {
        match self {
            Literal::StringLiteral(s) => &s.node,
            Literal::BooleanLiteral(b) => &b.node,
            Literal::NullLiteral(n) => &n.node,
            Literal::NumberLiteral(n) => &n.node,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub node: Node,
    pub value: String,
}
init_literal!(StringLiteral);

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub node: Node,
    pub value: bool,
}
init_literal!(BooleanLiteral);

#[derive(Debug, Clone, PartialEq)]
pub struct NullLiteral {
    pub node: Node,
}
init_literal!(NullLiteral);

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub node: Node,
    pub value: f64,
}
init_literal!(NumberLiteral);

#[derive(Debug, Clone, PartialEq, Expr)]
pub struct Identifier {
    pub node: Node,
    pub name: Atom,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableKind {
    Var,
    Let,
    Const,
}

impl TryFrom<Keyword> for VariableKind {
    type Error = ();

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Var => Ok(Self::Var),
            Keyword::Let => Ok(Self::Let),
            Keyword::Const => Ok(Self::Const),
            _ => Err(()),
        }
    }
}

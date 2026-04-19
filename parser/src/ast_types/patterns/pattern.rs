use parser_derive::FromVariants;

use crate::ast_types::{
    expressions::Expression, identifier::Identifier, node_objects::Node,
    patterns::{
        array_pattern::ArrayPattern, object_pattern::ObjectPattern, rest_element::RestElement,
    },
};
use crate::utils::parser_error::ParserErrorInfo;

// es5
// interface Pattern <: Node { }
#[derive(Debug, PartialEq, Clone, FromVariants)]
pub enum Pattern {
    Identifier(Box<Identifier>),
    ArrayPattern(Box<ArrayPattern>),
    ObjectPattern(Box<ObjectPattern>),
    RestElement(Box<RestElement>),
}

impl Pattern {
    pub fn node(&self) -> &Node {
        match self {
            Self::Identifier(p) => &p.node,
            Self::ArrayPattern(p) => &p.node,
            Self::ObjectPattern(p) => &p.node,
            Self::RestElement(p) => &p.node,
        }
    }

    /// Converts an expression into a pattern for use in for-in/for-of loops
    pub fn try_from_expression(expr: Expression) -> Result<Pattern, ParserErrorInfo> {
        use crate::throw_error;
        match expr {
            Expression::Identifier(id) => Ok(Pattern::Identifier(id)),
            Expression::MemberExpression(_) => {
                // Member expressions are valid assignment targets in for-in/of
                // but we need to convert them to patterns
                // For now, we'll throw an error - this can be extended later
                throw_error!(InvalidToken)
            }
            _ => throw_error!(InvalidToken),
        }
    }
}

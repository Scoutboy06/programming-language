use crate::ast_types::node_objects::Node;

use super::Expression;
use parser_derive::Expr;

// es5
// interface MemberExpression <: Expression, Pattern {
//     type: "MemberExpression";
//     object: Expression;
//     property: Expression;
//     computed: boolean;
// }
#[derive(Debug, PartialEq, Clone, Expr)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: Expression,
    pub computed: bool,
}

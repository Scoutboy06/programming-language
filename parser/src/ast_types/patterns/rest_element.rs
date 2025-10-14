use crate::ast_types::patterns::pattern::Pattern;

// es2015
// interface RestElement <: Pattern {
//     type: "RestElement";
//     argument: Pattern;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct RestElement {
    pub argument: Pattern,
}

use crate::ast_types::patterns::pattern::Pattern;

// es2015
// interface ArrayPattern <: Pattern {
//     type: "ArrayPattern";
//     elements: [ Pattern | null ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPattern {
    pub elements: Vec<Option<Pattern>>,
}

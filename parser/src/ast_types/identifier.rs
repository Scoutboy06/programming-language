use crate::ast_types::node_objects::Node;

use string_cache::DefaultAtom as Atom;

/*
es5
interface Identifier <: Expression, Pattern {
    type: "Identifier";
    name: string;
}
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub node: Node,
    pub name: Atom,
}

use crate::nodes::Node;
use lexer::TypeKeyword;
use string_cache::DefaultAtom as Atom;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub node: Node,
    pub type_value: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub node: Node,
    pub value: TypeValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeValue {
    KeywordType(TypeKeyword),
    TypeReference(Atom),
}

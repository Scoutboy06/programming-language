use crate::ast_types::node_objects::Node;

// es5
// interface Literal <: Expression {
//     type: "Literal";
//     value: string | boolean | null | number | RegExp;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub node: Node,
    pub value: LiteralValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    String(String),
    Boolean(bool),
    Null,
    Number(f64),
    RegExp(RegExpLiteral),
}

// es5
// interface RegExpLiteral <: Literal {
//   regex: {
//     pattern: string;
//     flags: string;
//   };
// }
#[derive(Debug, Clone, PartialEq)]
pub struct RegExpLiteral {
    pattern: String,
    flags: String,
}

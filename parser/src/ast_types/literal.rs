use crate::ast_types::node_objects::Node;

// es5
// interface Literal <: Expression {
//     type: "Literal";
//     value: string | boolean | null | number | RegExp;
// }
//
// es2020
// extend interface Literal <: Expression {
//     type: "Literal";
//     value: string | boolean | null | number | RegExp | bigint;
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
    Bigint(BigIntLiteral),
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

impl Into<LiteralValue> for RegExpLiteral {
    fn into(self) -> LiteralValue {
        LiteralValue::RegExp(self)
    }
}

// es2020
// interface BigIntLiteral <: Literal {
//   bigint: string;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct BigIntLiteral {
    bigint: String,
}

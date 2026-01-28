use crate::ast_types::{
    identifier::Identifier, node_objects::Node, patterns::pattern::Pattern,
    statements::FunctionBody,
};

// es5
// interface Function <: Node {
//    id: Identifier | null;
//    params: [ Pattern ];
//    body: FunctionBody;
// }
//
// es2015
// extend interface Function {
//     generator: boolean;
// }
//
// es2017
// extend interface Function {
//     async: boolean;
// }
pub struct Function {
    pub node: Node,
    pub id: Option<Identifier>,
    pub params: Vec<Pattern>,
    pub body: FunctionBody,
    pub generator: bool,
    pub _async: bool,
}

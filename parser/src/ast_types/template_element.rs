use crate::ast_types::node_objects::Node;

// es2015
// interface TemplateElement <: Node {
//     type: "TemplateElement";
//     tail: boolean;
//     value: {
//         cooked: string;
//         raw: string;
//     };
// }
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateElement {
    pub node: Node,
    pub tail: bool,
    pub value: TemplateElementValue,
}

#[derive(Debug, Clone, PartialEq)]
struct TemplateElementValue {
    pub cooked: String,
    pub raw: String,
}

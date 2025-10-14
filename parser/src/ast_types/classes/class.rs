use crate::ast_types::{
    classes::{class_declaration::ClassDeclaration, class_expression::ClassExpression},
    expressions::{Expression, FunctionExpression},
};

// es2015
// interface Class <: Node {
//     id: Identifier | null;
//     superClass: Expression | null;
//     body: ClassBody;
// }
pub enum Class {
    ClassDeclaration(Box<ClassDeclaration>),
    ClassExpression(Box<ClassExpression>),
}

// es2015
// interface ClassBody <: Node {
//     type: "ClassBody";
//     body: [ MethodDefinition ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody {
    pub body: Vec<MethodDefinition>,
}

// es2015
// interface MethodDefinition <: Node {
//     type: "MethodDefinition";
//     key: Expression;
//     value: FunctionExpression;
//     kind: "constructor" | "method" | "get" | "set";
//     computed: boolean;
//     static: boolean;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDefinition {
    pub key: Expression,
    pub value: FunctionExpression,
    pub kind: MethodDefinitionKind,
    pub computed: bool,
    pub _static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MethodDefinitionKind {
    Constructor,
    Method,
    Get,
    Set,
}

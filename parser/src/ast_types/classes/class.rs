use string_cache::DefaultAtom as Atom;

use crate::ast_types::{
    classes::{class_declaration::ClassDeclaration, class_expression::ClassExpression},
    expressions::{Expression, FunctionExpression},
    node_objects::Node,
    statements::StaticBlock,
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
//
// es2022
// extend interface ClassBody {
//     body: [ MethodDefinition | PropertyDefinition | StaticBlock ];
// }
#[derive(Debug, Clone, PartialEq)]
pub struct ClassBody {
    pub node: Node,
    pub body: Vec<MethodDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassBodyBody {
    MethodDefinition(MethodDefinition),
    PropertyDefinition(PropertyDefinition),
    StaticBlock(StaticBlock),
}

// es2022
// interface PropertyDefinition <: Node {
//     type: "PropertyDefinition";
//     key: Expression | PrivateIdentifier;
//     value: Expression | null;
//     computed: boolean;
//     static: boolean;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDefinition {
    pub key: PropertyDefinitionKey,
    pub value: Option<Expression>,
    pub computed: bool,
    pub _static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyDefinitionKey {
    Expression(Expression),
    PrivateIdentifier(PrivateIdentifier),
}

// es2022
// interface PrivateIdentifier <: Node {
//     type: "PrivateIdentifier";
//     name: string;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateIdentifier {
    pub node: Node,
    pub name: Atom,
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
//
// es2022
// extend interface MethodDefinition {
//     key: Expression | PrivateIdentifier;
// }
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDefinition {
    pub node: Node,
    pub key: Expression,
    pub value: FunctionExpression,
    pub kind: MethodDefinitionKind,
    pub computed: bool,
    pub _static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MethodDefinitionKey {
    Expression(Expression),
    PrivateIdentifier(PrivateIdentifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MethodDefinitionKind {
    Constructor,
    Method,
    Get,
    Set,
}

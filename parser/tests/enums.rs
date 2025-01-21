use parser::{
    expressions::NumberLiteral,
    nodes::{program::Program, Node},
    statements::{EnumMember, EnumStatement, Identifier},
    Parser,
};
use pretty_assertions::assert_eq;
mod helpers;
use helpers::*;

#[test]
fn enum_statement() {
    let code = "enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: false,
            is_const: false,
            id: Identifier {
                node: code.node("Foo", 0),
                name: "Foo".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("Bar", 0),
                    id: Identifier {
                        node: code.node("Bar", 0),
                        name: "Bar".into(),
                    },
                    init: None,
                },
                EnumMember {
                    node: code.node("Baz", 0),
                    id: Identifier {
                        node: code.node("Baz", 0),
                        name: "Baz".into(),
                    },
                    init: None,
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn enum_with_initialized_values() {
    let code = "enum Color { White = 1, Black = 0 }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: false,
            is_const: false,
            id: Identifier {
                node: code.node("Color", 0),
                name: "Color".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("White = 1", 0),
                    id: Identifier {
                        node: code.node("White", 0),
                        name: "White".into(),
                    },
                    init: Some(
                        NumberLiteral {
                            node: code.node("1", 0),
                            value: 1.0,
                        }
                        .into(),
                    ),
                },
                EnumMember {
                    node: code.node("Black = 0", 0),
                    id: Identifier {
                        node: code.node("Black", 0),
                        name: "Black".into(),
                    },
                    init: Some(
                        NumberLiteral {
                            node: code.node("0", 0),
                            value: 0.0,
                        }
                        .into(),
                    ),
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn const_enum_statement() {
    let code = "const enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: false,
            is_const: true,
            id: Identifier {
                node: code.node("Foo", 0),
                name: "Foo".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("Bar", 0),
                    id: Identifier {
                        node: code.node("Bar", 0),
                        name: "Bar".into(),
                    },
                    init: None,
                },
                EnumMember {
                    node: code.node("Baz", 0),
                    id: Identifier {
                        node: code.node("Baz", 0),
                        name: "Baz".into(),
                    },
                    init: None,
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

#[test]
fn ambient_enum_statement() {
    let code = "declare enum Foo { Bar, Baz }";
    let mut parser = Parser::new(&code);
    let result = parser.parse();

    let expected = Program {
        node: Node::new(0, code.len()),
        shebang: None,
        body: vec![EnumStatement {
            node: Node::new(0, code.len()),
            is_declare: true,
            is_const: false,
            id: Identifier {
                node: code.node("Foo", 0),
                name: "Foo".into(),
            },
            members: vec![
                EnumMember {
                    node: code.node("Bar", 0),
                    id: Identifier {
                        node: code.node("Bar", 0),
                        name: "Bar".into(),
                    },
                    init: None,
                },
                EnumMember {
                    node: code.node("Baz", 0),
                    id: Identifier {
                        node: code.node("Baz", 0),
                        name: "Baz".into(),
                    },
                    init: None,
                },
            ],
        }
        .into()],
    };

    assert_eq!(result, Ok(expected));
}

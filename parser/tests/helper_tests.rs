use pretty_assertions::assert_eq;
mod helpers;
use helpers::NodeConstructor;
use parser::nodes::Node;

#[test]
fn helper_find_n() {
    let code = "let a = 50.5";

    assert_eq!(code.find_n("a", 0), Some(4));
    assert_eq!(code.find_n("=", 0), Some(6));
    assert_eq!(code.find_n("5", 0), Some(8));
    assert_eq!(code.find_n("5", 1), Some(11));
}

#[test]
fn helper_node() {
    let code = "{ hello() }";

    assert_eq!(code.node("hello", 0), Node::new(2, 7));
    assert_eq!(code.node("()", 0), Node::new(7, 9));
    assert_eq!(code.node("l", 0), Node::new(4, 5));
    assert_eq!(code.node("l", 1), Node::new(5, 6));
    assert_eq!(code.node(code, 0), Node::new(0, code.len()));
}

#[test]
fn helper_between() {
    let code = "function a() { hello() }";

    assert_eq!(code.between(("f", 0), ("n", 1)), Node::new(1, 7));
    assert_eq!(code.between(("(", 0), (")", 0)), Node::new(11, 11));
    assert_eq!(code.between(("(", 1), (")", 1)), Node::new(21, 21));
    assert_eq!(code.between(("{", 0), ("}", 0)), Node::new(14, 23));
}

#[test]
fn helper_between_incl() {
    let code = "function a() { hello() }";

    assert_eq!(code.between_incl(("f", 0), ("n", 1)), Node::new(0, 8));
    assert_eq!(code.between_incl(("(", 0), (")", 0)), Node::new(10, 12));
    assert_eq!(code.between_incl(("(", 1), (")", 1)), Node::new(20, 22));
    assert_eq!(code.between_incl(("{", 0), ("}", 0)), Node::new(13, 24));
}

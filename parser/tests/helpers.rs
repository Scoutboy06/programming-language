use parser::nodes::Node;
use pretty_assertions::assert_eq;

pub trait NodeConstructor {
    fn find_n(&self, target: &str, n: usize) -> Option<usize>;
    fn node(&self, target: &str, n: usize) -> Node;
    fn between(&self, left: (&str, usize), right: (&str, usize)) -> Node;
    fn between_incl(&self, left: (&str, usize), right: (&str, usize)) -> Node;
}

impl NodeConstructor for str {
    /// Searches for the n-th occurrence of `target`, and returns the index
    fn find_n(&self, target: &str, n: usize) -> Option<usize> {
        let mut index: Option<usize> = None;
        let bytes = self.as_bytes();
        let target_bytes = target.as_bytes();
        let mut count = 0;

        'outer: for i in 0..bytes.len() {
            for j in 0..target_bytes.len() {
                let b = bytes[i + j];
                if b != target_bytes[j] {
                    continue 'outer;
                }
            }

            count += 1;
            if count == n + 1 {
                index = Some(i);
                break 'outer;
            }
        }

        index
    }

    // Searches for the n-th occurrence of `target`, and returns a Node that spans inside it
    fn node(&self, target: &str, n: usize) -> Node {
        let index = self.find_n(target, n);

        assert!(
            index.is_some(),
            "Target not found in NodeConstructor::node()\n  target: {}\n  n: {}",
            target,
            n
        );

        Node::new(index.unwrap(), index.unwrap() + target.len())
    }

    /// Searches for the n-th occurrence of `left` and `right`, and returns a Node that spans between them.
    fn between(&self, left: (&str, usize), right: (&str, usize)) -> Node {
        let left_bytes = left.0.as_bytes();
        let right_bytes = right.0.as_bytes();

        assert!(self.len() > left_bytes.len() + right_bytes.len());

        let left_index = self.find_n(left.0, left.1).expect(&format!(
            "Could not find left value\n  left: {}\n  n: {}",
            left.0, left.1
        ));
        let right_index = self.find_n(right.0, right.1).expect(&format!(
            "Could not find right value\n  right: {}\n  n: {}",
            right.0, right.1
        ));

        assert!(
            left_index < right_index,
            "Left index and right index are wrong"
        );

        Node::new(left_index + left.0.len(), right_index)
    }

    /// Searches for the n-th occurrence of `left` and `right`, and returns a Node that spans between *and including* them
    fn between_incl(&self, left: (&str, usize), right: (&str, usize)) -> Node {
        let between = self.between(left, right);

        Node::new(between.start - left.0.len(), between.end + right.0.len())
    }
}

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

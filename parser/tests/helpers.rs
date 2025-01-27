use parser::nodes::Node;

pub trait NodeConstructor {
    fn find_n(&self, target: &str, n: usize) -> Option<usize>;
    fn node(&self, target: &str, n: usize) -> Node;
    fn between(&self, left: (&str, usize), right: (&str, usize)) -> Node;
    #[allow(unused)]
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

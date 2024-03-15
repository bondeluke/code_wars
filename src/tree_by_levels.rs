// https://www.codewars.com/kata/52bef5e3588c56132c0003bc

use std::collections::VecDeque;

pub struct Node {
    pub value: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32) -> Self {
        Node { value, left: None, right: None }
    }

    pub fn left(mut self, left: Node) -> Self {
        self.left = Some(Box::new(left));
        self
    }

    pub fn right(mut self, right: Node) -> Self {
        self.right = Some(Box::new(right));
        self
    }
}

#[allow(dead_code)]
fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    let mut queue: VecDeque<&Node> = VecDeque::new();
    queue.push_back(root);

    while let Some(item) = queue.pop_front() {
        result.push(item.value);
        if let Some(l) = &item.left {
            queue.push_back(l);
        }
        if let Some(r) = &item.right {
            queue.push_back(r);
        }
    }

    result
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(tree_by_levels(&Node::new(42)),
                   [42],
                   "\nYour result (left) didn't match the expected output (right).");
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2)
                .left(Node::new(4))
                .right(Node::new(5)))
            .right(Node::new(3)
                .left(Node::new(6)));
        assert_eq!(tree_by_levels(&root),
                   [1, 2, 3, 4, 5, 6],
                   "\nYour result (left) didn't match the expected output (right).");
    }
}
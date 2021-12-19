pub struct Node {
    value: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    pub fn root(left: Node, right: Node) -> Node {
        Node {
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right))
        }
    }

    pub fn leaf(value: u8) -> Node {
        Node {
            value: Some(value),
            left: None,
            right: None
        }
    }
}

#[test]
fn test_node() {
    let root = Node::root(
        Node::leaf(5),
        Node::leaf(2)
    );
}

#[test]
fn test_addition() {
    let tree_1 = Node::root(
        Node::leaf(5),
        Node::leaf(2)
    );

    let tree_2 = Node::root(
        Node::leaf(5),
        Node::leaf(2)
    );

    let sum = Node::root(
        tree_1,
        tree_2
    );
}

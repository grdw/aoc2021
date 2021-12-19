pub struct Node {
    depth: u8,
    value: Option<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    pub fn root(left: Node, right: Node) -> Node {
        Node {
            depth: 0,
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right))
        }
    }

    pub fn empty(left: Node, right: Node, depth: u8) -> Node {
        Node {
            depth: depth,
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right))
        }
    }

    pub fn leaf(value: u8, depth: u8) -> Node {
        Node {
            depth: depth,
            value: Some(value),
            left: None,
            right: None
        }
    }
}

#[test]
fn test_node() {
    let root = Node::root(
        Node::leaf(5, 1),
        Node::leaf(2, 1)
    );

    assert_eq!(action(&root), Action::NonAction)
}

#[test]
fn test_addition() {
    let tree_1 = Node::root(
        Node::leaf(5, 1),
        Node::leaf(2, 1)
    );

    let tree_2 = Node::root(
        Node::leaf(5, 1),
        Node::leaf(2, 1)
    );

    let sum = Node::root(
        tree_1,
        tree_2
    );

    assert_eq!(action(&sum), Action::NonAction)
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Explode,
    Split,
    NonAction
}

fn list_nodes(tree: &Node, nodes: &mut Vec<Node>) {
    if let Some(left) = &tree.left {
    }

    if let Some(tree) = &tree.right {

    }
}

fn action(tree: &Node) -> Action {
    let mut nodes = vec![];
    list_nodes(tree, &mut nodes);

    let exploding = nodes.iter().find(|&n| n.depth > 4);
    let splitting = nodes.iter().find(|&n| n.value.unwrap() > 9);

    if let Some(n) = exploding {
        return Action::Explode
    }

    if let Some(n) = splitting {
        return Action::Split
    }

    Action::NonAction
}

#[test]
fn test_explode() {
    let mut tree = Node::root(
        Node::empty(
            Node::empty(
                Node::empty(
                    Node::leaf(0, 4),
                    Node::leaf(9, 4),
                    3
                ),
                Node::leaf(2, 3),
                2
            ),
            Node::leaf(3, 2), 1
        ),
        Node::leaf(4, 1)
    );

    assert_eq!(action(&tree), Action::Explode)
}

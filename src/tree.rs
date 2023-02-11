#[derive(Debug, Clone)]
pub struct Tree {
    root: Option<Box<Node>>,
}
#[derive(Debug, Clone)]
pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            }
            Some(node) => Tree::insert_recursive(node, value),
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                }
                Some(node) => {
                    Tree::insert_recursive(node, value);
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(node) => {
                    Tree::insert_recursive(node, value);
                }
            }
        }
    }

    pub fn find(&self, value: i32) -> Option<&Box<Node>> {
        match &self.root {
            None => return None,
            Some(node) => return Tree::find_recursive(node, value),
        }
    }

    fn find_recursive(node: &Box<Node>, value: i32) -> Option<&Box<Node>> {
        if node.value == value {
            return Option::Some(node);
        } else if node.value < value {
            match &node.right {
                None => return None,
                Some(n) => Tree::find_recursive(n, value),
            }
        } else {
            match &node.left {
                None => return None,
                Some(n) => Tree::find_recursive(n, value),
            }
        }
    }
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            right: None,
            left: None,
        }
    }

    pub fn print(tree: &Tree) {
        let mut count = 1;
        match &tree.root {
            None => return,
            Some(n) => {
                println!("{}: {:?}", count, n.value);
                Node::print_recursive(n, &mut count);
            }
        }
    }

    fn print_recursive(node: &Box<Node>, count: &mut i32) {
        let mut is_left_none = false;
        let mut is_right_none = false;

        match &node.right {
            None => is_right_none = true,
            Some(n) => {
                *count += 1;
                println!("{}: {:?}", count, n.value);
                Node::print_recursive(Box::new(n).as_ref(), count);
            }
        }

        match &node.left {
            None => is_left_none = true,
            Some(n) => {
                *count += 1;
                println!("{}:  {:?}", count, n.value);
                Node::print_recursive(Box::new(n).as_ref(), count);
            }
        }

        if is_left_none && is_right_none {
            return;
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

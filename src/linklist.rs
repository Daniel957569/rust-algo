#[derive(Debug)]
#[allow(dead_code)]
pub enum ReturnParent {
    Node(Box<Node>),
    String(String),
}
enum ReturnChild {
    Node(Box<Node>),
    String(String),
}

#[derive(Debug, Clone)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}
#[derive(Debug, Clone)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    fn new_option(value: i32) -> Option<Box<Node>> {
        Option::Some(Box::new(Node::new(value)))
    }

    pub fn add(&mut self, value: i32) {
        match &mut self.head {
            None => self.head = LinkedList::new_option(value),
            Some(node) => LinkedList::add_recursively(node, value),
        }
    }

    fn add_recursively(node: &mut Box<Node>, value: i32) {
        match &mut node.next {
            None => node.next = LinkedList::new_option(value),
            Some(n) => LinkedList::add_recursively(n, value),
        }
    }

    pub fn add_first(&mut self, value: i32) {
        match &self.head {
            None => self.head = LinkedList::new_option(value),
            Some(node) => {
                let mut new_node = Box::new(Node::new(value));
                new_node.next = Option::Some(node.clone());
                self.head = Option::Some(new_node);
            }
        }
    }

    pub fn remove_first(&mut self) {
        match &mut self.head {
            None => return,
            Some(node) => self.head = node.next.clone(),
        }
    }

    pub fn print(&self) {
        let mut current = self.clone().head;
        let mut num = 1;

        while let Some(node) = current {
            println!("{} {:?}", num, node.value);
            current = node.next;
            num += 1;
        }
    }

    pub fn size(&self) -> i32 {
        let mut current = self.clone().head;
        let mut num = 1;

        while let Some(node) = current {
            num += 1;
            current = node.next;
        }
        return num;
    }

    pub fn find(&mut self, value: i32) -> ReturnParent {
        match &mut self.head {
            None => ReturnParent::String(String::from("value not found")),
            Some(node) => match LinkedList::find_recusively(node, value) {
                ReturnChild::Node(n) => ReturnParent::Node(n),
                ReturnChild::String(_) => ReturnParent::String(String::from("Value Wasn't Found")),
            },
        }
    }

    fn find_recusively(node: &mut Box<Node>, value: i32) -> ReturnChild {
        if node.value == value {
            return ReturnChild::Node(node.clone());
        }
        match &mut node.next {
            None => ReturnChild::String(String::from("")),
            Some(n) => LinkedList::find_recusively(n, value),
        }
    }

    pub fn contains(&mut self, value: i32) -> bool {
        match &mut self.head {
            None => false,
            Some(node) => LinkedList::contains_recusively(node, value),
        }
    }

    fn contains_recusively(node: &mut Box<Node>, value: i32) -> bool {
        if node.value == value {
            return true;
        }
        match &mut node.next {
            None => false,
            Some(n) => LinkedList::contains_recusively(n, value),
        }
    }

    fn reverse(&self) {}
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }
}

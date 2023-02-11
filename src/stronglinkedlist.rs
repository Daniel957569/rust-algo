use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
pub struct Node<T: Copy + PartialEq> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy + PartialEq> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T: Copy + PartialEq> From<Node<T>> for Option<Rc<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

type NodePtr<T: Copy + PartialEq> = Rc<RefCell<Node<T>>>;

#[derive(Debug, Clone)]
pub struct List<T: Copy + PartialEq> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
}

impl<T: Copy + PartialEq> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);

        match &mut self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into();
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            None => None,
            Some(current_tail) => {
                let mut tail = current_tail.borrow_mut();
                let prev = tail.prev.take();
                match prev {
                    None => {
                        self.head.take();
                    }
                    Some(prev) => {
                        let prev = prev.upgrade();

                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev)
                        }
                    }
                };
                Some(tail.value)
            }
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);

        match &mut self.head.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(head) => {
                node.next = Some(head.clone());
                self.head = node.into();

                if let Some(h) = &self.head {
                    head.borrow_mut().prev = Some(Rc::downgrade(&h));
                    return;
                }
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();

                match next {
                    None => {
                        self.tail.take();
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }

                Some(head.value)
            }
        }
    }

    pub fn contains(&mut self, value: T) -> bool {
        match &mut self.head {
            None => false,
            Some(n) => List::contains_recursive(n, value),
        }
    }

    fn contains_recursive(node: &Rc<RefCell<Node<T>>>, value: T) -> bool {
        if node.borrow_mut().value == value {
            return true;
        }

        match &node.borrow_mut().next {
            None => {
                if node.borrow_mut().value == value {
                    return true;
                }
                return false;
            }
            Some(node) => List::contains_recursive(node, value),
        }
    }

    pub fn size(&mut self) -> usize {
        let size: usize = 0;
        match &self.head {
            None => 0,
            Some(node) => List::size_recursive(node, size),
        }
    }

    fn size_recursive(node: &Rc<RefCell<Node<T>>>, value: usize) -> usize {
        let value = value + 1;
        match &mut node.borrow_mut().next {
            None => value,
            Some(v) => List::size_recursive(&v, value),
        }
    }
}

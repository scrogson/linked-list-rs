use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let head = Node::new(elem);
        match self.head.take() {
            Some(old) => {
                old.borrow_mut().prev = Some(head.clone());
                head.borrow_mut().next = Some(old);
                self.head = Some(head);
            }
            None => {
                self.tail = Some(head.clone());
                self.head = Some(head);
            }
        }
    }
}

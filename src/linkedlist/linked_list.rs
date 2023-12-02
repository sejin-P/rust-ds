use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;

pub struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl <T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn insert(&mut self, val: T) {
        if self.head.is_none() {
            let node_ptr = Rc::new(RefCell::new(Node{val, next: None}));
            self.head = Some(Rc::clone(&node_ptr));
            self.tail = Some(Rc::clone(&node_ptr));
            self.len += 1;
            return
        }

        let node_ptr = Rc::new(RefCell::new(Node{val, next: None}));
        let mut binding = self.tail.clone().unwrap();
        let borrow_tail = binding.borrow_mut();
        unsafe {
            (*borrow_tail.as_ptr()).next = Some(Rc::clone(&node_ptr));
        }
        self.tail = Some(Rc::clone(&node_ptr));
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn tail(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.tail.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkedList};

    #[test]
    fn insert_tail() {
        let mut l = LinkedList::new();
        l.insert(1);
        l.insert(2);

        let t = l.tail();
        assert_eq!(2, t.unwrap().borrow().val)
    }
}

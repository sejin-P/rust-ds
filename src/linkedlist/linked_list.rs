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
        let new_node = Rc::new(RefCell::new(Node{val, next: None}));

        match self.tail.take() {
            Some(mut old_tail) => {
                let mut binding = old_tail.clone();
                let borrow_tail = binding.borrow_mut();
                unsafe {
                    (*borrow_tail.as_ptr()).next = Some(Rc::clone(&new_node));
                }
                self.tail = Some(new_node);
            },
            None => {
                // If the list is empty, new node becomes both head and tail.
                self.head = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
        }

        self.len += 1;
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
        l.insert(3);

        let t = l.tail();
        assert_eq!(3, t.unwrap().borrow().val)
    }
}

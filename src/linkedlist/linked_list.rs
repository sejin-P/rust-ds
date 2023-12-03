use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::mem;
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

    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node{val, next: None}));

        match self.tail.take() {
            Some(mut old_tail) => {
                unsafe {
                    (*old_tail.clone().as_ptr()).next = Some(Rc::clone(&new_node));
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

    pub fn push_front(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node{val, next: self.head.clone()}));
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| unsafe {
            // below cloned old_head is safely dropped, so we can call try_unwrap
            let next = mem::replace(&mut (*old_head.clone().as_ptr()).next, None);
            self.head = next;
            return Rc::try_unwrap(old_head)
                .ok()
                .expect("There are multiple owners of the node")
                .into_inner()
                .val
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn tail(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.tail.clone()
    }
    pub fn head(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{LinkedList};

    #[test]
    fn push_back() {
        let mut l = LinkedList::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);

        let t = l.tail();
        assert_eq!(3, t.unwrap().borrow().val);
        assert_eq!(1, l.head().unwrap().borrow().val);
    }

    #[test]
    fn push_front() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        let h = l.head();
        assert_eq!(3, h.clone().unwrap().borrow().val);
        assert_eq!(2, h.clone().unwrap().borrow().next.clone().unwrap().borrow().val);
    }

    #[test]
    fn pop_front() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        let p = l.pop_front();
        assert_eq!(2, p.unwrap());
        let p = l.pop_front();
        assert_eq!(1, p.unwrap());
        let p = l.pop_front();
        assert_eq!(None, p);
    }
}

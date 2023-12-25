use std::borrow::BorrowMut;
use std::cell::{RefCell};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;

pub struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

pub struct LinkedList<T: PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl <T: PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node{val, next: None}));
        let new_tail = Some(new_node.clone());

        match self.tail.take() {
            Some(old_tail) => {
                RefCell::borrow_mut(&old_tail).next = new_tail;
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
        if self.head.clone().is_none() {
            self.tail = Some(new_node.clone())
        }
        self.head = Some(new_node);
        self.len += 1;

    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let next = mem::replace(&mut (RefCell::borrow_mut(&old_head)).next, None);
            self.head = next;
            self.len -= 1;
            // below cloned old_head is safely dropped, so we can call try_unwrap
            return Rc::try_unwrap(old_head)
                .ok()
                .expect("There are multiple owners of the node")
                .into_inner()
                .val
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut prev = None;
        let mut n = self.head.clone();
        while !n.is_none() {
            let next = n.clone().unwrap().borrow().next.clone();
            if next.is_none() {
                break
            }

            prev = n.clone();
            n = next;
        }

        match prev.take() {
            Some(old_prev) => {
                RefCell::borrow_mut(&old_prev).next = None;
            }
            None => {
                self.head = None
            }
        }

        self.len -= 1;
        n.take().map(|last_node| {
            Rc::try_unwrap(last_node)
                .ok()
                .expect("There are multiple owners of the node")
                .into_inner()
                .val
        })
    }

    pub fn search(&self, val: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut n = self.head.clone();
        while !n.is_none() {
            if n.clone().unwrap().borrow().val == val {
                return n;
            }
            n = n.clone().unwrap().borrow().next.clone();
        }

        return None;
    }

    pub fn remove(&mut self, val: T) {
        let mut prev: Option<Rc<RefCell<Node<T>>>> = None;
        let mut n = self.head.clone();
        while !n.is_none() {
            if n.clone().unwrap().borrow().val == val {
                self.len -= 1;
                if prev.is_none() && n.clone().unwrap().borrow().next.is_none() {
                    self.head = None;
                    self.tail = None;
                    return
                }
                if prev.is_none() {
                    self.head = n.unwrap().borrow().next.clone();
                    return
                }
                if n.clone().unwrap().borrow().next.is_none() {
                    self.tail = prev.clone()
                }
                RefCell::borrow_mut(&(prev.unwrap())).next = n.unwrap().borrow().next.clone();

                return
            }
            prev = n.clone();
            n = n.clone().unwrap().borrow().next.clone();
        }
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

    #[test]
    fn pop_back() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        let p = l.pop_back();
        assert_eq!(1, p.unwrap());
        let p = l.pop_back();
        assert_eq!(2, p.unwrap());
    }

    #[test]
    fn search() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        let n = l.search(2);
        assert_eq!(2, n.unwrap().borrow().val)
    }

    #[test]
    fn remove() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        l.remove(1);
        let n = l.search(1);
        assert!(n.is_none());
    }
}

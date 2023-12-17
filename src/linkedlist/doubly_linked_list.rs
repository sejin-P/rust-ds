use std::cell::RefCell;
use std::rc::Rc;

pub struct DoubleNode<T> {
    val: T,
    next: Option<Rc<RefCell<DoubleNode<T>>>>,
    prev: Option<Rc<RefCell<DoubleNode<T>>>>
}

pub struct DoublyLinkedList<T: PartialEq> {
    head: Option<Rc<RefCell<DoubleNode<T>>>>,
    tail: Option<Rc<RefCell<DoubleNode<T>>>>,
    len: usize,
}

impl <T: PartialEq> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(DoubleNode{
            val,
            next: None,
            prev: self.tail.clone(),
        }));
        let new_tail = Some(new_node.clone());

        match self.tail.take() {
            Some(old_tail) => {
                RefCell::borrow_mut(&old_tail).next = new_tail;
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node)
            }
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(DoubleNode{
            val,
            next: self.head.clone(),
            prev: None,
        }));

        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(old_head) => {
                // cannot move issue occurs when var assigned by RefCell::borrow_mut(&old_head) and remains out of this scope.
                if let Some(next) = RefCell::borrow_mut(&old_head).next.take() {
                    RefCell::borrow_mut(&next).prev = None;
                    self.head = Some(next);
                }

                return Some(Rc::try_unwrap(old_head).ok().unwrap().into_inner().val);;
            }
            None => None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(old_tail) => {
                if let Some(prev) = RefCell::borrow_mut(&old_tail).next.take() {
                    RefCell::borrow_mut(&prev).next = None;
                    self.tail = Some(prev);
                }

                return Some(Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val);
            }

            None => None
        }
    }

    pub fn search(&self, val: T) -> Option<Rc<RefCell<DoubleNode<T>>>> {
        let mut n = self.head.clone();
        if !n.is_none() {
            if val == n.clone().unwrap().borrow().val {
                return n
            }
        }

        return None;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn tail(&self) -> Option<Rc<RefCell<DoubleNode<T>>>> {
        self.tail.clone()
    }

    pub fn head(&self) -> Option<Rc<RefCell<DoubleNode<T>>>> {
        self.head.clone()
    }
}

#[cfg(test)]
mod test {
    use super::{DoublyLinkedList};

    #[test]
    fn push_back() {
        let mut l = DoublyLinkedList::new();
        l.push_back(1);
        l.push_back(2);
    }
}

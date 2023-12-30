use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct BinarySearchTree<T: PartialOrd> {
    val: T,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new(val: T) -> Self {
        BinarySearchTree {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: T) {
        if val < self.val {
            match &mut self.left {
                Some(left) => left.insert(val),
                None => self.left = Some(Box::new(BinarySearchTree::new(val))),
            }
        } else if val > self.val {
            match &mut self.right {
                Some(right) => right.insert(val),
                None => self.right = Some(Box::new(BinarySearchTree::new(val))),
            }
        }
        // If val == self.val, do nothing (or handle duplicates as needed)
    }

    pub fn search(&self, val: &T) -> Option<&BinarySearchTree<T>> {
        if *val < self.val {
            self.left.as_ref()?.search(val)
        } else if *val > self.val {
            self.right.as_ref()?.search(val)
        } else {
            Some(self)
        }
    }
}




#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::BinarySearchTree;

    #[test]
    fn insert_search() {
        let mut b = BinarySearchTree::new(5);
        b.insert(1);
        b.insert(9);
        assert_eq!(1, b.search(&1).unwrap().val)
    }
}

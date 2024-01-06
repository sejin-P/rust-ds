use std::cell::RefCell;
use std::mem;
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

    pub fn delete_by_merging(&mut self, val: &T) {
        if val < &self.val {
            match self.left.as_mut() {
                None => { return }
                Some(tree) => {
                    if &tree.val == val && tree.left.is_none() && tree.right.is_none() {
                        self.left = None;
                        return
                    }
                    return tree.delete_by_merging(val)
                }
            }
        }

        if val > &self.val {
            match self.right.as_mut() {
                None => {  }
                Some(tree) => {
                    if &tree.val == val && tree.left.is_none() && tree.right.is_none() {
                        self.right = None;
                        return
                    }
                    return tree.delete_by_merging(val)
                }
            }
        }

        if self.right.is_none() {
            let mut left = self.left.take().unwrap();
            self.val = left.val;
            self.right = left.right;
            self.left = left.left;

            return
        }

        if self.left.is_none() {
            let mut right = self.right.take().unwrap();
            self.val = right.val;
            self.right = right.right;
            self.left = right.left;
            return
        }

        let right = self.right.take().unwrap();
        let mut left = self.left.take().unwrap();

        self.val = left.val;
        self.left = left.left;

        self.attach_right(right);
    }

    fn attach_right(&mut self, right: Box<BinarySearchTree<T>>) {
        match &mut self.right {
            Some(right_child) => {
                right_child.attach_right(right)
            }
            None => {
                self.right = Some(right);
            }
        }
    }

    pub fn rotate_right(&mut self, mut n: Box<BinarySearchTree<T>>) -> Box<BinarySearchTree<T>> {
        if n.left.is_none() {
            return n;
        }

        let mut y = n.left.take().unwrap();
        let y_right = y.right.take();
        n.left = y_right;
        y.right = Some(n);

        return y;
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

    #[test]
    fn delete() {
        let mut b = BinarySearchTree::new(5);
        b.insert(1);
        b.insert(7);
        b.insert(6);
        b.insert(8);
        b.delete_by_merging(&7);
        assert_eq!(true, b.search(&7).is_none());
        assert_eq!(false, b.search(&1).is_none());
    }
}

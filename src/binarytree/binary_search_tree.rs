use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct BinarySearchTree<T: PartialOrd+Clone> {
    val: T,
    left: Option<Rc<RefCell<BinarySearchTree<T>>>>,
    right: Option<Rc<RefCell<BinarySearchTree<T>>>>,
    size: usize,
    // if use rc,it increases reference count infinitely => memory leak
    self_ref: RefCell<Weak<BinarySearchTree<T>>>
}

// Implement `Clone` for `BinarySearchTree`
impl<T: PartialOrd + Clone> Clone for BinarySearchTree<T> {
    fn clone(&self) -> Self {
        BinarySearchTree {
            val: self.val.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
            size: self.size,
            self_ref: self.self_ref.clone(),
        }
    }
}

fn convert_rc_to_rc_refcell<T:PartialOrd+Clone>(rc: Rc<BinarySearchTree<T>>) -> Rc<RefCell<BinarySearchTree<T>>> {
    Rc::new(RefCell::new((*rc).clone()))
}

impl<T: PartialOrd+Clone> BinarySearchTree<T> {
    pub fn new(val: T) -> Rc<RefCell<BinarySearchTree<T>>> {
        let b = Rc::new(BinarySearchTree{
            val,
            left: None,
            right: None,
            size: 1,
            self_ref: RefCell::new(Weak::new())
        });

        *RefCell::borrow_mut(&b.self_ref) = Rc::downgrade(&b);
        return convert_rc_to_rc_refcell(b);
    }

    pub fn insert(&mut self, val: T) {
        if self.val.eq(&val) {
            return
        }

        if self.val.gt(&val) {
            match &mut self.left {
                None => {
                    self.left = Some(BinarySearchTree::new(val))
                },
                Some(node) => RefCell::borrow_mut(node).insert(val),
            }
        } else {
            match &mut self.right {
                None => {
                    self.right = Some(BinarySearchTree::new(val))
                },
                Some(node) => RefCell::borrow_mut(node).insert(val),
            }
        }
    }

    pub fn search(&self, val: &T) -> Option<Rc<BinarySearchTree<T>>> {
        if self.val.eq(val) {
            return self.self_ref.borrow().upgrade()
        }

        if self.val.gt(val) {
            match &self.left {
                None => None,
                Some(node) => {
                    node.borrow().search(val)
                }
            }
        } else {
            match &self.right {
                None => None,
                Some(node) => {
                    node.borrow().search(val)
                }
            }
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
        RefCell::borrow_mut(&b).insert(3);
        RefCell::borrow_mut(&b).insert(7);
        let a = b.borrow().search(&3);
        assert_eq!(3, a.clone().unwrap().val)
    }
}

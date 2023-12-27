use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    val: Rc<RefCell<T>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(val: T, left: Option<Rc<RefCell<Node<T>>>>, right: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Node {
            val: Rc::new(RefCell::new(val)),
            left,
            right,
        }
    }

    // The 'a lifetime parameter indicates that the returned references are valid for as long as the self reference is valid.
    pub fn preorder<>(&self) -> Vec<Rc<RefCell<T>>> {
        let mut li = vec![self.val.clone()];
        let mut left_li = match self.left.clone() {
            None => {vec![]}
            Some(node) => {
                node.borrow().preorder()
            }
        };
        let mut right_li = match self.right.clone() {
            None => {vec![]}
            Some(node) => {
                node.borrow().preorder()
            }
        };

        li.append(&mut left_li);
        li.append(&mut right_li);
        li
    }
}

pub struct A {
    val: i32,
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::binarytree::binarytree::{A, Node};

    #[test]
    fn preorder() {
        let mut n = Node::new(A{val: 1}, Some(Rc::new(RefCell::new(Node::new(A{val: 2}, None, None)))), Some(Rc::new(RefCell::new(Node::new(A{val: 3}, None, None)))));
        let a = n.preorder();
        println!("sdf");
    }
}

use std::cmp::max;
use std::mem::needs_drop;

// Why divide node and tree?
// because I should mutate tree's self box type => impossible!
pub struct Node<T: Ord + Copy> {
    val: T,
    height: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct AVLTree<T: Ord + Copy> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord + Copy> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree {
            root: None,
        }
    }

    fn height(&self, node: &Option<Box<Node<T>>>) -> i32 {
        return match node {
            None => {0}
            Some(n) => {n.height}
        }
    }

    fn balance_factor(&self, node: &Box<Node<T>>) -> i32 {
        self.height(&node.left) - self.height(&node.right)
    }

    fn rotate_right(&self, mut y: Box<Node<T>>) -> Box<Node<T>> {
        match y.left.take() {
            None => {return y}
            Some(mut x) => {
                y.left = x.right.take();
                x.right = Some(y);
                x.right.as_mut().unwrap().height = 1 + max(self.height(&x.right.as_mut().unwrap().left), self.height(&x.right.as_mut().unwrap().right));
                x.height = 1 + max(self.height(&x.left), self.height(&x.right));
                x
            }
        }
    }

    fn rotate_left(&self, mut x: Box<Node<T>>) -> Box<Node<T>> {
        match x.right.take() {
            None => {return x}
            Some(mut y) => {
                x.right = y.left.take();
                y.left = Some(x);
                y.left.as_mut().unwrap().height = 1 + max(self.height(&y.left.as_mut().unwrap().right), self.height(&y.left.as_mut().unwrap().left));
                y.height = self.height(&y.left)+self.height(&y.right);
                y
            }
        }
    }

    pub fn insert(&mut self, val: T) {
        let root = self.root.take();
        self.root = self.insert_rec(root, val)
    }

    fn insert_rec(&mut self, node: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
        let mut node = match node {
            None => {
                return Some(Box::new(Node{
                    val,
                    height: 0,
                    left: None,
                    right: None,
                }))
            }
            Some(mut n) => {
                if val < n.val {
                    n.left = self.insert_rec(n.left, val)
                } else if val > n.val {
                    n.right = self.insert_rec(n.right, val)
                } else {
                    return Some(n)
                }
                n
            }
        };

        node.height = 1 + max(self.height(&node.left), self.height(&node.right));
        let balance = self.balance_factor(&node);

        if balance > 1 && val < node.left.as_ref().unwrap().val {
            return Some(self.rotate_right(node));
        }

        if balance < -1 && val > node.right.as_ref().unwrap().val {
            return Some(self.rotate_left(node));
        }

        if balance > 1 && val > node.left.as_ref().unwrap().val {
            node.left = Some(self.rotate_left(node.left.unwrap()));
            return Some(self.rotate_right(node.left.unwrap()))
        }

        if balance < -1 && val < node.right.as_ref().unwrap().val {
            node.right = Some(self.rotate_right(node.right.unwrap()));
            return Some(self.rotate_left(node.right.unwrap()));
        }

        Some(node)
    }
}

#[cfg(test)]
mod test {
    use crate::binarytree::avltree::AVLTree;

    #[test]
    fn insert() {
        let mut b = AVLTree::new();
        b.insert(1);
        b.insert(9);
        b.insert(5);
        b.insert(6);
        b.insert(3);
    }
}

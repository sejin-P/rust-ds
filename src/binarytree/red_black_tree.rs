// root should be black
// leaf node should be black
// reed node should has 2 black children
// no of black nodes in route to leaf node should be same

use std::cmp::max;

pub struct Node<T: Ord> {
    val: T,
    is_black: bool,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub struct RBTree<T: Ord> {
    root: Option<Box<Node<T>>>
}

impl<T: Ord> RBTree<T> {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

    fn rotate_right(&self, mut y: Box<Node<T>>) -> Box<Node<T>> {
        match y.left.take() {
            None => {return y}
            Some(mut x) => {
                y.left = x.right.take();
                x.right = Some(y);
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
                y
            }
        }
    }

    pub fn insert(&mut self, val: T) {
        // 1. insert like bst
        // 2. inserted node color is red
        // 3. if first inserted => turn color to red
        // 4. if inserted.parent.color is black -> do nothing
        // 5. if inserted.parent.color is red
        // 5-1. if sibling of parent.color(uncle) is red ->
        // turn grand parent's color to red, parent and uncle to black
        // 5-2. if sibling of parent.color(uncle) is black ->
        // 5-2-1. if grandparent - parent - inserted is linear -> rotate grandparent right, turn parent color to black, grandparent to red
        // 5-2-2. triangle -> rotate inserted left, rotate inserted right, change inserted color to black, grand parent to black
        let mut root = self.root.take();
        let mut new_root = self.insert_rec(root, val);
        match new_root {
            None => {
                self.root = new_root;
            }
            Some(mut n) => {
                if !n.is_black {
                    n.is_black = true;
                }
                self.root = Some(n)
            }
        }
    }

    // i don't wanna do it again with rust
    fn insert_rec(&mut self, node: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
        match node {
            None => {
                return Some(Box::new(Node{
                    val,
                    left: None,
                    right: None,
                    is_black: false,
                }))
            }
            Some(mut n) => {
                if val < n.val {
                    let mut left = n.left.take();
                    match left {
                        None => {
                            n.left = Some(Box::new(Node{
                                val,
                                left: None,
                                right: None,
                                is_black: false,
                            }));
                            return Some(n)
                        }
                        Some(mut l) => {
                            if val < l.val {
                                let ll = l.left.take();
                                match ll {
                                    None => {
                                        if l.is_black {
                                            l.left = Some(Box::new(Node{
                                                val,
                                                left: None,
                                                right: None,
                                                is_black: false,
                                            }));
                                            n.left = Some(l);
                                        } else {
                                            let right = n.right.take();
                                            match right {
                                                None => {
                                                    l.left = Some(Box::new(Node{
                                                        val,
                                                        left: None,
                                                        right: None,
                                                        is_black: false,
                                                    }));
                                                    n.left = Some(l);
                                                    n = self.rotate_right(n);
                                                }
                                                Some(mut r) => {
                                                    if !r.is_black {
                                                        n.is_black = false;
                                                        l.is_black = true;
                                                        r.is_black = true;
                                                        l.left = Some(Box::new(Node{
                                                            val,
                                                            left: None,
                                                            right: None,
                                                            is_black: false,
                                                        }));
                                                        n.left = Some(l);
                                                    } else {
                                                        l.left = Some(Box::new(Node{
                                                            val,
                                                            left: None,
                                                            right: None,
                                                            is_black: false,
                                                        }));
                                                        n.left = Some(l);
                                                        n = self.rotate_right(n);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(l_l) => {
                                        l.left = self.insert_rec(Some(l_l), val);
                                    }
                                }
                            } else if val > l.val {

                            }

                        }
                    }

                    return Some(n)
                } else if val > n.val {
                    // same as left
                    return Some(n)
                } else {
                    return Some(n)
                }
            }
        };
    }
}

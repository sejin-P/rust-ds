// root should be black
// leaf node should be black
// reed node should has 2 black children
// no of black nodes in route to leaf node should be same

pub struct Node<T: Ord> {
    val: T,
    is_black: bool,
    height: i32,
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
}

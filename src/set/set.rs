use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct Node<T> {
    key: T,
    parent: Option<Rc<RefCell<Node<T>>>>
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        return Node {
            key: val,
            // actually, node in set need to link to itself, but self referencing in rust is kinda hard.
            parent: None,
        }
    }

    // `find` now returns an Rc<RefCell<Node<T>>>
    pub fn find(node: &Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let parent = node.borrow().parent.clone();
        match parent {
            Some(parent_rc) => Node::find(&parent_rc),
            None => Rc::clone(node),
        }
    }
}


#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::set::set::Node;

    #[test]
    fn find() {
        let n = Rc::new(RefCell::new(Node::new(1)));
        let f = Node::find(&n);
        let k = f.borrow().key;
        assert_eq!(k, 1)
    }
}

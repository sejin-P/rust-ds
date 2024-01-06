use std::cell::RefCell;
use std::rc::Rc;

pub struct AVLTree<T: PartialOrd> {
    val: T,
    left: Option<Rc<RefCell<AVLTree<T>>>>,
    right: Option<Rc<RefCell<AVLTree<T>>>>,
}
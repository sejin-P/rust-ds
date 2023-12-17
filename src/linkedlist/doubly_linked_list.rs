use std::cell::RefCell;
use std::rc::Rc;

pub struct DoubleNode<T> {
    val: T,
    next: Option<Rc<RefCell<DoubleNode<T>>>>,
    prev: Option<Rc<RefCell<DoubleNode<T>>>>
}



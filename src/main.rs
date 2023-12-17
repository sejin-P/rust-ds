use std::cell::RefCell;
use std::rc::Rc;

pub mod linkedlist;
pub mod list;
mod stack;
mod queue;
mod hashmap;

struct SomeStruct<T> {
    val: T,
}

fn main() {
    let a = Rc::new(RefCell::new(SomeStruct { val: 5 }));
    let mut c = a.borrow_mut(); // c is of type RefMut<SomeStruct<i32>>, not &mut Rc<RefCell<SomeStruct<i32>>>

    c.val = 10; // Modify the val field of SomeStruct

    println!("Modified value: {}", c.val); // Prints: Modified value: 10
}

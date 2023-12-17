use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use fnv::FnvHasher;

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

    let mut h = FnvHasher::default();
    let mut a = 1;
    a.hash(&mut h);
    println!("{}", h.finish());
    let mut b = 1;
    b.hash(&mut h);
    println!("{}", h.finish());
}

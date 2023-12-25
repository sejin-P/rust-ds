use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;
use fnv::FnvHasher;
use crate::linkedlist::linked_list::{LinkedList, Node};

// Why do not use set_len? => It's so flaky! Unpredictable!

// The flaky behavior of your test—sometimes succeeding, sometimes failing—when using set_len to set the length of a Vec with uninitialized elements can be attributed to the nature of undefined behavior in Rust (and programming in general).
//
// When you use set_len on a Vec without properly initializing its elements, you're entering the realm of undefined behavior. Here's what that means and why it results in flaky test outcomes:
//
// Accessing Uninitialized Memory
// Unpredictable State: The uninitialized elements in the Vec are in an unpredictable state. The memory for these elements might contain whatever data was previously at that memory location, which can be literally anything.
//
// Non-Deterministic Behavior: Because the state of uninitialized memory is non-deterministic, your program's behavior can vary widely. It might appear to work correctly if the random garbage in memory happens to not cause any immediate issues. Alternatively, it might fail spectacularly if the garbage data leads to invalid operations, crashes, or other errors.
//
// Undefined Behavior in Rust
// Compiler Assumptions: The Rust compiler assumes that your code does not violate the language's safety guarantees. When you introduce undefined behavior, these assumptions are broken, which can lead to optimizations that make your program behave erratically.
//
// Memory Safety Violations: Accessing or manipulating uninitialized memory is a violation of Rust's strict memory safety guarantees. Rust's usual protections (like ownership and borrowing rules) cannot help once undefined behavior is introduced.
//
// Why Tests Behave Flakily
// Dependence on Memory State: The outcome of running code with undefined behavior often depends on the exact state of memory at the time of execution, which can change between runs. This leads to the flakiness you observed—sometimes the tests pass (perhaps the uninitialized memory doesn't immediately cause observable issues), and sometimes they fail (the uninitialized memory leads to problems).
//
// Environmental Factors: Factors such as what other programs are running, the state of the operating system, and even previous executions of the same program can influence the state of uninitialized memory.
//
// Conclusion
// In short, the flakiness of your tests is a direct consequence of undefined behavior caused by using uninitialized memory. This behavior is inherently unpredictable and unreliable. To avoid such issues, always ensure that all elements in a Vec (or any other data structure) are properly initialized before they are accessed. This is a fundamental aspect of safe and reliable programming in Rust.

pub struct Pair<K: PartialEq, V> {
    k: K,
    v: V,
}

impl<K: PartialEq, V> PartialEq for Pair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.k == other.k
    }
}

pub struct ChainingMap<K: Hash + PartialEq, V> {
    m: usize,
    kvs: Vec<LinkedList<Pair<K, Option<V>>>>
}

impl<K: Hash + PartialEq, V> ChainingMap<K, V> {
    pub fn new() -> Self {
        let m = 128;
        let kvs = (0..m).map(|_| LinkedList::new()).collect::<Vec<_>>();
        ChainingMap { m, kvs }
    }

    fn h(&self, k: &K) -> usize {
        let mut hasher = FnvHasher::default();
        (*k).hash(&mut hasher);
        let h = hasher.finish();

        return (h as usize) % self.m;
    }

    pub fn get(&self, k: K) -> Option<V> where V: Clone {
        let h = self.h(&k);
        let s = self.kvs[h].search(Pair{k, v: None});
        s.map(|kv| {
            kv.borrow().val.v.clone().expect("")
        })
    }

    pub fn set(&mut self, k: K, v: V) {
        println!("AS");
        let h = self.h(&k);
        let mut node = self.kvs[h].head();
        while !node.is_none() {
            if node.clone().unwrap().borrow().val.k == k {
                RefCell::borrow_mut(&(node.clone().unwrap())).val.v = Some(v);
                return
            }
        }

        self.kvs[h].push_back(Pair{k, v: Some(v)})
    }

    pub fn remove(&mut self, k: K) {
        let h = self.h(&k);
        let mut prev: Option<Rc<RefCell<Node<Pair<K, Option<V>>>>>> = None;
        let mut node = self.kvs[h].head();
        while !node.clone().is_none() {
            if node.clone().unwrap().borrow().val.k == k {
                if prev.is_none() && node.clone().unwrap().borrow().next.is_none() {
                    self.kvs[h] = LinkedList::new();
                    return;
                }
                if prev.is_none() {
                    self.kvs[h].remove(Pair{k, v: None});
                    return
                }
                RefCell::borrow_mut(&(prev.unwrap())).next = node.clone().unwrap().borrow().next.clone();
                self.kvs[h].decr_len();
            }
            prev = node.clone();
            node = node.clone().unwrap().borrow().next.clone();
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::hashmap::chaining::ChainingMap;

    #[test]
    fn set() {
        let mut m = ChainingMap::new();
        m.set(1, 2);
        m.set(2, 3);
        let a = m.get(2).unwrap();
        assert_eq!(a, 3);
    }
}

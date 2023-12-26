use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::Deref;

use fnv::FnvHasher;

pub struct LinearProbeMap<K: Hash + PartialEq, V> {
    m: usize,
    k: Vec<Option<K>>,
    v: Vec<Option<V>>,
}

impl<K: Hash + PartialEq, V> LinearProbeMap<K, V> {
    pub fn new() -> Self {
        let m = 4;
        let k_arr = (0..m).map(|_| None).collect::<Vec<_>>();
        let v_arr = (0..m).map(|_| None).collect::<Vec<_>>();

        return LinearProbeMap {
            m,
            k: k_arr,
            v: v_arr,
        };
    }

    pub fn h(&mut self, k: &K) -> usize {
        let mut hasher = FnvHasher::default();
        (*k).hash(&mut hasher);
        let h = hasher.finish();

        return (h as usize) % self.m;
    }

    pub fn find_slot(&mut self, k: &K) -> Option<usize> {
        let mut h = self.h(k);
        while !self.k[h].is_none() {
            if self.k[h].as_ref().unwrap() == k {
                return Some(h);
            } else {
                h += 1;
            }
        }

        return None;
    }

    fn double_cap(&mut self) {
        self.m *= 2;
        self.k.resize_with(self.m, { || None });
        self.v.resize_with(self.m, { || None });
    }

    pub fn get(&mut self, k: K) -> Option<&V> {
        match self.find_slot(&k) {
            Some(slot) => return self.v[slot].as_ref(),
            None => None
        }
    }

    pub fn set(&mut self, k: K, v: V) {
        let mut h = self.h(&k);
        let mut iter = 0;
        while self.k[h].is_some() {
            if iter == self.m {
                self.double_cap();
                h = self.h(&k);
                iter = 0;
                continue;
            }
            if self.k[h].as_ref().unwrap() == &k {
                self.k[h] = Some(k);
                self.v[h] = Some(v);
                return;
            }

            h = (h + 1) % self.m;
            iter += 1;
        }

        self.k[h] = Some(k);
        self.v[h] = Some(v);
    }

    pub fn remove(&mut self, k: K) {
        let mut h = self.h(&k);
        let mut iter = 0;
        while self.k[h].is_some() {
            if iter == self.m {
                break;
            }
            if self.k[h].as_ref().unwrap() == &k {
                self.k[h].take();
                self.v[h].take();
                return;
            }

            h = (h + 1) % self.m;
            iter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hashmap::linear_probe::LinearProbeMap;

    #[test]
    fn set() {
        let mut m = LinearProbeMap::new();
        m.set(1, 2);
        m.set(2, 3);
        m.set(3, 4);
        m.set(4, 5);
        m.set(5, 6);
        assert_eq!(*(m.get(5).unwrap()), 6);
    }

    #[test]
    fn remove() {
        let mut m = LinearProbeMap::new();
        m.set(1, 2);
        m.set(2, 3);
        m.remove(1);
        assert_eq!(None, m.get(1))
    }
}

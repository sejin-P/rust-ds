use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct LinearProbeMap<K: Hash + PartialEq, V> {
    hasher: Box<dyn Hasher>,
    m: usize,
    k: Vec<Option<K>>,
    v: Vec<Option<V>>,
}

impl <K: Hash + PartialEq, V> LinearProbeMap<K, V> {
    pub fn new() -> Self {
        let m = 4;
        let mut k_arr = unsafe {
            let mut v = Vec::with_capacity(m);
            v.set_len(m);
            v
        };
        for i in 0..m {
            k_arr[i] = None;
        }
        let mut v_arr = unsafe {
            let mut v = Vec::with_capacity(m);
            v.set_len(m);
            v
        };
        for i in 0..m {
            v_arr[i] = None;
        }

        return LinearProbeMap{
            hasher: Box::new(DefaultHasher::new()),
            m,
            k: k_arr,
            v: v_arr,
        }
    }

    pub fn h(&mut self, k: &K) -> usize {
        k.hash(&mut self.hasher);

        return (self.hasher.finish() as usize)%self.m
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
        self.k.resize_with(self.m, {|| None});
        self.v.resize_with(self.m, {|| None});
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
                continue
            }
            if self.k[h].as_ref().unwrap() == &k {
                self.k[h] = Some(k);
                self.v[h] = Some(v);
                return;
            } else {
                h = (h+1)%self.m;
                iter += 1;
            }
        }

        self.k[h] = Some(k);
        self.v[h] = Some(v);
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
}

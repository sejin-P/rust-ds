pub struct LinearProbeMap<V> {
    m: usize,
    k: Vec<Option<i32>>,
    v: Vec<Option<V>>,
}

impl <V> LinearProbeMap<V> {
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
            m,
            k: k_arr,
            v: v_arr,
        }
    }

    pub fn h(&self, k: i32) -> usize {
        return (k as usize)%self.m
    }

    pub fn find_slot(&self, k: i32) -> Option<usize> {
        let mut h = self.h(k);
        while !self.k[h].is_none() {
            if self.k[h].unwrap() == k {
                return Some(h);
            } else {
                h += 1;
            }
        }

        return None;
    }

    fn double_cap(&mut self) {
        self.m *= 2;
        self.k.resize(self.m, None);
        self.v.resize_with(self.m, {|| None});
    }

    pub fn get(&self, k: i32) -> Option<&V> {
        match self.find_slot(k) {
            Some(slot) => return self.v[slot].as_ref(),
            None => None
        }
    }

    pub fn set(&mut self, k: i32, v: V) {
        let mut h = self.h(k);
        let mut iter = 0;
        while self.k[h].is_some() {
            if iter == self.m {
                self.double_cap();
                h = self.h(k);
                iter = 0;
                continue
            }
            if self.k[h].unwrap() == k {
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

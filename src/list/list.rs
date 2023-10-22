pub struct List<T> {
    array: Vec<Option<T>>,
    cap: usize,
    len: usize,
}

impl <T> List<T> {
    // Specifically, when using the vec![value; count] syntax,
    // the vec! macro tries to clone value count times, even if the value is something like None.
    pub fn new(cap: usize) -> Self {
        let mut arr: Vec<Option<T>> = unsafe {
            let mut v = Vec::with_capacity(cap);
            v.set_len(cap);
            v
        };
        for i in 0..cap {
            arr[i] = None;
        }
        List { array: arr, cap, len: 0 }
    }
    pub fn append(&mut self, t: T) {
        if self.len == self.cap {
            self.double_cap();
        }

        self.array[self.len] = Some(t);
        self.len += 1;
        return
    }

    pub fn get(&mut self, i: usize) -> Option<T> {
        if i < self.len {
            std::mem::replace(&mut self.array[i], None)
        } else {
            None
        }
    }

    fn double_cap(&mut self) {
        let mut new_vec: Vec<Option<T>> = unsafe {
            let mut v = Vec::with_capacity(2*self.cap);
            v.set_len(2*self.cap);
            v
        };
        for i in 0..self.cap {
            new_vec[i] = self.array[i].take();
        }
        for i in self.cap..2*self.cap {
            new_vec[i] = None;
        }

        self.array = new_vec;
        self.cap = 2*self.cap;
    }
}

#[cfg(test)]
mod tests {
    use crate::list::list::List;

    #[test]
    fn append_one() {
        let mut li = List::new(1);
        li.append(1);
        let a = li.get(0).expect("");
        assert_eq!(1, a);
    }

    fn append_exceeded_cap() {
        let mut li = List::new(1);
        li.append(1);
        li.append(2);
        let a = li.get(1).expect("");
        assert_eq!(2, a);
    }

    fn append_any_struct() {
        struct A {}
        let mut li = List::new(1);
        li.append(A{});
    }
}

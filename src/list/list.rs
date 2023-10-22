pub struct List<T: Copy> {
    array: Vec<Option<T>>,
    cap: usize,
    len: usize,
}

impl <T: Copy> List<T> {
    // Specifically, when using the vec![value; count] syntax,
    // the vec! macro tries to clone value count times, even if the value is something like None.
    pub fn new(cap: usize) -> Self {
        List { array: vec![None; cap], cap, len: 0 }
    }
    pub fn append(&mut self, t: T) {
        if self.len == self.cap {
            self.double_cap();
        }

        self.array[self.len] = Some(t);
        self.len += 1;
        return
    }

    pub fn get(&self, i: usize) -> Option<T> {
        return self.array[i]
    }

    fn double_cap(&mut self) {
        let mut new_vec = vec![None; 2*self.cap];
        for i in 0..self.len {
            new_vec[i] = self.array[i].clone();
        }

        self.array = new_vec;
        self.cap += 1;
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
}

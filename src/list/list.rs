pub struct List<T> {
    array: Vec<Option<T>>,
    cap: usize,
    len: usize,
}

impl <T> List<T> {
    pub fn new(cap: usize) -> Self {
        List { array: vec![None; cap], cap, len: 0 }
    }
    pub fn append(&mut self, t: T) {
        if self.array.len() == self.cap {
            self.double_cap();
        }

        self.array[self.array.len()] = t;
        self.len += 1;
        return
    }

    fn double_cap(&mut self) {
        let mut new_vec = vec![None, 2*self.cap];
        for i in 0..self.len {
            new_vec[i] = self.array[i].take();
        }

        self.array = new_vec;
        self.cap += 1;
    }
}

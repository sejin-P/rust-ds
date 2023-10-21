pub struct List<T> {
    array: Vec<T>,
    cap: usize,
}

impl <T> List<T> {
    pub fn new<T>(cap: i32) -> Self {
        List { array: Vec::with_capacity(cap as usize), cap: cap as usize}
    }
    pub fn append(&mut self, t: T) {
        let l = self.array.len();
        if l == self.cap {
            let new_vec = Vec::with_capacity(2*self.cap);
            let mut i = 0;
            while let Some(t) = self.array.pop() {
                new_vec[l-i-1] = t;
                i += 1;
            }
            self = &mut List { array: new_vec, cap: 2 * self.cap };
            return
        }

        self.array[l] = t;
        return
    }
}

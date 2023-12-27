use std::mem::swap;

pub struct Heap<V: PartialOrd> {
    li: Vec<V>
}

impl<V: PartialOrd> Heap<V> {
    fn down(li: &mut Vec<V>, i: usize) {
        if li.len() <= 2*i+1 {
            return
        }

        if li.len() == 2*i+2 {
            if li[i] < li[2*i+1] {
                li.swap(i, 2*i+1);
            }
            return
        }

        if li[i] < li[2*i+1] && li[2*i+2] < li[2*i+1] {
            li.swap(i, 2*i+1);
            Heap::down(li, 2*i+1);
            return
        }

        if li[i] < li[2*i+2] && li[2*i+1] < li[2*i+2] {
            li.swap(i, 2*i+2);
            Heap::down(li, 2*i+2);
            return
        }
    }

    fn up(li: &mut Vec<V>, i: usize) {
        if i == 0 {
            return
        }

        if li[(i-1)/2] < li[i] {
            li.swap((i-1)/2, i);
            Heap::up(li, (i-1)/2);
            return
        }
    }

    pub fn new(mut li: Vec<V>) -> Self {
        for i in (0..li.len()).rev() {
            if i == 0 {
                Heap::down(&mut li, 0);
                break
            }
            if li[(i-1)/2] < li[i] {
                li.swap((i-1)/2, i);
                Heap::down(&mut li, (i-1)/2);
            }
        }

        Heap{
            li
        }
    }

    pub fn insert(&mut self, v: V) {
        self.li.push(v);
        let i = self.li.len()-1;
        Heap::up(&mut self.li, i);
    }

    pub fn get_head(&self) -> Option<&V> {
        if self.li.len() == 0 {
            return None
        }
        return Some(&self.li[0])
    }

    pub fn delete_head(&mut self) {
        if self.li.len() == 0 {
            return;
        }
        let i = self.li.len() - 1;
        self.li.swap(0, i);
        self.li.pop();
        Heap::down(&mut self.li, 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::heap::heap::Heap;

    #[test]
    fn new() {
        let mut a = Heap::new(vec![1, 2, 5, 4, 3]);
        let h = a.get_head();
        assert_eq!(*h.unwrap(), 5);
    }

    #[test]
    fn insert() {
        let mut a = Heap::new(vec![1,3,5,4,2]);
        let h = a.get_head();
        assert_eq!(*h.unwrap(), 5);
        a.insert(3);
        a.insert(7);
        let h = a.get_head();
        assert_eq!(*h.unwrap(), 7);
    }

    #[test]
    fn delete() {
        let mut a= Heap::new(vec![]);
        a.insert(1);
        a.insert(3);
        a.insert(5);
        a.insert(4);
        a.insert(2);
        a.insert(7);
        a.insert(9);
        let h = a.get_head();
        assert_eq!(*h.unwrap(), 9);
        a.delete_head();
        let h = a.get_head();
        assert_eq!(*h.unwrap(), 7);
    }
}

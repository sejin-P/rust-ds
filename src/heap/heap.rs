use std::mem::swap;

pub struct Heap<V: PartialOrd> {
    li: Vec<V>
}

impl<V: PartialOrd> Heap<V> {
    pub fn new(mut li: Vec<V>) -> Self {
        fn down<V: PartialOrd>(li: &mut Vec<V>, i: usize) {
            if li.len() <= 2*i+1 {
                return
            }
            if li[i] < li[2*i+1] {
                li.swap(i, 2*i+1);
                down(li, 2*i+1);
                return
            }
            if li[i] < li[2*i+2] {
                li.swap(i, 2*i+2);
                down(li, 2*i+2);
                return
            }
        }

        for i in (0..li.len()).rev() {
            if i == 0 {
                down(&mut li, 0);
                break
            }
            if li[(i-1)/2] < li[i] {
                li.swap((i-1)/2, i);
                down(&mut li, (i-1)/2);
            }
        }

        Heap{
            li
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::heap::heap::Heap;

    #[test]
    fn new() {
        let mut a = Heap::new(vec![1, 2, 5, 4, 3]);
        print!("a")
    }
}

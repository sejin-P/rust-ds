use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::queue::queue::QueueErr::EmptyErr;

pub struct Queue<T> {
    li: Vec<T>,
}

// TODO implement with using linked list, not using vec type.
impl <T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            li: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, val: T) {
        self.li.push(val)
    }

    pub fn dequeue(&mut self) -> Result<T, QueueErr> {
        if self.is_empty() {
            return Err(QueueErr::EmptyErr())
        }

        return Ok(self.li.remove(0))
    }

    pub fn is_empty(&self) -> bool {
        return self.li.is_empty()
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum QueueErr {
    EmptyErr()
}

impl Display for QueueErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyErr() => write!(f, "empty err"),
        }
    }
}

impl Error for QueueErr {}

#[cfg(test)]
mod tests {
    use super::{Queue, QueueErr};

    #[test]
    fn enqueue_and_dequeue() {
        let mut q = Queue::new();
        q.enqueue(1);
        let v = q.dequeue().unwrap();
        assert_eq!(1, v);
    }
}

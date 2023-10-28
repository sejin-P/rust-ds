use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::stack::stack::StackErr::IndexOutErr;

pub struct Stack<T> {
    li: Vec<T>,
}

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            li: Vec::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.li.push(t);
    }

    pub fn pop(&mut self) -> Result<T, StackErr> {
        if self.li.len() == 0 {
            return Err(IndexOutErr())
        }
        return Ok(self.li.pop().unwrap());
    }

    pub fn top(&self) -> Result<&T, StackErr> {
        if self.li.len() == 0 {
            return Err(IndexOutErr())
        }
        return Ok(&self.li[self.li.len()-1]);
    }

    pub fn len(&self) -> usize {
        return self.li.len()
    }
}

#[derive(Debug)]
enum StackErr {
    IndexOutErr()
}
impl Display for StackErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexOutErr() => write!(f, "index error"),
        }
    }
}

impl Error for StackErr {}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn push_and_pop() {
        let mut st = Stack::new();
        st.push(1);
        let v = st.pop().unwrap();
        assert_eq!(1, v);
    }
}

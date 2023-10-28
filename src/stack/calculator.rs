use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref OPERATORS: HashMap<char, i32> = HashMap::from([
        ('(', 0),
        ('+', 0),
        ('-', 0),
        ('*', 0),
        ('/', 0),
        (')', 0),
    ]);
}

pub struct Calculator {
}

impl Calculator {
    pub fn calculate(formula: String) -> Result<i32, CalculatorErr> {
        let mut opstack = vec![];
        let mut outstack: Vec<char> = vec![];
        let mut was_num = false;
        for token in formula.chars() {
            if OPERATORS.contains_key(&token) {
                was_num = false;
                if token == ')' {
                    while outstack.len() > 0 {
                        let out = outstack.pop();
                        if out.unwrap() == '(' {
                            break
                        }

                        opstack.push(outstack.pop().unwrap().to_string());
                    }

                    continue
                }

                while outstack.len() > 0 {
                    if OPERATORS[&outstack[outstack.len()-1]] >= 3 && OPERATORS[&token] <= 2 {
                        opstack.push(outstack.pop().unwrap().to_string());
                    } else {
                        break
                    }
                }
                outstack.push(token);
                continue
            }

            if was_num {
                let idx = opstack.len()-1;
                opstack[idx].push(token);
            } else {
                opstack.push(String::from(token));
                was_num = true;
            }

        }

        while outstack.len() > 0 {
            opstack.push(outstack.pop().unwrap().to_string());
        }

        let mut val = 0;
        let mut expr_stack = vec![];
        for token in opstack {
            if token.len() > 1 || !OPERATORS.contains_key(&(token.as_bytes()[0] as char)) {
                expr_stack.push(token.parse::<i32>().unwrap());
            } else {
                let a = expr_stack.pop().unwrap();
                let b = expr_stack.pop().unwrap();
                match token.as_str() {
                    "+" => {expr_stack.push(a+b)},
                    "-" => {expr_stack.push(a-b)},
                    "*" => {expr_stack.push(a*b)},
                    "/" => {expr_stack.push(a/b)},
                    _ => {return Err(CalculatorErr::UnavailableTokenErr)}
                }
            }
        }

        return Ok(expr_stack.pop().unwrap())
    }
}

#[derive(Debug)]
pub enum CalculatorErr {
    UnavailableTokenErr
}

#[cfg(test)]
mod tests {
    use super::{Calculator};

    #[test]
    fn a() {
        let a = Calculator::calculate(String::from("1+2*3"));
        assert_eq!(7, a.unwrap())
    }

    fn b() {
        let b = Calculator::calculate(String::from("(1+2)*3"));
        assert_eq!(9, b.unwrap())
    }

    fn c() {
        let c = Calculator::calculate(String::from("(1+2)/3"));
        assert_eq!(1, c.unwrap())
    }

    fn d() {
        let d = Calculator::calculate(String::from("(1+2)*(3*4)"));
        assert_eq!(36, d.unwrap())
    }
}

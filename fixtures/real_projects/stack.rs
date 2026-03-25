/// A simple generic stack implementation.
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

/// Check if parentheses are balanced.
pub fn is_balanced(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

/// Simple calculator that evaluates reverse polish notation.
pub fn eval_rpn(tokens: &[&str]) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    for token in tokens {
        match *token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap_or(0.0);
                let a = stack.pop().unwrap_or(0.0);
                let result = match *token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b != 0.0 {
                            a / b
                        } else {
                            0.0
                        }
                    }
                    _ => 0.0,
                };
                stack.push(result);
            }
            num => {
                if let Ok(n) = num.parse::<f64>() {
                    stack.push(n);
                }
            }
        }
    }
    stack.pop().unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_basic() {
        let mut s = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.len(), 3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert!(s.is_empty());
    }

    #[test]
    fn test_balanced_parens() {
        assert!(is_balanced("()"));
        assert!(is_balanced("([{}])"));
        assert!(!is_balanced("([)]"));
        assert!(!is_balanced("("));
        assert!(is_balanced(""));
    }

    #[test]
    fn test_rpn() {
        assert_eq!(eval_rpn(&["3", "4", "+"]), 7.0);
        assert_eq!(eval_rpn(&["5", "1", "2", "+", "4", "*", "+", "3", "-"]), 14.0);
    }
}

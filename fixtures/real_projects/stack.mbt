pub struct Stack[T] {
  elements : Array[T]
}
pub fn Stack::new() -> Self {
  { elements: Array.new() }
}
pub fn Stack::push(self, item : T) {
  self.elements.push(item)
}
pub fn Stack::pop(self) -> Option[T] {
  self.elements.pop()
}
pub fn Stack::peek(self) -> Option[T] {
  self.elements.last()
}
pub fn Stack::is_empty(self) -> Bool {
  self.elements.is_empty()
}
pub fn Stack::len(self) -> Int {
  self.elements.length()
}

pub fn is_balanced(input : String) -> Bool {
  let mut stack : Array[Char] = Array.new()
  for ch in input.chars() {
    match ch {
      '(' | '[' | '{' => stack.push(ch)
      ')' =>       if stack.pop() != Some('(') {
        return false
      }
      ']' =>       if stack.pop() != Some('[') {
        return false
      }
      '}' =>       if stack.pop() != Some('{') {
        return false
      }
      _ => 
    }
  }
  stack.is_empty()
}
pub fn eval_rpn(tokens : Array[String]) -> Double {
  let mut stack : Array[Double] = Array.new()
  for token in tokens {
    match token {
      "+" | "-" | "*" | "/" => {
        let b = stack.pop().unwrap_or(0.0)
        let a = stack.pop().unwrap_or(0.0)
        let result = match token {
          "+" => a + b
          "-" => a - b
          "*" => a * b
          "/" =>           if b != 0.0 {
            a / b
          } else {
            0.0
          }
          _ => 0.0
        }
        stack.push(result)
      }
      num =>       match num.parse() {
        Ok(n) => {
          stack.push(n)
        }
        _ => ()
      }
    }
  }
  stack.pop().unwrap_or(0.0)
}

test "stack basic" {
  let mut s = Stack.new()
  assert_true(s.is_empty())
  s.push(1)
  s.push(2)
  s.push(3)
  assert_eq(s.length(), 3)
  assert_eq(s.pop(), Some(3))
  assert_eq(s.pop(), Some(2))
  assert_eq(s.pop(), Some(1))
  assert_true(s.is_empty())
}
test "balanced parens" {
  assert_true(is_balanced("()"))
  assert_true(is_balanced("([{}])"))
  assert_true(not(is_balanced("([)]")))
  assert_true(not(is_balanced("(")))
  assert_true(is_balanced(""))
}
test "rpn" {
  assert_eq(eval_rpn(["3", "4", "+"]), 7.0)
  assert_eq(eval_rpn(["5", "1", "2", "+", "4", "*", "+", "3", "-"]), 14.0)
}


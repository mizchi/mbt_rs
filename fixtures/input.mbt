fn add(x : Int, y : Int) -> Int {
  x + y
}

pub fn hello() -> String {
  "hello".to_string()
}

struct Point {
  x : Int
  y : Int
}

pub struct Color {
  r : Byte
  g : Byte
  b : Byte
}

enum Shape {
  Circle(Double)
  Rect(Double, Double)
}

type Score = Int

fn compute() -> Int {
  let x = 1
  let y = 2
  x + y
}

fn counter() -> Int {
  let mut x = 0
  x = 10
  x
}

fn max(a : Int, b : Int) -> Int {
  if a > b {
    a
  } else {
    b
  }
}

const MAX_SIZE : Int = 100

fn swap(a : Int, b : Int) -> (Int, Int) {
  (b, a)
}

fn safe_div(a : Int, b : Int) -> Int? {
  if b == 0 {
    None
  } else {
    Some(a / b)
  }
}

fn sum_to(n : Int) -> Int {
  let mut total = 0
  let mut i = 1
  while i <= n {
    total = total + i
    i = i + 1
  }
  total
}

fn nums() -> Array[Int] {
  [1, 2, 3]
}

type Handle

fn apply(f : (Int) -> Int, x : Int) -> Int {
  f(x)
}

fn sum_arr(arr : Array[Int]) -> Int {
  let mut total = 0
  for x in arr {
    total = total + x
  }
  total
}

fn area(s : Shape) -> Double {
  match s {
    Circle(r) => 3.14 * r * r
    Rect(w, h) => w * h
  }
}

fn classify(x : Int) -> String {
  match x {
    n if n > 0 => "positive"
    n if n < 0 => "negative"
    _ => "zero"
  }
}

fn first_of_pair(pair : (Int, Int)) -> Int {
  let (a, _) = pair
  a
}

fn is_weekend(day : Int) -> Bool {
  match day {
    6 | 7 => true
    _ => false
  }
}

fn check(x : Int) -> Int {
  if x < 0 {
    return 0
  }
  x
}

fn identity[T](x : T) -> T {
  x
}

fn negate(x : Int) -> Int {
  -x
}

fn clamp(x : Int, lo : Int, hi : Int) -> Int {
  if x < lo {
    lo
  } else {
    if x > hi {
      hi
    } else {
      x
    }
  }
}

fn typed() -> Int {
  let x : Int = 42
  x
}

fn unwrap_or(opt : Int?, default : Int) -> Int {
  match opt {
    Some(v) => v
    None => default
  }
}

struct Wrapper(Int)

fn apply_fn(f : (Int) -> Int, x : Int) -> Int {
  f(x)
}

fn move_right(p : Point) -> Point {
  { ..p, x: p.x + 1 }
}

fn classify(x : Int) -> String {
  match x {
    n if n > 0 => "positive"
    n if n < 0 => "negative"
    _ => "zero"
  }
}

fn first_of_pair(pair : (Int, Int)) -> Int {
  let (a, _) = pair
  a
}

fn is_weekend(day : Int) -> Bool {
  match day {
    6 | 7 => true
    _ => false
  }
}

fn negate(x : Int) -> Int {
  -x
}

fn clamp(x : Int, lo : Int, hi : Int) -> Int {
  if x < lo {
    lo
  } else {
    if x > hi {
      hi
    } else {
      x
    }
  }
}

fn typed() -> Int {
  let x : Int = 42
  x
}

fn unwrap_or(opt : Int?, default : Int) -> Int {
  match opt {
    Some(v) => v
    None => default
  }
}

fn identity[T](x : T) -> T {
  x
}

fn divmod(a : Int, b : Int) -> (Int, Int) {
  (a / b, a % b)
}

fn deep(x : Int?) -> Int {
  match x {
    Some(v) => match v {
      0 => 0
      n => n * 2
    }
    None => -1
  }
}

struct Pair[A, B] {
  first : A
  second : B
}

fn complex(x : Int) -> Int {
  let a = x * 2
  let b = a + 1
  let c = if b > 10 { b } else { 10 }
  c
}

fn compare(a : Int, b : Int) -> Bool {
  a == b || a != b && a <= b
}

struct ColorPoint { x : Int; y : Int; color : String } derive(Show, Eq)

fn sum_range() -> Int {
  let mut total = 0
  for i = 0; i < 10; i = i + 1 {
    total = total + i
  }
  total
}

test "addition" {
  assert_eq(1 + 1, 2)
}

enum Expr {
  Lit(Int)
  BinOp(op~ : String, lhs~ : Int, rhs~ : Int)
}

fn print_pairs(arr : Array[(Int, String)]) -> Unit {
  for k, v in arr {
    println(k.to_string())
  }
}

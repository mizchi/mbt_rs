///|
/// Round-trip tests: MoonBit → Rust → (verify structure preserved).
/// These verify the Rust output contains expected patterns.

///|
fn assert_roundtrip(source : String, checks : Array[String]) -> Unit raise {
  let (impls, errors) = @moonbitlang/parser.parse_string(source)
  assert_eq(errors.length(), 0)
  let rust = to_rust(impls)
  for check in checks {
    assert_true(
      rust.contains(check),
      msg="Rust output missing: '\{check}'\n--- output ---\n\{rust}",
    )
  }
}

///|
test "roundtrip: simple fn" {
  assert_roundtrip(
    (
      #|fn add(x : Int, y : Int) -> Int { x + y }
    ),
    ["fn add(", "x: i32", "y: i32", "-> i32", "x + y"],
  )
}

///|
test "roundtrip: struct" {
  assert_roundtrip(
    (
      #|pub struct Point { x : Int; y : Int }
    ),
    ["pub struct Point", "pub x: i32", "pub y: i32"],
  )
}

///|
test "roundtrip: enum" {
  assert_roundtrip(
    (
      #|enum Shape { Circle(Double); Rect(Double, Double) }
    ),
    ["enum Shape", "Circle(f64)", "Rect(f64, f64)"],
  )
}

///|
test "roundtrip: generic fn" {
  assert_roundtrip(
    (
      #|fn identity[T](x : T) -> T { x }
    ),
    ["fn identity<T>(", "x: T", "-> T"],
  )
}

///|
test "roundtrip: option match" {
  assert_roundtrip(
    (
      #|fn unwrap_or(opt : Int?, default : Int) -> Int {
      #|  match opt {
      #|    Some(v) => v
      #|    None => default
      #|  }
      #|}
    ),
    [
      "fn unwrap_or(", "Option<i32>", "match opt", "Some(v) => v", "None => default",
    ],
  )
}

///|
test "roundtrip: if else" {
  assert_roundtrip(
    (
      #|fn max(a : Int, b : Int) -> Int {
      #|  if a > b { a } else { b }
      #|}
    ),
    ["fn max(", "-> i32", "if a > b", "a", "b"],
  )
}

///|
test "roundtrip: let mut" {
  assert_roundtrip(
    (
      #|fn counter() -> Int {
      #|  let mut x = 0
      #|  x = 10
      #|  x
      #|}
    ),
    ["fn counter(", "let mut x = 0", "x = 10"],
  )
}

///|
test "roundtrip: while loop" {
  assert_roundtrip(
    (
      #|fn sum_to(n : Int) -> Int {
      #|  let mut total = 0
      #|  let mut i = 1
      #|  while i <= n {
      #|    total = total + i
      #|    i = i + 1
      #|  }
      #|  total
      #|}
    ),
    ["fn sum_to(", "while i <= n", "total = total + i", "i = i + 1"],
  )
}

///|
test "roundtrip: derive" {
  assert_roundtrip(
    (
      #|struct Point { x : Int; y : Int } derive(Show, Eq)
    ),
    ["#[derive(Debug", "PartialEq, Eq", "struct Point"],
  )
}

///|
test "roundtrip: const" {
  assert_roundtrip(
    (
      #|const MAX : Int = 100
    ),
    ["const MAX: i32 = 100"],
  )
}

///|
test "roundtrip: type alias" {
  assert_roundtrip(
    (
      #|type Score = Int
    ),
    ["type Score = i32"],
  )
}

///|
test "roundtrip: array literal" {
  assert_roundtrip(
    (
      #|fn nums() -> Array[Int] { [1, 2, 3] }
    ),
    ["fn nums(", "Vec<i32>", "vec![1, 2, 3]"],
  )
}

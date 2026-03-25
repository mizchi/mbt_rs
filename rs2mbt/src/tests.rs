use crate::to_moonbit;

fn assert_rs2mbt(rust_src: &str, expected: &str) {
    let result = to_moonbit(rust_src);
    assert_eq!(result.trim(), expected.trim(), "\n--- got ---\n{}\n--- expected ---\n{}", result.trim(), expected.trim());
}

#[test]
fn test_simple_function() {
    assert_rs2mbt(
        "fn add(x: i32, y: i32) -> i32 { x + y }",
        "fn add(x : Int, y : Int) -> Int {\n  x + y\n}",
    );
}

#[test]
fn test_pub_function() {
    assert_rs2mbt(
        "pub fn hello() -> String { String::new() }",
        "pub fn hello() -> String {\n  String.new()\n}",
    );
}

#[test]
fn test_struct() {
    assert_rs2mbt(
        "struct Point { x: i32, y: i32 }",
        "struct Point {\n  x : Int\n  y : Int\n}",
    );
}

#[test]
fn test_enum() {
    assert_rs2mbt(
        "enum Color { Red, Green, Blue }",
        "enum Color {\n  Red\n  Green\n  Blue\n}",
    );
}

#[test]
fn test_enum_with_payload() {
    assert_rs2mbt(
        "enum Shape { Circle(f64), Rect(f64, f64) }",
        "enum Shape {\n  Circle(Double)\n  Rect(Double, Double)\n}",
    );
}

#[test]
fn test_type_alias() {
    assert_rs2mbt(
        "type Score = i32;",
        "type Score = Int",
    );
}

#[test]
fn test_const() {
    assert_rs2mbt(
        "const MAX: i32 = 100;",
        "const MAX : Int = 100",
    );
}

#[test]
fn test_let_binding() {
    assert_rs2mbt(
        "fn foo() -> i32 { let x = 1; x }",
        "fn foo() -> Int {\n  let x = 1\n  x\n}",
    );
}

#[test]
fn test_let_mut() {
    assert_rs2mbt(
        "fn foo() -> i32 { let mut x = 0; x = 10; x }",
        "fn foo() -> Int {\n  let mut x = 0\n  x = 10\n  x\n}",
    );
}

#[test]
fn test_if_else() {
    assert_rs2mbt(
        "fn max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }",
        "fn max(a : Int, b : Int) -> Int {\n  if a > b {\n    a\n  } else {\n    b\n  }\n}",
    );
}

#[test]
fn test_match() {
    assert_rs2mbt(
        r#"fn describe(x: i32) -> String { match x { 0 => String::new(), _ => String::new() } }"#,
        "fn describe(x : Int) -> String {\n  match x {\n    0 => String.new()\n    _ => String.new()\n  }\n}",
    );
}

#[test]
fn test_generic_function() {
    assert_rs2mbt(
        "fn identity<T>(x: T) -> T { x }",
        "fn identity[T](x : T) -> T {\n  x\n}",
    );
}

#[test]
fn test_tuple() {
    assert_rs2mbt(
        "fn swap(a: i32, b: i32) -> (i32, i32) { (b, a) }",
        "fn swap(a : Int, b : Int) -> (Int, Int) {\n  (b, a)\n}",
    );
}

#[test]
fn test_array_literal() {
    assert_rs2mbt(
        "fn nums() -> Vec<i32> { vec![1, 2, 3] }",
        "fn nums() -> Array[Int] {\n  [1 , 2 , 3]\n}",
    );
}

#[test]
fn test_while_loop() {
    assert_rs2mbt(
        "fn count() { let mut i = 0; while i < 10 { i = i + 1; } }",
        "fn count() {\n  let mut i = 0\n  while i < 10 {\n    i = i + 1\n  }\n}",
    );
}

#[test]
fn test_for_loop() {
    assert_rs2mbt(
        "fn sum(arr: Vec<i32>) -> i32 { let mut t = 0; for x in arr { t = t + x; } t }",
        "fn sum(arr : Array[Int]) -> Int {\n  let mut t = 0\n  for x in arr {\n    t = t + x\n  }\n  t\n}",
    );
}

#[test]
fn test_option_type() {
    assert_rs2mbt(
        "fn find(x: i32) -> Option<i32> { None }",
        "fn find(x : Int) -> Option[Int] {\n  None\n}",
    );
}

#[test]
fn test_unary_neg() {
    assert_rs2mbt(
        "fn negate(x: i32) -> i32 { -x }",
        "fn negate(x : Int) -> Int {\n  -x\n}",
    );
}

#[test]
fn test_closure() {
    assert_rs2mbt(
        "fn apply(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }",
        "fn apply(f : (Int) -> Int, x : Int) -> Int {\n  f(x)\n}",
    );
}

#[test]
fn test_unit_return_omitted() {
    assert_rs2mbt(
        "fn noop() {}",
        "fn noop() {\n\n}",
    );
}

#[test]
fn test_derive() {
    assert_rs2mbt(
        "#[derive(Debug, PartialEq, Eq)]\nstruct Point { x: i32, y: i32 }",
        "derive(Show, Eq)struct Point {\n  x : Int\n  y : Int\n}",
    );
}

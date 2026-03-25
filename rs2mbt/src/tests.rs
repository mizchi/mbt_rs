use crate::to_moonbit;

fn assert_rs2mbt(rust_src: &str, expected: &str) {
    let result = to_moonbit(rust_src);
    assert_eq!(result.trim(), expected.trim(), "\n--- got ---\n{}\n--- expected ---\n{}", result.trim(), expected.trim());
}

// === Basic functions ===

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
        "pub fn hello() -> String {\n  \"\"\n}",
    );
}

#[test]
fn test_unit_return_omitted() {
    assert_rs2mbt(
        "fn noop() {}",
        "fn noop() -> Unit {\n\n}",
    );
}

#[test]
fn test_generic_function() {
    assert_rs2mbt(
        "fn identity<T>(x: T) -> T { x }",
        "fn[T] identity(x : T) -> T {\n  x\n}",
    );
}

// === Type definitions ===

#[test]
fn test_struct() {
    assert_rs2mbt(
        "struct Point { x: i32, y: i32 }",
        "struct Point {\n  x : Int\n  y : Int\n}",
    );
}

#[test]
fn test_pub_struct() {
    assert_rs2mbt(
        "pub struct Color { pub r: u8, pub g: u8, pub b: u8 }",
        "pub struct Color {\n  r : Byte\n  g : Byte\n  b : Byte\n}",
    );
}

#[test]
fn test_tuple_struct() {
    assert_rs2mbt(
        "struct Wrapper(i32);",
        "struct Wrapper(Int)",
    );
}

#[test]
fn test_generic_struct() {
    assert_rs2mbt(
        "struct Pair<A, B> { first: A, second: B }",
        "struct Pair[A, B] {\n  first : A\n  second : B\n}",
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
fn test_enum_struct_variant() {
    assert_rs2mbt(
        "enum Expr { Lit(i32), BinOp { op: String, lhs: i32, rhs: i32 } }",
        "enum Expr {\n  Lit(Int)\n  BinOp(op~ : String, lhs~ : Int, rhs~ : Int)\n}",
    );
}

#[test]
fn test_generic_enum() {
    assert_rs2mbt(
        "enum MyResult<T, E> { Ok(T), Err(E) }",
        "enum MyResult[T, E] {\n  Ok(T)\n  Err(E)\n}",
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
fn test_derive() {
    assert_rs2mbt(
        "#[derive(Debug, PartialEq, Eq)]\nstruct Point { x: i32, y: i32 }",
        "struct Point {\n  x : Int\n  y : Int\n} derive(Show, Eq)",
    );
}

// === Let bindings ===

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
fn test_let_with_type() {
    assert_rs2mbt(
        "fn foo() -> i32 { let x: i32 = 42; x }",
        "fn foo() -> Int {\n  let x : Int = 42\n  x\n}",
    );
}

#[test]
fn test_let_tuple_destructure() {
    assert_rs2mbt(
        "fn foo() -> i32 { let (a, _) = (1, 2); a }",
        "fn foo() -> Int {\n  let (a, _) = (1, 2)\n  a\n}",
    );
}

#[test]
fn test_multiple_lets() {
    assert_rs2mbt(
        "fn foo() -> i32 { let a = 1; let b = 2; let c = 3; a + b + c }",
        "fn foo() -> Int {\n  let a = 1\n  let b = 2\n  let c = 3\n  a + b + c\n}",
    );
}

// === Control flow ===

#[test]
fn test_if_else() {
    assert_rs2mbt(
        "fn max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }",
        "fn max(a : Int, b : Int) -> Int {\n  if a > b {\n    a\n  } else {\n    b\n  }\n}",
    );
}

#[test]
fn test_nested_if() {
    assert_rs2mbt(
        "fn clamp(x: i32, lo: i32, hi: i32) -> i32 { if x < lo { lo } else { if x > hi { hi } else { x } } }",
        "fn clamp(x : Int, lo : Int, hi : Int) -> Int {\n  if x < lo {\n    lo\n  } else {\n    if x > hi {\n      hi\n    } else {\n      x\n    }\n  }\n}",
    );
}

#[test]
fn test_match() {
    assert_rs2mbt(
        r#"fn describe(x: i32) -> &str { match x { 0 => "zero", _ => "other" } }"#,
        "fn describe(x : Int) -> String {\n  match x {\n    0 => \"zero\"\n    _ => \"other\"\n  }\n}",
    );
}

#[test]
fn test_match_with_guard() {
    assert_rs2mbt(
        r#"fn classify(x: i32) -> &str { match x { n if n > 0 => "pos", _ => "other" } }"#,
        "fn classify(x : Int) -> String {\n  match x {\n    n if n > 0 => \"pos\"\n    _ => \"other\"\n  }\n}",
    );
}

#[test]
fn test_match_option() {
    assert_rs2mbt(
        "fn unwrap_or(opt: Option<i32>, default: i32) -> i32 { match opt { Some(v) => v, None => default } }",
        "fn unwrap_or(opt : Option[Int], default : Int) -> Int {\n  match opt {\n    Some(v) => v\n    None => default\n  }\n}",
    );
}

#[test]
fn test_match_or_pattern() {
    assert_rs2mbt(
        "fn is_weekend(day: i32) -> bool { match day { 6 | 7 => true, _ => false } }",
        "fn is_weekend(day : Int) -> Bool {\n  match day {\n    6 | 7 => true\n    _ => false\n  }\n}",
    );
}

#[test]
fn test_while_loop() {
    assert_rs2mbt(
        "fn count() { let mut i = 0; while i < 10 { i = i + 1; } }",
        "fn count() -> Unit {\n  let mut i = 0\n  while i < 10 {\n    i = i + 1\n  }\n}",
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
fn test_return_early() {
    assert_rs2mbt(
        "fn check(x: i32) -> i32 { if x < 0 { return 0; } x }",
        "fn check(x : Int) -> Int {\n  if x < 0 {\n    return 0\n  }\n  x\n}",
    );
}

#[test]
fn test_break_continue() {
    assert_rs2mbt(
        "fn find() -> i32 { let mut i = 0; while true { if i > 5 { break; } i = i + 1; } i }",
        "fn find() -> Int {\n  let mut i = 0\n  while true {\n    if i > 5 {\n      break\n    }\n    i = i + 1\n  }\n  i\n}",
    );
}

// === Expressions ===

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
        "fn nums() -> Array[Int] {\n  [1, 2, 3]\n}",
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
fn test_not_operator() {
    assert_rs2mbt(
        "fn invert(b: bool) -> bool { !b }",
        "fn invert(b : Bool) -> Bool {\n  not(b)\n}",
    );
}

#[test]
fn test_comparison_operators() {
    assert_rs2mbt(
        "fn cmp(a: i32, b: i32) -> bool { a == b || a != b && a <= b }",
        "fn cmp(a : Int, b : Int) -> Bool {\n  a == b || a != b && a <= b\n}",
    );
}

#[test]
fn test_field_access() {
    assert_rs2mbt(
        "fn get_x(p: Point) -> i32 { p.x }",
        "fn get_x(p : Point) -> Int {\n  p.x\n}",
    );
}

#[test]
fn test_method_call() {
    assert_rs2mbt(
        "fn len(s: String) -> usize { s.len() }",
        "fn len(s : String) -> Int {\n  s.length()\n}",
    );
}

#[test]
fn test_method_call_with_arg() {
    assert_rs2mbt(
        "fn push_val(arr: Vec<i32>, v: i32) { arr.push(v); }",
        "fn push_val(arr : Array[Int], v : Int) -> Unit {\n  arr.push(v)\n}",
    );
}

#[test]
fn test_chained_method() {
    assert_rs2mbt(
        "fn process(s: String) -> usize { s.trim().len() }",
        "fn process(s : String) -> Int {\n  s.trim().length()\n}",
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
fn test_closure_expr() {
    assert_rs2mbt(
        "fn foo() -> i32 { let f = |x: i32| x + 1; f(10) }",
        "fn foo() -> Int {\n  let f = fn(x : Int) -> Unit { x + 1 }\n  f(10)\n}",
    );
}

#[test]
fn test_index_access() {
    assert_rs2mbt(
        "fn first(arr: Vec<i32>) -> i32 { arr[0] }",
        "fn first(arr : Array[Int]) -> Int {\n  arr[0]\n}",
    );
}

#[test]
fn test_index_assign() {
    assert_rs2mbt(
        "fn set_first(arr: Vec<i32>) { arr[0] = 99; }",
        "fn set_first(arr : Array[Int]) -> Unit {\n  arr[0] = 99\n}",
    );
}

#[test]
fn test_struct_literal() {
    assert_rs2mbt(
        "fn origin() -> Point { Point { x: 0, y: 0 } }",
        "fn origin() -> Point {\n  { x: 0, y: 0 }\n}",
    );
}

#[test]
fn test_record_update() {
    assert_rs2mbt(
        "fn move_right(p: Point) -> Point { Point { x: p.x + 1, ..p } }",
        "fn move_right(p : Point) -> Point {\n  { ..p, x: p.x + 1 }\n}",
    );
}

#[test]
fn test_cast() {
    assert_rs2mbt(
        "fn to_i64(x: i32) -> i64 { x as i64 }",
        "fn to_i64(x : Int) -> Int64 {\n  (x : Int64)\n}",
    );
}

#[test]
fn test_try_operator() {
    assert_rs2mbt(
        "fn maybe() -> Result<i32, String> { let x = foo()?; Ok(x) }",
        "fn maybe() -> Result[Int, String] {\n  let x = foo()!\n  Ok(x)\n}",
    );
}

#[test]
fn test_assert_macro() {
    assert_rs2mbt(
        "fn check() { assert_eq!(1, 1); }",
        "fn check() -> Unit {\n  assert_eq(1, 1)\n}",
    );
}

#[test]
fn test_println_macro() {
    assert_rs2mbt(
        r#"fn hello() { println!("hello"); }"#,
        "fn hello() -> Unit {\n  println(\"hello\")\n}",
    );
}

#[test]
fn test_todo_macro() {
    assert_rs2mbt(
        "fn placeholder() -> i32 { todo!() }",
        "fn placeholder() -> Int {\n  ...\n}",
    );
}

// === Trait ===

#[test]
fn test_trait() {
    assert_rs2mbt(
        "trait Printable { fn to_string(&self) -> String; }",
        "trait Printable {\n  to_string(self) -> String\n}",
    );
}

#[test]
fn test_trait_with_super() {
    assert_rs2mbt(
        "trait Drawable: Display { fn draw(&self); }",
        "trait Drawable : Show {\n  draw(self) -> Unit\n}",
    );
}

// === Types ===

#[test]
fn test_all_numeric_types() {
    assert_rs2mbt(
        "fn types(a: i32, b: u32, c: i64, d: u64, e: f32, f: f64) {}",
        "fn types(a : Int, b : UInt, c : Int64, d : UInt64, e : Float, f : Double) -> Unit {\n\n}",
    );
}

#[test]
fn test_reference_dropped() {
    assert_rs2mbt(
        "fn len(s: &str) -> usize { s.len() }",
        "fn len(s : String) -> Int {\n  s.length()\n}",
    );
}

// === Lifetime / Pointer stripping ===
// MoonBit is GC'd, so references, lifetimes, Box, Rc, Arc are all stripped.

#[test]
fn test_mut_ref_dropped() {
    assert_rs2mbt(
        "fn inc(x: &mut i32) { *x = *x + 1; }",
        "fn inc(x : Int) -> Unit {\n  x = x + 1\n}",
    );
}

#[test]
fn test_box_unwrapped() {
    assert_rs2mbt(
        "fn unbox(b: Box<i32>) -> i32 { *b }",
        "fn unbox(b : Int) -> Int {\n  b\n}",
    );
}

#[test]
fn test_box_in_struct() {
    assert_rs2mbt(
        "struct Node { value: i32, next: Option<Box<Node>> }",
        "struct Node {\n  value : Int\n  next : Option[Node]\n}",
    );
}

#[test]
fn test_rc_stripped_with_note() {
    assert_rs2mbt(
        "fn get_val(r: Rc<String>) -> String { r.clone() }",
        "// NOTE: The following Rust ownership/synchronization types were stripped\n// during conversion because MoonBit is garbage-collected:\n//   Rc<T> (shared ownership, reference counted) → T\n\nfn get_val(r : String) -> String {\n  r\n}",
    );
}

#[test]
fn test_arc_stripped_with_note() {
    assert_rs2mbt(
        "fn get_val(a: Arc<i32>) -> i32 { *a }",
        "// NOTE: The following Rust ownership/synchronization types were stripped\n// during conversion because MoonBit is garbage-collected:\n//   Arc<T> (thread-safe shared ownership) → T\n\nfn get_val(a : Int) -> Int {\n  a\n}",
    );
}

#[test]
fn test_lifetime_in_fn_dropped() {
    assert_rs2mbt(
        "fn first<'a>(s: &'a str) -> &'a str { s }",
        "fn first(s : String) -> String {\n  s\n}",
    );
}

#[test]
fn test_lifetime_in_struct_dropped() {
    assert_rs2mbt(
        "struct Ref<'a> { data: &'a str }",
        "struct Ref {\n  data : String\n}",
    );
}

#[test]
fn test_multiple_lifetimes_dropped() {
    assert_rs2mbt(
        "fn longer<'a, 'b>(a: &'a str, b: &'b str) -> &'a str { if a.len() > b.len() { a } else { b } }",
        "fn longer(a : String, b : String) -> String {\n  if a.length() > b.length() {\n    a\n  } else {\n    b\n  }\n}",
    );
}

#[test]
fn test_lifetime_with_type_param() {
    assert_rs2mbt(
        "fn wrap<'a, T>(val: &'a T) -> &'a T { val }",
        "fn[T] wrap(val : T) -> T {\n  val\n}",
    );
}

#[test]
fn test_cow_unwrapped() {
    assert_rs2mbt(
        "fn to_owned(s: Cow<str>) -> String { s.into_owned() }",
        "fn to_owned(s : String) -> String {\n  s.into_owned()\n}",
    );
}

#[test]
fn test_cell_unwrapped() {
    assert_rs2mbt(
        "fn get_cell(c: Cell<i32>) -> i32 { c.get() }",
        "fn get_cell(c : Int) -> Int {\n  c.get()\n}",
    );
}

#[test]
fn test_refcell_stripped_with_note() {
    assert_rs2mbt(
        "fn borrow_val(r: RefCell<String>) -> String { r.borrow().clone() }",
        "// NOTE: The following Rust ownership/synchronization types were stripped\n// during conversion because MoonBit is garbage-collected:\n//   RefCell<T> (interior mutability, runtime borrow check) → T\n\nfn borrow_val(r : String) -> String {\n  r\n}",
    );
}

#[test]
fn test_deref_expr_dropped() {
    assert_rs2mbt(
        "fn read(p: &i32) -> i32 { *p }",
        "fn read(p : Int) -> Int {\n  p\n}",
    );
}

#[test]
fn test_raw_pointer_dropped() {
    assert_rs2mbt(
        "fn read_ptr(p: *const i32) -> i32 { 0 }",
        "fn read_ptr(p : Int) -> Int {\n  0\n}",
    );
}

#[test]
fn test_complex_ownership_chain() {
    // Box<Vec<&str>> → Array[String]
    assert_rs2mbt(
        "fn get_items(items: Box<Vec<&str>>) -> Vec<&str> { *items }",
        "fn get_items(items : Array[String]) -> Array[String] {\n  items\n}",
    );
}

#[test]
fn test_option_ref_simplified() {
    // Option<&T> → Option[T]
    assert_rs2mbt(
        "fn find(v: &Vec<i32>, key: i32) -> Option<&i32> { None }",
        "fn find(v : Array[Int], key : Int) -> Option[Int] {\n  None\n}",
    );
}

#[test]
fn test_result_with_box_error() {
    // Result<T, Box<dyn Error>> → Result[T, Error]
    assert_rs2mbt(
        "fn parse(s: &str) -> Result<i32, Box<dyn Error>> { Ok(0) }",
        "fn parse(s : String) -> Result[Int, Error] {\n  Ok(0)\n}",
    );
}

#[test]
fn test_if_let_is_pattern() {
    assert_rs2mbt(
        "fn try_parse(s: &str) -> i32 { if let Ok(n) = s.parse::<i32>() { n } else { 0 } }",
        "fn try_parse(s : String) -> Int {\n  if s.parse() is Ok(n) {\n    n\n  } else {\n    0\n  }\n}",
    );
}

#[test]
fn test_self_resolved_in_impl() {
    assert_rs2mbt(
        "impl Stack { fn new() -> Self { Stack { elements: Vec::new() } } }",
        "fn Stack::new() -> Stack {\n  { elements: [] }\n}",
    );
}

#[test]
fn test_impl_with_lifetime() {
    assert_rs2mbt(
        "impl<'a> Ref<'a> { fn new(data: &'a str) -> Self { Ref { data } } }",
        "fn Ref::new(data : String) -> Ref {\n  { data: data }\n}",
    );
}

// === New patterns ===

#[test]
fn test_if_let_to_match() {
    assert_rs2mbt(
        "fn extract(opt: Option<i32>) -> i32 { if let Some(x) = opt { x } else { 0 } }",
        "fn extract(opt : Option[Int]) -> Int {\n  if opt is Some(x) {\n    x\n  } else {\n    0\n  }\n}",
    );
}

#[test]
fn test_infinite_loop() {
    assert_rs2mbt(
        "fn spin() { loop { break; } }",
        "fn spin() -> Unit {\n  while true {\n    break\n  }\n}",
    );
}

#[test]
fn test_range_exclusive() {
    assert_rs2mbt(
        "fn range_test() -> Unit { for i in 0..10 {} }",
        "fn range_test() -> Unit {\n  for i in 0..<10 {\n\n  }\n}",
    );
}

#[test]
fn test_range_inclusive() {
    assert_rs2mbt(
        "fn range_test() -> Unit { for i in 0..=10 {} }",
        "fn range_test() -> Unit {\n  for i in 0..=10 {\n\n  }\n}",
    );
}

#[test]
fn test_augmented_assign() {
    assert_rs2mbt(
        "fn inc(x: &mut i32) { *x += 1; }",
        "fn inc(x : Int) -> Unit {\n  x += 1\n}",
    );
}

#[test]
fn test_string_literal() {
    assert_rs2mbt(
        r#"fn greeting() -> &str { "hello world" }"#,
        "fn greeting() -> String {\n  \"hello world\"\n}",
    );
}

#[test]
fn test_bool_literal() {
    assert_rs2mbt(
        "fn yes() -> bool { true }",
        "fn yes() -> Bool {\n  true\n}",
    );
}

#[test]
fn test_char_literal() {
    assert_rs2mbt(
        "fn ch() -> char { 'a' }",
        "fn ch() -> Char {\n  'a'\n}",
    );
}

#[test]
fn test_i64_literal() {
    assert_rs2mbt(
        "fn big() -> i64 { 42i64 }",
        "fn big() -> Int64 {\n  42L\n}",
    );
}

#[test]
fn test_u32_literal() {
    assert_rs2mbt(
        "fn unsigned() -> u32 { 42u32 }",
        "fn unsigned() -> UInt {\n  42U\n}",
    );
}

#[test]
fn test_float_literal() {
    assert_rs2mbt(
        "fn pi() -> f64 { 3.14 }",
        "fn pi() -> Double {\n  3.14\n}",
    );
}

#[test]
fn test_multiple_match_arms() {
    assert_rs2mbt(
        r#"fn describe(x: i32) -> &str {
            match x {
                1 => "one",
                2 => "two",
                3 => "three",
                _ => "other",
            }
        }"#,
        "fn describe(x : Int) -> String {\n  match x {\n    1 => \"one\"\n    2 => \"two\"\n    3 => \"three\"\n    _ => \"other\"\n  }\n}",
    );
}

#[test]
fn test_nested_match() {
    assert_rs2mbt(
        "fn deep(x: Option<i32>) -> i32 { match x { Some(v) => match v { 0 => 0, n => n * 2, }, None => -1, } }",
        "fn deep(x : Option[Int]) -> Int {\n  match x {\n    Some(v) => match v {\n      0 => 0\n      n => n * 2\n    }\n    None => -1\n  }\n}",
    );
}

#[test]
fn test_match_tuple_pattern() {
    assert_rs2mbt(
        "fn classify(pair: (i32, i32)) -> i32 { match pair { (0, _) => 0, (_, 0) => 1, _ => 2, } }",
        "fn classify(pair : (Int, Int)) -> Int {\n  match pair {\n    (0, _) => 0\n    (_, 0) => 1\n    _ => 2\n  }\n}",
    );
}

#[test]
fn test_complex_body() {
    assert_rs2mbt(
        "fn complex(x: i32) -> i32 { let a = x * 2; let b = a + 1; if b > 10 { b } else { 10 } }",
        "fn complex(x : Int) -> Int {\n  let a = x * 2\n  let b = a + 1\n  if b > 10 {\n    b\n  } else {\n    10\n  }\n}",
    );
}

#[test]
fn test_multiple_params_no_return() {
    assert_rs2mbt(
        "fn log(msg: &str, level: i32) {}",
        "fn log(msg : String, level : Int) -> Unit {\n\n}",
    );
}

#[test]
fn test_generic_with_bounds() {
    assert_rs2mbt(
        "fn print_it<T: Display>(x: T) {}",
        "fn[T : Show] print_it(x : T) -> Unit {\n\n}",
    );
}

#[test]
fn test_trait_multiple_methods() {
    assert_rs2mbt(
        "trait Collection { fn size(&self) -> usize; fn is_empty(&self) -> bool; }",
        "trait Collection {\n  size(self) -> Int\n  is_empty(self) -> Bool\n}",
    );
}

#[test]
fn test_paren_expr() {
    assert_rs2mbt(
        "fn foo(a: i32, b: i32) -> i32 { (a + b) * 2 }",
        "fn foo(a : Int, b : Int) -> Int {\n  (a + b) * 2\n}",
    );
}

#[test]
fn test_array_repeat() {
    assert_rs2mbt(
        "fn zeros() -> Vec<i32> { vec![0; 10] }",
        // vec![0; 10] parses differently - as repeat expr
        "fn zeros() -> Array[Int] {\n  [0 ; 10]\n}",
    );
}

#[test]
fn test_field_assign() {
    assert_rs2mbt(
        "fn set_x(p: &mut Point) { p.x = 42; }",
        "fn set_x(p : Point) -> Unit {\n  p.x = 42\n}",
    );
}

#[test]
fn test_for_tuple_destructure() {
    assert_rs2mbt(
        "fn sum_pairs(pairs: Vec<(i32, i32)>) -> i32 { let mut t = 0; for (a, b) in pairs { t = t + a + b; } t }",
        "fn sum_pairs(pairs : Array[(Int, Int)]) -> Int {\n  let mut t = 0\n  for _item in pairs {\n    let (a, b) = _item\n    t = t + a + b\n  }\n  t\n}",
    );
}

#[test]
fn test_derive_hash() {
    assert_rs2mbt(
        "#[derive(Hash)]\nstruct Key { id: i32 }",
        "struct Key {\n  id : Int\n} derive(Hash)",
    );
}

#[test]
fn test_empty_struct() {
    assert_rs2mbt(
        "struct Unit;",
        "struct Unit",
    );
}

// === New patterns round 3 ===

#[test]
fn test_static_const() {
    assert_rs2mbt(
        "static MAX: i32 = 100;",
        "const MAX : Int = 100",
    );
}

#[test]
fn test_static_mut() {
    assert_rs2mbt(
        "static mut COUNTER: i32 = 0;",
        "let mut COUNTER : Int = 0",
    );
}

#[test]
fn test_inherent_impl() {
    assert_rs2mbt(
        "impl Point { fn new(x: i32, y: i32) -> Point { Point { x, y } } }",
        "fn Point::new(x : Int, y : Int) -> Point {\n  { x: x, y: y }\n}",
    );
}

#[test]
fn test_inherent_impl_method() {
    assert_rs2mbt(
        "impl Point { fn distance(&self) -> f64 { 0.0 } }",
        "fn Point::distance(self : Point) -> Double {\n  0.0\n}",
    );
}

#[test]
fn test_matches_macro() {
    assert_rs2mbt(
        "fn is_some(x: Option<i32>) -> bool { matches!(x, Some(_)) }",
        "fn is_some(x : Option[Int]) -> Bool {\n  x is Some (_)\n}",
    );
}

#[test]
fn test_while_let() {
    assert_rs2mbt(
        "fn drain(v: &mut Vec<i32>) { while let Some(x) = v.pop() { println!(\"{}\", x); } }",
        "fn drain(v : Array[Int]) -> Unit {\n  // while let → loop+match\n  while true {\n    match v.pop() {\n      Some(x) => {\n        println(\"{}\", x)\n      }\n      _ => break\n    }\n  }\n}",
    );
}

#[test]
fn test_closure_with_return_type() {
    assert_rs2mbt(
        "fn foo() -> i32 { let f = |x: i32| -> i32 { x + 1 }; f(10) }",
        "fn foo() -> Int {\n  let f = fn(x : Int) -> Int {   x + 1 }\n  f(10)\n}",
    );
}

#[test]
fn test_nested_method_calls() {
    assert_rs2mbt(
        "fn count(v: Vec<i32>) -> usize { v.iter().filter(|x| *x > 0).count() }",
        "fn count(v : Array[Int]) -> Int {\n  v.iter().filter(fn(x) -> Unit { x > 0 }).length()\n}",
    );
}

#[test]
fn test_multiple_trait_bounds() {
    assert_rs2mbt(
        "fn show<T: Display + Debug>(x: T) {}",
        "fn[T : Show + Debug] show(x : T) -> Unit {\n\n}",
    );
}

#[test]
fn test_method_to_string() {
    assert_rs2mbt(
        "fn stringify(x: i32) -> String { x.to_string() }",
        "fn stringify(x : Int) -> String {\n  x.to_string()\n}",
    );
}

#[test]
fn test_method_is_empty() {
    assert_rs2mbt(
        "fn check(v: Vec<i32>) -> bool { v.is_empty() }",
        "fn check(v : Array[Int]) -> Bool {\n  v.is_empty()\n}",
    );
}

#[test]
fn test_method_clone() {
    assert_rs2mbt(
        "fn dup(s: String) -> String { s.clone() }",
        "fn dup(s : String) -> String {\n  s\n}",
    );
}

#[test]
fn test_impl_trait_with_body() {
    assert_rs2mbt(
        "impl Display for Point { fn fmt(&self, f: &mut Formatter) -> Result<(), Error> { Ok(()) } }",
        "impl Show for Point with fmt(self, f : Formatter) -> Result[Unit, Error] {\n  Ok(())\n}",
    );
}

#[test]
fn test_multiline_fn() {
    assert_rs2mbt(
        r#"fn fibonacci(n: i32) -> i32 {
            if n <= 1 {
                return n;
            }
            let mut a = 0;
            let mut b = 1;
            let mut i = 2;
            while i <= n {
                let temp = a + b;
                a = b;
                b = temp;
                i = i + 1;
            }
            b
        }"#,
        "fn fibonacci(n : Int) -> Int {\n  if n <= 1 {\n    return n\n  }\n  let mut a = 0\n  let mut b = 1\n  let mut i = 2\n  while i <= n {\n    let temp = a + b\n    a = b\n    b = temp\n    i = i + 1\n  }\n  b\n}",
    );
}

#[test]
fn test_enum_with_derive_multiple() {
    assert_rs2mbt(
        "#[derive(Debug, Clone, PartialEq, Eq, Hash)]\nenum Token { Ident(String), Number(i32), Plus, Minus }",
        "enum Token {\n  Ident(String)\n  Number(Int)\n  Plus\n  Minus\n} derive(Show, Eq, Hash)",
    );
}

#[test]
fn test_complex_match_body() {
    assert_rs2mbt(
        r#"fn eval(expr: Expr) -> i32 {
            match expr {
                Expr::Lit(n) => n,
                Expr::Add(a, b) => eval(*a) + eval(*b),
                _ => 0,
            }
        }"#,
        "fn eval(expr : Expr) -> Int {\n  match expr {\n    Expr::Lit(n) => n\n    Expr::Add(a, b) => eval(a) + eval(b)\n    _ => 0\n  }\n}",
    );
}

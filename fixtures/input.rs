use std::rc::Rc;
use std::sync::Arc;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn compute() -> i32 {
    let x = 1;
    let y = 2;
    x + y
}

fn counter() -> i32 {
    let mut x = 0;
    x = 10;
    x
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Red,
    Green,
    Blue,
}

enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

type Score = i32;

const MAX_SIZE: i32 = 100;

fn sum_to(n: i32) -> i32 {
    let mut total = 0;
    let mut i = 1;
    while i <= n {
        total = total + i;
        i = i + 1;
    }
    total
}

fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn negate(x: i32) -> i32 {
    -x
}

fn identity<T>(x: T) -> T {
    x
}

fn clamp(x: i32, lo: i32, hi: i32) -> i32 {
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

fn unwrap_or(opt: Option<i32>, default: i32) -> i32 {
    match opt {
        Some(v) => v,
        None => default,
    }
}

fn is_weekend(day: i32) -> bool {
    match day {
        6 | 7 => true,
        _ => false,
    }
}

fn classify(x: i32) -> i32 {
    match x {
        n if n > 0 => 1,
        n if n < 0 => -1,
        _ => 0,
    }
}

fn fibonacci(n: i32) -> i32 {
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
}

fn first_of_pair(pair: (i32, i32)) -> i32 {
    let (a, _) = pair;
    a
}

fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

// === Lifetime / ownership tests ===
// These Rust functions use &, &mut, Box, lifetimes etc.
// MoonBit doesn't need any of that (GC'd), so they should
// convert to clean MoonBit without any pointer/lifetime info.

fn len_ref(s: &str) -> usize {
    s.len()
}

fn inc_mut(x: &mut i32) -> i32 {
    *x + 1
}

fn unbox(b: Box<i32>) -> i32 {
    *b
}

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn find_ref(v: &Vec<i32>, key: i32) -> Option<&i32> {
    None
}

#[derive(Debug, PartialEq, Eq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

fn make_rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb { r, g, b }
}

fn rc_value(r: Rc<i32>) -> i32 {
    *r
}

fn arc_pair(a: Arc<i32>, b: Arc<i32>) -> i32 {
    *a + *b
}

// === Pattern coverage for cross-validation ===

// Closure as parameter
fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// if let → is pattern
fn extract_opt(opt: Option<i32>) -> i32 {
    if let Some(x) = opt {
        x
    } else {
        0
    }
}

// match with guard
fn sign(x: i32) -> i32 {
    match x {
        n if n > 0 => 1,
        n if n < 0 => -1,
        _ => 0,
    }
}

// for loop with range
fn sum_range(n: i32) -> i32 {
    let mut total = 0;
    for i in 0..n {
        total = total + i;
    }
    total
}

// for with tuple destructure
fn sum_pairs(pairs: Vec<(i32, i32)>) -> i32 {
    let mut total = 0;
    for (a, b) in pairs {
        total = total + a + b;
    }
    total
}

// method call mapping
fn str_len(s: &str) -> usize {
    s.len()
}

// chained method
fn trim_len(s: &str) -> usize {
    s.trim().len()
}

// record update
fn move_point(p: Point, dx: i32) -> Point {
    Point { x: p.x + dx, ..p }
}

// generic with bounds
fn max_val<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// not operator
fn is_not_empty(v: &Vec<i32>) -> bool {
    !v.is_empty()
}

// nested if
fn fizzbuzz(n: i32) -> i32 {
    if n % 15 == 0 {
        0
    } else {
        if n % 3 == 0 {
            1
        } else {
            if n % 5 == 0 {
                2
            } else {
                3
            }
        }
    }
}

// multiple let + complex body
fn quadratic(a: f64, b: f64, c: f64, x: f64) -> f64 {
    let x2 = x * x;
    let term1 = a * x2;
    let term2 = b * x;
    term1 + term2 + c
}

// bool ops
fn all_positive(a: i32, b: i32, c: i32) -> bool {
    a > 0 && b > 0 && c > 0
}

fn any_zero(a: i32, b: i32, c: i32) -> bool {
    a == 0 || b == 0 || c == 0
}

// === Basic syntax patterns ===

// return early
fn check_positive(x: i32) -> i32 {
    if x <= 0 {
        return 0;
    }
    x * 2
}

// typed let binding
fn typed_let() -> i32 {
    let x: i32 = 42;
    let y: i32 = x + 8;
    y
}

// nested match
fn nested_match(x: Option<Option<i32>>) -> i32 {
    match x {
        Some(inner) => match inner {
            Some(v) => v,
            None => -1,
        },
        None => -2,
    }
}

// string and char
fn greeting() -> String {
    "hello".to_string()
}

fn initial() -> char {
    'A'
}

// numeric literals
fn int64_val() -> i64 {
    123456789i64
}

fn float_val() -> f64 {
    3.14159
}

fn uint_val() -> u32 {
    42u32
}

// array operations
fn get_second(arr: Vec<i32>) -> i32 {
    arr[1]
}

// field access and struct creation
fn make_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

fn get_point_x(p: Point) -> i32 {
    p.x
}

// empty function
fn noop() {}

// multiple return values
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

// tuple access
fn first(pair: (i32, i32)) -> i32 {
    pair.0
}

fn second(pair: (i32, i32)) -> i32 {
    pair.1
}

// simple recursion
fn factorial(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// multiple if-else
fn grade(score: i32) -> i32 {
    if score >= 90 {
        5
    } else if score >= 80 {
        4
    } else if score >= 70 {
        3
    } else if score >= 60 {
        2
    } else {
        1
    }
}

// generic struct
struct Pair<A, B> {
    first: A,
    second: B,
}

fn make_pair(a: i32, b: String) -> Pair<i32, String> {
    Pair { first: a, second: b }
}

fn get_pair_first(p: Pair<i32, String>) -> i32 {
    p.first
}

// or pattern in match
fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

// deeply nested expression
fn complex_calc(a: i32, b: i32, c: i32) -> i32 {
    let sum = a + b + c;
    let avg = sum / 3;
    let diff = if a > b { a - b } else { b - a };
    avg + diff
}

// === Additional basic patterns ===

// const
const LIMIT: i32 = 256;

// bool literal
fn always_true() -> bool {
    true
}

fn always_false() -> bool {
    false
}

// comparison operators
fn compare(a: i32, b: i32) -> bool {
    a == b || a != b && a <= b
}

// array literal
fn three_nums() -> Vec<i32> {
    vec![10, 20, 30]
}


// Option construction
fn maybe_val(flag: bool) -> Option<i32> {
    if flag {
        Some(42)
    } else {
        None
    }
}

// Box unwrap (recursive tree)
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn leaf(v: i32) -> TreeNode {
    TreeNode {
        value: v,
        left: None,
        right: None,
    }
}

fn tree_sum(node: &TreeNode) -> i32 {
    let mut sum = node.value;
    if let Some(l) = &node.left {
        sum = sum + tree_sum(l);
    }
    if let Some(r) = &node.right {
        sum = sum + tree_sum(r);
    }
    sum
}

// closure stored in variable
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// while true + break
fn first_over_100(arr: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut result = -1;
    while i < arr.len() {
        if arr[i] > 100 {
            result = arr[i];
            break;
        }
        i = i + 1;
    }
    result
}

// multi-line match with complex bodies
fn describe_num(n: i32) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        n if n < 0 => "negative".to_string(),
        n if n > 100 => "big".to_string(),
        _ => "other".to_string(),
    }
}

// derive on struct
#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn origin_3d() -> Coord {
    Coord { x: 0, y: 0, z: 0 }
}

// multiple fields
fn offset_coord(c: Coord, dx: i32, dy: i32, dz: i32) -> Coord {
    Coord {
        x: c.x + dx,
        y: c.y + dy,
        z: c.z + dz,
    }
}

// logical operations
fn clamp_positive(x: i32) -> i32 {
    if x > 0 { x } else { 0 }
}

fn abs_val(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

// string operations
fn is_long(s: &str) -> bool {
    s.len() > 10
}

// chained operations
fn sum_of_squares(n: i32) -> i32 {
    let mut total = 0;
    let mut i = 1;
    while i <= n {
        total = total + i * i;
        i = i + 1;
    }
    total
}

// enum with multiple variants
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn is_horizontal(d: &Direction) -> bool {
    match d {
        Direction::East | Direction::West => true,
        _ => false,
    }
}

// === Result return ===

fn parse_int(s: &str) -> Result<i32, String> {
    if s.is_empty() {
        Err("empty string".to_string())
    } else {
        Ok(42)
    }
}

fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// === Nested struct ===

struct Address {
    city: String,
    zip: String,
}

struct Person {
    name: String,
    age: i32,
    address: Address,
}

fn get_city(p: &Person) -> String {
    p.address.city.clone()
}

fn is_adult(p: &Person) -> bool {
    p.age >= 18
}

// === Multi-arm match with block bodies ===

fn eval_op(op: &str, a: i32, b: i32) -> i32 {
    match op {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => {
            if b != 0 {
                a / b
            } else {
                0
            }
        }
        _ => 0,
    }
}

// === Default values ===

fn or_default(x: Option<i32>) -> i32 {
    match x {
        Some(v) => v,
        None => 0,
    }
}

// === Vec operations ===

fn contains_val(v: &Vec<i32>, target: i32) -> bool {
    for x in v {
        if *x == target {
            return true;
        }
    }
    false
}

// === Pattern matching ===

fn color_code(c: &str) -> i32 {
    match c {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        "yellow" => 4,
        _ => -1,
    }
}

// === Arithmetic ===

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn power(base: i32, exp: i32) -> i32 {
    let mut result = 1;
    let mut i = 0;
    while i < exp {
        result = result * base;
        i = i + 1;
    }
    result
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i = i + 1;
    }
    true
}

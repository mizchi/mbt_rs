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

#[test]
fn test_apply() {
    assert_eq!(apply(|x| x * 2, 5), 10);
}

#[test]
fn test_extract_opt() {
    assert_eq!(extract_opt(Some(42)), 42);
    assert_eq!(extract_opt(None), 0);
}

#[test]
fn test_sign() {
    assert_eq!(sign(5), 1);
    assert_eq!(sign(-3), -1);
    assert_eq!(sign(0), 0);
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(fizzbuzz(15), 0);
    assert_eq!(fizzbuzz(3), 1);
    assert_eq!(fizzbuzz(5), 2);
    assert_eq!(fizzbuzz(7), 3);
}

#[test]
fn test_quadratic() {
    // f(x) = 2x^2 + 3x + 1, f(2) = 8 + 6 + 1 = 15
    assert_eq!(quadratic(2.0, 3.0, 1.0, 2.0), 15.0);
}

#[test]
fn test_all_positive() {
    assert!(all_positive(1, 2, 3));
    assert!(!all_positive(1, -2, 3));
}


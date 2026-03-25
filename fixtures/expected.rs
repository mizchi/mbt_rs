#![allow(dead_code, unused_variables, unused_mut, unreachable_code, unused_imports)]
use std::collections::HashMap;

fn add(x: i32, y: i32) -> i32 {
    x + y
}
pub fn hello() -> String {
    "hello".to_string()
}

#[derive(Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
enum Shape {
    Circle(f64),
    Rect(f64, f64),
}
type Score = i32;
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
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
const MAX_SIZE: i32 = 100;
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}
fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
fn sum_to(n: i32) -> i32 {
    let mut total = 0;
    let mut i = 1;
    while i <= n {
        total = total + i;
        i = i + 1
    };
    total
}
fn nums() -> Vec<i32> {
    vec![1, 2, 3]
}
struct Handle;
struct Wrapper(i32);
fn apply_fn(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
fn move_right(p: Point) -> Point {
    Point { x: p.x + 1, ..p }
}
fn classify(x: i32) -> String {
    match x {
        n if n > 0 => "positive".to_string(),
        n if n < 0 => "negative".to_string(),
        _ => "zero".to_string(),
    }
}
fn first_of_pair(pair: (i32, i32)) -> i32 {
    let (a, _) = pair;
    a
}
fn is_weekend(day: i32) -> bool {
    match day {
        6 | 7 => true,
        _ => false,
    }
}
fn negate(x: i32) -> i32 {
    -x
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
fn typed() -> i32 {
    let x: i32 = 42;
    x
}
fn unwrap_or(opt: Option<i32>, default: i32) -> i32 {
    match opt {
        Some(v) => v,
        None => default,
    }
}
fn identity<T>(x: T) -> T {
    x
}
fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
fn deep(x: Option<i32>) -> i32 {
    match x {
        Some(v) => match v {
            0 => 0,
            n => n * 2,
        },
        None => -1,
    }
}
struct Pair<A, B> {
    pub first: A,
    pub second: B,
}
fn complex(x: i32) -> i32 {
    let a = x * 2;
    let b = a + 1;
    let c = if b > 10 {
        b
    } else {
        10
    };
    c
}
fn compare(a: i32, b: i32) -> bool {
    a == b || a != b && a <= b
}
#[derive(Debug, PartialEq, Eq)]
struct ColorPoint {
    pub x: i32,
    pub y: i32,
    pub color: String,
}
fn sum_range() -> i32 {
    let mut total = 0;
    let mut i = 0;
    while i < 10 {
        total = total + i;
        i = i + 1;
    };
    total
}
#[test]
fn test_addition() {
    assert_eq!(1 + 1, 2)
}

enum Expr {
    Lit(i32),
    BinOp {
        op: String,
        lhs: i32,
        rhs: i32,
    },
}
fn print_pairs(arr: Vec<(i32, String)>) {
    for (k, v) in arr {
        println!("{}", k.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(3, 5), 5);
        assert_eq!(max(7, 2), 7);
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap(1, 2), (2, 1));
    }

    #[test]
    fn test_compute() {
        assert_eq!(compute(), 3);
    }

    #[test]
    fn test_counter() {
        assert_eq!(counter(), 10);
    }

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(10), 55);
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10, 2), Some(5));
        assert_eq!(safe_div(10, 0), None);
    }

    #[test]
    fn test_struct() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn test_nums() {
        assert_eq!(nums(), vec![1, 2, 3]);
    }

    #[test]
    fn test_apply_fn() {
        assert_eq!(apply_fn(|x| x * 2, 5), 10);
    }

    #[test]
    fn test_move_right() {
        let p = Point { x: 1, y: 2 };
        let q = move_right(p);
        assert_eq!(q.x, 2);
        assert_eq!(q.y, 2);
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify(5), "positive");
        assert_eq!(classify(-3), "negative");
        assert_eq!(classify(0), "zero");
    }

    #[test]
    fn test_first_of_pair() {
        assert_eq!(first_of_pair((10, 20)), 10);
    }

    #[test]
    fn test_is_weekend() {
        assert!(is_weekend(6));
        assert!(is_weekend(7));
        assert!(!is_weekend(3));
    }

    #[test]
    fn test_negate() {
        assert_eq!(negate(5), -5);
        assert_eq!(negate(-3), 3);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-1, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_typed() {
        assert_eq!(typed(), 42);
    }

    #[test]
    fn test_unwrap_or() {
        assert_eq!(unwrap_or(Some(5), 0), 5);
        assert_eq!(unwrap_or(None, 0), 0);
    }

    #[test]
    fn test_identity() {
        assert_eq!(identity(42), 42);
        assert_eq!(identity("hello"), "hello");
    }

    #[test]
    fn test_divmod() {
        assert_eq!(divmod(10, 3), (3, 1));
    }

    #[test]
    fn test_deep() {
        assert_eq!(deep(Some(5)), 10);
        assert_eq!(deep(Some(0)), 0);
        assert_eq!(deep(None), -1);
    }

    #[test]
    fn test_complex() {
        assert_eq!(complex(3), 10);  // a=6, b=7, c=10 (7 < 10)
        assert_eq!(complex(6), 13);  // a=12, b=13, c=13 (13 > 10)
    }

    #[test]
    fn test_compare() {
        assert!(compare(1, 1));   // 1==1 is true
        assert!(compare(1, 2));   // 1!=2 && 1<=2 is true
    }

    #[test]
    fn test_enum_struct_variant() {
        let e = Expr::BinOp { op: "+".to_string(), lhs: 1, rhs: 2 };
        match e {
            Expr::BinOp { op, lhs, rhs } => {
                assert_eq!(op, "+");
                assert_eq!(lhs, 1);
                assert_eq!(rhs, 2);
            }
            _ => panic!("expected BinOp"),
        }
    }

    #[test]
    fn test_derive_struct() {
        let p = ColorPoint { x: 1, y: 2, color: "red".to_string() };
        let q = ColorPoint { x: 1, y: 2, color: "red".to_string() };
        assert_eq!(p, q);
        assert_eq!(format!("{:?}", p), "ColorPoint { x: 1, y: 2, color: \"red\" }");
    }

    #[test]
    fn test_sum_range() {
        assert_eq!(sum_range(), 45);  // 0+1+2+...+9
    }
}

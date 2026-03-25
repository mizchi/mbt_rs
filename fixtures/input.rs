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

/// Quality measurement test file.
/// Contains a mix of convertible and non-convertible Rust patterns.
/// Used to measure what percentage of real code can be auto-converted.

use std::collections::HashMap;
use std::rc::Rc;

// === Fully convertible ===

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    let mut i = 2;
    while i <= n {
        let t = a + b;
        a = b;
        b = t;
        i = i + 1;
    }
    b
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn distance_squared(p: &Point, q: &Point) -> i32 {
    let dx = p.x - q.x;
    let dy = p.y - q.y;
    dx * dx + dy * dy
}

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { radius } => 3.14159 * radius * radius,
        Shape::Rectangle { width, height } => width * height,
    }
}

fn safe_div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// === Partially convertible (needs manual fixes) ===

// impl methods - self type annotation needed
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn translate(&self, dx: i32, dy: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn origin() -> Self {
        Self::new(0, 0)
    }
}

// trait + impl
trait Describable {
    fn describe(&self) -> String;
}

impl Describable for Point {
    fn describe(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

// Generic with bounds
fn max_of_three<T: PartialOrd>(a: T, b: T, c: T) -> T {
    if a >= b && a >= c {
        a
    } else if b >= c {
        b
    } else {
        c
    }
}

// Closure stored
fn apply_all(values: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    let mut result = Vec::new();
    for v in values {
        result.push(f(*v));
    }
    result
}

// Box<dyn Trait>
fn make_greeter(name: String) -> Box<dyn Fn() -> String> {
    Box::new(move || format!("Hello, {}!", name))
}

// === Difficult / unlikely to convert correctly ===

// Lifetime annotations
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// Complex iterator chain
fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    let mut pairs: Vec<_> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs.truncate(n);
    pairs
}

// Async
async fn fetch_value() -> i32 {
    42
}

async fn compute_async() -> i32 {
    let a = fetch_value().await;
    let b = fetch_value().await;
    a + b
}

// Macro rules (not convertible)
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    };
}

// Unsafe (stripped with comment)
fn raw_add(a: *const i32, b: *const i32) -> i32 {
    unsafe { *a + *b }
}

// Complex pattern matching with bindings
fn categorize(value: i32) -> &'static str {
    match value {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=100 => "small positive",
        101..=1000 => "medium",
        _ => "large",
    }
}

// Trait objects and dynamic dispatch
fn print_all(items: &[&dyn Describable]) {
    for item in items {
        println!("{}", item.describe());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_point() {
        let p = Point::new(3, 4);
        assert_eq!(p.x, 3);
        assert_eq!(distance_squared(&p, &Point::origin()), 25);
    }

    #[test]
    fn test_area() {
        let c = Shape::Circle { radius: 1.0 };
        assert!((area(&c) - 3.14159).abs() < 0.001);
    }

    #[test]
    fn test_max() {
        assert_eq!(max_of_three(1, 3, 2), 3);
    }

    #[test]
    fn test_apply() {
        assert_eq!(apply_all(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
    }
}

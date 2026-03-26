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

// === Standard library method mappings ===

fn make_empty_vec() -> Vec<i32> {
    Vec::new()
}

fn make_empty_string() -> String {
    String::new()
}

fn option_unwrap_or(x: Option<i32>) -> i32 {
    x.unwrap_or(0)
}

fn option_is_some(x: Option<i32>) -> bool {
    x.is_some()
}

fn str_to_lower(s: &str) -> String {
    s.to_lowercase()
}

fn str_to_upper(s: &str) -> String {
    s.to_uppercase()
}

fn str_starts_with(s: &str, prefix: &str) -> bool {
    s.starts_with(prefix)
}

fn str_trim(s: &str) -> String {
    s.trim().to_string()
}

// Bytes operations
fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

fn bytes_length(data: &[u8]) -> usize {
    data.len()
}

fn count_byte(data: &[u8], target: u8) -> i32 {
    let mut count = 0;
    for b in data {
        if *b == target {
            count = count + 1;
        }
    }
    count
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

// === Higher-order functions ===

fn apply_fn(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn compose(f: fn(i32) -> i32, g: fn(i32) -> i32, x: i32) -> i32 {
    f(g(x))
}

fn map_array(arr: Vec<i32>, f: fn(i32) -> i32) -> Vec<i32> {
    let mut result = Vec::new();
    for x in arr {
        result.push(f(x));
    }
    result
}

fn filter_array(arr: Vec<i32>, pred: fn(i32) -> bool) -> Vec<i32> {
    let mut result = Vec::new();
    for x in arr {
        if pred(x) {
            result.push(x);
        }
    }
    result
}

fn fold_array(arr: Vec<i32>, init: i32, f: fn(i32, i32) -> i32) -> i32 {
    let mut acc = init;
    for x in arr {
        acc = f(acc, x);
    }
    acc
}

// === String operations ===

fn count_char(s: &str, target: char) -> i32 {
    let mut count = 0;
    for c in s.chars() {
        if c == target {
            count = count + 1;
        }
    }
    count
}

// === Numeric operations ===

fn clamp_f64(x: f64, lo: f64, hi: f64) -> f64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn sum_array(arr: Vec<i32>) -> i32 {
    let mut total = 0;
    for x in arr {
        total = total + x;
    }
    total
}

fn product_array(arr: Vec<i32>) -> i32 {
    let mut total = 1;
    for x in arr {
        total = total * x;
    }
    total
}

fn average(arr: Vec<i32>) -> f64 {
    if arr.is_empty() {
        return 0.0;
    }
    let mut total = 0;
    for x in &arr {
        total = total + x;
    }
    total as f64 / arr.len() as f64
}

// === Chained if-else ===

fn http_status(code: i32) -> String {
    if code == 200 {
        "OK".to_string()
    } else if code == 404 {
        "Not Found".to_string()
    } else if code == 500 {
        "Internal Server Error".to_string()
    } else if code == 301 {
        "Moved Permanently".to_string()
    } else {
        "Unknown".to_string()
    }
}

// === Early return patterns ===

fn find_index(arr: Vec<i32>, target: i32) -> i32 {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == target {
            return i as i32;
        }
        i = i + 1;
    }
    -1
}

fn all_positive_arr(arr: Vec<i32>) -> bool {
    for x in arr {
        if x <= 0 {
            return false;
        }
    }
    true
}

// === Nested struct access ===

fn person_city_length(p: &Person) -> i32 {
    p.address.city.len() as i32
}

// === Option chaining ===

fn double_if_positive(x: i32) -> Option<i32> {
    if x > 0 {
        Some(x * 2)
    } else {
        None
    }
}

fn chain_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match a {
        Some(va) => match b {
            Some(vb) => Some(va + vb),
            None => None,
        },
        None => None,
    }
}

// === Bit operations ===

fn is_power_of_two(n: i32) -> bool {
    n > 0 && (n & (n - 1)) == 0
}

// === Result chaining ===

fn checked_add(a: i32, b: i32, max: i32) -> Result<i32, String> {
    let sum = a + b;
    if sum > max {
        Err("overflow".to_string())
    } else {
        Ok(sum)
    }
}

fn chain_checked(a: i32, b: i32, c: i32, max: i32) -> Result<i32, String> {
    match checked_add(a, b, max) {
        Ok(ab) => checked_add(ab, c, max),
        Err(e) => Err(e),
    }
}

// === Nested Option ===

fn get_nested(arr: Vec<Vec<i32>>, i: usize, j: usize) -> Option<i32> {
    if i < arr.len() {
        let inner = &arr[i];
        if j < inner.len() {
            Some(inner[j])
        } else {
            None
        }
    } else {
        None
    }
}

// === Array of structs ===

fn sum_points_x(points: Vec<Point>) -> i32 {
    let mut total = 0;
    for p in points {
        total = total + p.x;
    }
    total
}

fn find_point_by_x(points: Vec<Point>, target_x: i32) -> Option<Point> {
    for p in points {
        if p.x == target_x {
            return Some(p);
        }
    }
    None
}

// === Recursive ===

// === Accumulator ===

fn running_sum(arr: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut sum = 0;
    for x in arr {
        sum = sum + x;
        result.push(sum);
    }
    result
}

fn max_in_array(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut best = arr[0];
    let mut i = 1;
    while i < arr.len() {
        if arr[i] > best {
            best = arr[i];
        }
        i = i + 1;
    }
    Some(best)
}

fn min_in_array(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut best = arr[0];
    let mut i = 1;
    while i < arr.len() {
        if arr[i] < best {
            best = arr[i];
        }
        i = i + 1;
    }
    Some(best)
}

// === State machine ===

fn count_words(s: &str) -> i32 {
    let mut count = 0;
    let mut in_word = false;
    for c in s.chars() {
        if c == ' ' {
            in_word = false;
        } else {
            if !in_word {
                count = count + 1;
                in_word = true;
            }
        }
    }
    count
}

// === Range / boundary ===

fn in_range(x: i32, lo: i32, hi: i32) -> bool {
    x >= lo && x <= hi
}

fn clamp_to_byte(x: i32) -> i32 {
    if x < 0 { 0 } else if x > 255 { 255 } else { x }
}

fn wrap_around(x: i32, max: i32) -> i32 {
    ((x % max) + max) % max
}

// === Multi-return patterns ===

fn split_at(arr: Vec<i32>, idx: usize) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut i = 0;
    while i < arr.len() {
        if i < idx {
            left.push(arr[i]);
        } else {
            right.push(arr[i]);
        }
        i = i + 1;
    }
    (left, right)
}

// === Zip / combine ===

fn dot_product(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    let len = if a.len() < b.len() { a.len() } else { b.len() };
    while i < len {
        sum = sum + a[i] * b[i];
        i = i + 1;
    }
    sum
}

fn pairwise_max(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let len = if a.len() < b.len() { a.len() } else { b.len() };
    while i < len {
        if a[i] > b[i] {
            result.push(a[i]);
        } else {
            result.push(b[i]);
        }
        i = i + 1;
    }
    result
}

// === Matrix-like ===

fn transpose_2x2(a: i32, b: i32, c: i32, d: i32) -> (i32, i32, i32, i32) {
    (a, c, b, d)
}

// === Multiple conditions ===

fn classify_triangle(a: i32, b: i32, c: i32) -> String {
    if a + b <= c || a + c <= b || b + c <= a {
        "invalid".to_string()
    } else if a == b && b == c {
        "equilateral".to_string()
    } else if a == b || b == c || a == c {
        "isosceles".to_string()
    } else {
        "scalene".to_string()
    }
}

fn fizzbuzz_string(n: i32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

// === Array manipulation ===

fn remove_duplicates(arr: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for x in arr {
        let mut found = false;
        for y in &result {
            if *y == x {
                found = true;
                break;
            }
        }
        if !found {
            result.push(x);
        }
    }
    result
}

fn flatten_nested(arr: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    for inner in arr {
        for x in inner {
            result.push(x);
        }
    }
    result
}

fn take_n(arr: Vec<i32>, n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < n && i < arr.len() {
        result.push(arr[i]);
        i = i + 1;
    }
    result
}

fn drop_n(arr: Vec<i32>, n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = n;
    while i < arr.len() {
        result.push(arr[i]);
        i = i + 1;
    }
    result
}

// === String analysis ===

fn is_digit_string(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    for c in s.chars() {
        if c < '0' || c > '9' {
            return false;
        }
    }
    true
}

fn char_at(s: &str, idx: usize) -> Option<char> {
    let mut i = 0;
    for c in s.chars() {
        if i == idx {
            return Some(c);
        }
        i = i + 1;
    }
    None
}

// === Math ===

fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else {
        let g = gcd(a, b);
        (a / g) * b
    }
}

fn collatz_steps(n: i32) -> i32 {
    let mut n = n;
    let mut steps = 0;
    while n != 1 && n > 0 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        steps = steps + 1;
    }
    steps
}

fn digit_sum(n: i32) -> i32 {
    let mut n = if n < 0 { -n } else { n };
    let mut sum = 0;
    while n > 0 {
        sum = sum + n % 10;
        n = n / 10;
    }
    sum
}

// === Sorting / ordering ===

fn is_sorted(arr: &Vec<i32>) -> bool {
    let mut i = 1;
    while i < arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
        i = i + 1;
    }
    true
}

// === Lookup table ===

fn roman_digit(n: i32) -> String {
    match n {
        1 => "I".to_string(),
        2 => "II".to_string(),
        3 => "III".to_string(),
        4 => "IV".to_string(),
        5 => "V".to_string(),
        6 => "VI".to_string(),
        7 => "VII".to_string(),
        8 => "VIII".to_string(),
        9 => "IX".to_string(),
        _ => "?".to_string(),
    }
}

// === Two pointer ===

fn has_pair_sum(sorted: &Vec<i32>, target: i32) -> bool {
    if sorted.len() < 2 {
        return false;
    }
    let mut lo = 0;
    let mut hi = sorted.len() - 1;
    while lo < hi {
        let sum = sorted[lo] + sorted[hi];
        if sum == target {
            return true;
        } else if sum < target {
            lo = lo + 1;
        } else {
            hi = hi - 1;
        }
    }
    false
}

// === Counting ===

fn count_occurrences(arr: &Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    for x in arr {
        if *x == target {
            count = count + 1;
        }
    }
    count
}

fn most_frequent(arr: Vec<i32>) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut best = arr[0];
    let mut best_count = 0;
    for x in &arr {
        let c = count_occurrences(&arr, *x);
        if c > best_count {
            best_count = c;
            best = *x;
        }
    }
    Some(best)
}

// === Conversion between types ===

fn bool_to_int(b: bool) -> i32 {
    if b { 1 } else { 0 }
}

fn int_to_bool(n: i32) -> bool {
    n != 0
}

fn sign_char(n: i32) -> char {
    if n > 0 { '+' } else if n < 0 { '-' } else { '0' }
}

// === Option utilities ===

fn first_some(a: Option<i32>, b: Option<i32>, c: Option<i32>) -> Option<i32> {
    match a {
        Some(_) => a,
        None => match b {
            Some(_) => b,
            None => c,
        },
    }
}

fn zip_options(a: Option<i32>, b: Option<i32>) -> Option<(i32, i32)> {
    match a {
        Some(va) => match b {
            Some(vb) => Some((va, vb)),
            None => None,
        },
        None => None,
    }
}

// === Array generation ===

fn range_array(start: i32, end: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = start;
    while i < end {
        result.push(i);
        i = i + 1;
    }
    result
}

fn repeat_val(val: i32, n: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        result.push(val);
        i = i + 1;
    }
    result
}

// === Binary search ===

fn binary_search(arr: &Vec<i32>, target: i32) -> Option<i32> {
    let mut lo: i32 = 0;
    let mut hi: i32 = arr.len() as i32 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid as usize] == target {
            return Some(mid);
        } else if arr[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}

// === Previously broken, now fixed ===

fn sum_ref(arr: &Vec<i32>) -> i32 {
    let mut s = 0;
    for x in arr {
        s = s + *x;
    }
    s
}

fn option_unwrap_or_val(x: Option<i32>, default: i32) -> i32 {
    x.unwrap_or(default)
}

fn int_to_string(n: i32) -> String {
    n.to_string()
}

fn deref_in_loop(arr: &Vec<i32>, target: i32) -> bool {
    for x in arr {
        if *x == target {
            return true;
        }
    }
    false
}

// === Option comprehensive ===

fn option_map(x: Option<i32>) -> Option<i32> {
    x.map(|v| v * 2)
}

fn option_and_then(x: Option<i32>) -> Option<i32> {
    x.and_then(|v| if v > 0 { Some(v) } else { None })
}

fn option_flatten(x: Option<Option<i32>>) -> Option<i32> {
    x.flatten()
}

fn option_unwrap_or_else(x: Option<i32>) -> i32 {
    x.unwrap_or_else(|| 0)
}

fn option_is_some_and(x: Option<i32>) -> bool {
    x.is_some() && x.unwrap() > 0
}

fn option_or(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match a {
        Some(_) => a,
        None => b,
    }
}

fn option_to_result(x: Option<i32>) -> Result<i32, String> {
    match x {
        Some(v) => Ok(v),
        None => Err("none".to_string()),
    }
}

// === Result comprehensive ===

fn result_map(x: Result<i32, String>) -> Result<i32, String> {
    x.map(|v| v * 2)
}

fn result_map_err(x: Result<i32, String>) -> Result<i32, String> {
    x.map_err(|e| format!("error: {}", e))
}

fn result_and_then(x: Result<i32, String>) -> Result<i32, String> {
    x.and_then(|v| if v > 0 { Ok(v) } else { Err("negative".to_string()) })
}

fn result_unwrap_or(x: Result<i32, String>) -> i32 {
    x.unwrap_or(0)
}

fn result_is_ok_and(x: Result<i32, String>) -> bool {
    x.is_ok()
}

fn result_is_err_and(x: Result<i32, String>) -> bool {
    x.is_err()
}

fn result_to_option(x: Result<i32, String>) -> Option<i32> {
    match x {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn count_ones(n: i32) -> i32 {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        count = count + (n & 1);
        n = n >> 1;
    }
    count
}

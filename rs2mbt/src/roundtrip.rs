/// Round-trip tests: Rust → MoonBit → (verify structure preserved)
/// These test that converting Rust to MoonBit and back preserves key structures.
/// Full round-trip (Rust → MoonBit → Rust) requires mbt2rs which runs in MoonBit,
/// so we verify the intermediate MoonBit output contains expected patterns.
#[cfg(test)]
mod tests {
    use crate::to_moonbit;

    /// Verify that the MoonBit output from a Rust function contains the expected
    /// structural elements (function name, param names, return type, body patterns).
    fn assert_roundtrip_structure(rust_src: &str, checks: &[&str]) {
        let mbt = to_moonbit(rust_src);
        for check in checks {
            assert!(
                mbt.contains(check),
                "MoonBit output missing expected pattern: '{}'\n--- output ---\n{}",
                check,
                mbt
            );
        }
    }

    #[test]
    fn roundtrip_simple_fn() {
        assert_roundtrip_structure(
            "fn add(x: i32, y: i32) -> i32 { x + y }",
            &["fn add(", "x : Int", "y : Int", "-> Int", "x + y"],
        );
    }

    #[test]
    fn roundtrip_struct() {
        assert_roundtrip_structure(
            "pub struct Point { x: i32, y: i32 }",
            &["pub struct Point", "x : Int", "y : Int"],
        );
    }

    #[test]
    fn roundtrip_enum() {
        assert_roundtrip_structure(
            "enum Shape { Circle(f64), Rect(f64, f64) }",
            &["enum Shape", "Circle(Double)", "Rect(Double, Double)"],
        );
    }

    #[test]
    fn roundtrip_generic_fn() {
        assert_roundtrip_structure(
            "fn identity<T>(x: T) -> T { x }",
            &["fn[T] identity(", "x : T", "-> T"],
        );
    }

    #[test]
    fn roundtrip_option_match() {
        assert_roundtrip_structure(
            "fn unwrap_or(opt: Option<i32>, default: i32) -> i32 { match opt { Some(v) => v, None => default } }",
            &["fn unwrap_or(", "Option[Int]", "match opt", "Some(v) => v", "None => default"],
        );
    }

    #[test]
    fn roundtrip_lifetime_stripped() {
        assert_roundtrip_structure(
            "fn first<'a>(s: &'a str) -> &'a str { s }",
            &["fn first(", "s : String", "-> String"],
        );
        // Verify no lifetime traces remain
        let mbt = to_moonbit("fn first<'a>(s: &'a str) -> &'a str { s }");
        assert!(!mbt.contains("'a"), "Lifetime should be stripped: {}", mbt);
        assert!(!mbt.contains("&"), "Reference should be stripped: {}", mbt);
    }

    #[test]
    fn roundtrip_box_stripped() {
        assert_roundtrip_structure(
            "fn unbox(b: Box<i32>) -> i32 { *b }",
            &["fn unbox(", "b : Int", "-> Int"],
        );
        let mbt = to_moonbit("fn unbox(b: Box<i32>) -> i32 { *b }");
        assert!(!mbt.contains("Box"), "Box should be stripped: {}", mbt);
    }

    #[test]
    fn roundtrip_if_let_to_is() {
        assert_roundtrip_structure(
            "fn extract(opt: Option<i32>) -> i32 { if let Some(x) = opt { x } else { 0 } }",
            &["if opt is Some(x)"],
        );
    }

    #[test]
    fn roundtrip_self_resolved() {
        assert_roundtrip_structure(
            "impl Stack { fn new() -> Self { Stack { elements: Vec::new() } } }",
            &["fn Stack::new()", "-> Stack"],
        );
        let mbt = to_moonbit("impl Stack { fn new() -> Self { Stack { elements: Vec::new() } } }");
        assert!(!mbt.contains("Self"), "Self should be resolved: {}", mbt);
    }

    #[test]
    fn roundtrip_method_mapping() {
        assert_roundtrip_structure(
            "fn process(v: Vec<i32>) -> usize { v.len() }",
            &["fn process(", "Array[Int]", "-> Int", "v.length()"],
        );
    }

    #[test]
    fn roundtrip_derive() {
        assert_roundtrip_structure(
            "#[derive(Debug, PartialEq, Eq)]\nstruct Point { x: i32, y: i32 }",
            &["derive(Show, Eq)", "struct Point"],
        );
    }

    #[test]
    fn roundtrip_trait() {
        assert_roundtrip_structure(
            "trait Printable { fn to_string(&self) -> String; }",
            &["trait Printable", "to_string(self)", "-> String"],
        );
    }

    #[test]
    fn roundtrip_impl_trait() {
        assert_roundtrip_structure(
            "impl Show for Point { fn fmt(&self, f: &mut Formatter) -> Result<(), Error> { Ok(()) } }",
            &["impl Show for Point with fmt(self"],
        );
    }

    #[test]
    fn roundtrip_where_clause_fn() {
        assert_roundtrip_structure(
            "impl<K: Eq, V> Map<K, V> { pub fn retain<F>(&mut self, keep_fn: F) where F: FnMut(&K, &mut V) -> bool { } }",
            &["fn Map::retain(self : Map, keep_fn : (K, V) -> Bool)"],
        );
    }

    #[test]
    fn roundtrip_complex_fn() {
        assert_roundtrip_structure(
            r#"fn fibonacci(n: i32) -> i32 {
                if n <= 1 { return n; }
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
            &[
                "fn fibonacci(",
                "-> Int",
                "if n <= 1",
                "return n",
                "let mut a = 0",
                "let mut b = 1",
                "while i <= n",
                "let temp = a + b",
            ],
        );
    }

    #[test]
    fn roundtrip_test_block() {
        assert_roundtrip_structure(
            "#[test]\nfn test_add() { assert_eq!(1 + 1, 2); }",
            &["test \"add\"", "assert_eq(1 + 1, 2)"],
        );
    }

    #[test]
    fn roundtrip_borrow_stripped() {
        let mbt = to_moonbit("fn get(v: &Vec<i32>, key: &i32) -> Option<&i32> { None }");
        assert!(!mbt.contains(".borrow()"), "borrow should be stripped: {}", mbt);
        assert!(!mbt.contains("&"), "references should be stripped: {}", mbt);
        assert!(mbt.contains("Array[Int]"), "Vec<i32> should map to Array[Int]: {}", mbt);
    }
}

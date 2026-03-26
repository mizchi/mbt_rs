# mbt_rs

Bidirectional transpiler between MoonBit and Rust.

Focuses on syntax-level conversion with standard library API mapping, ownership/lifetime stripping, Drop→defer conversion, and async syntax support. Does not aim for perfect conversion — converts as much as possible and leaves the rest as WARNING comments for manual fixing.

## Install

```bash
# rs2mbt (Rust → MoonBit) CLI
cargo install --git https://github.com/mizchi/mbt_rs rs2mbt

# mbt2rs (MoonBit → Rust) library
moon add mizchi/mbt_rs
```

## Usage

### Rust → MoonBit (CLI)

```bash
# Single file
rs2mbt input.rs > output.mbt

# With macro expansion (recommended for real projects)
rs2mbt --expand src/lib.rs > output.mbt

# Quality report
rs2mbt --report input.rs

# Convert + auto format
just convert input.rs
```

### MoonBit → Rust (Library)

```
// moon.pkg
import {
  "mizchi/mbt_rs/mbt2rs",
  "moonbitlang/parser",
}
```

```moonbit
let source = "fn add(x : Int, y : Int) -> Int { x + y }"
let (impls, _) = @moonbitlang/parser.parse_string(source)
let rust_code = @mbt2rs.to_rust(impls)
// → "fn add(x: i32, y: i32) -> i32 {\n    x + y\n}\n"
```

### Quality Report

```bash
rs2mbt --report input.rs
rs2mbt --expand --report src/lib.rs  # with macro expansion
```

Results on real projects:

| File | Lines | Conversion | Notes |
|------|-------|-----------|-------|
| calculator.rs | 211 | 100% | Expression tree + eval + simplify |
| stack.rs | 114 | 100% | Generic stack |
| quality_test.rs | 181 | 97% | Mixed patterns |
| linear_map.rs | 606 | 96% | Vec-based map |
| hex crate | 525 | 96% | Hex encode/decode |
| After `cargo expand` | — | 99% | Macro expansion eliminates most WARNINGs |

## Conversion Reference

### Types

| Rust | MoonBit |
|------|---------|
| `i32` | `Int` |
| `u32` | `UInt` |
| `i64` | `Int64` |
| `u64` | `UInt64` |
| `f32` | `Float` |
| `f64` | `Double` |
| `bool` | `Bool` |
| `char` | `Char` |
| `u8` | `Byte` |
| `String` / `&str` | `String` |
| `Vec<T>` | `Array[T]` |
| `HashMap<K,V>` | `Map[K,V]` |
| `Option<T>` | `Option[T]` |
| `Result<T,E>` | `Result[T,E]` |
| `&[u8]` | `Bytes` |
| `Box<T>` / `Rc<T>` / `Arc<T>` | `T` (GC'd, wrapper stripped) |

### Syntax

| Rust | MoonBit |
|------|---------|
| `fn foo<T>(x: T) -> T` | `fn[T] foo(x: T) -> T` |
| `impl<T> S<T> { fn m(&self) }` | `fn S::m(self: S[T])` |
| `trait Foo { fn bar(&self) -> T; }` | `trait Foo { bar(Self) -> T }` |
| `impl Foo for Bar { fn bar(&self) { } }` | `impl Foo for Bar with bar(self: Bar) { }` |
| `fn f(x: &dyn Trait)` | `fn[X: Trait] f(x: X)` |
| `#[derive(Debug, Eq)]` | `} derive(Show, Eq)` |
| `if let Some(x) = opt` | `if opt is Some(x)` |
| `expr.await` | `expr` (no await needed) |
| `vec![1,2,3]` | `[1,2,3]` |
| `format!("{} {}", a, b)` | `"\{a} \{b}"` |
| `assert_eq!(a, b)` | `assert_eq(a, b)` |
| `println!("hi")` | `println("hi")` |
| `matches!(x, Pat)` | `x is Pat` |
| `String::from("s")` | `"s"` |
| `Vec::new()` | `[]` |
| `Box::new(x)` | `x` |
| `x.clone()` | `x` (GC'd) |
| `x.len()` | `x.length()` |
| `x.iter().map(f).collect()` | `x.map(f)` |
| `x.and_then(f)` | `x.bind(f)` |
| `x as f64` | `x.to_double()` |
| `!expr` | `not(expr)` |
| `#[test] fn test_foo()` | `test "foo" { }` |
| `struct Unit;` | `struct Unit {}` |
| `let e = Unit;` | `let e = Unit::{ }` |
| `loop { }` | `while true { }` |
| `while let Some(x) = v.next()` | `for x in v { }` |
| `for (a,b) in v` | `for _item in v { let (a,b) = _item }` |

### Ownership / Lifetimes

All stripped. MoonBit is garbage-collected.

| Rust | MoonBit |
|------|---------|
| `&T` / `&mut T` | `T` |
| `*const T` / `*mut T` | `T` |
| `Box<T>` | `T` |
| `Rc<T>` / `Arc<T>` | `T` (with NOTE comment) |
| `Cell<T>` / `RefCell<T>` | `T` |
| `'a` lifetime params | removed |
| `*expr` (deref) | `expr` |
| `impl Drop for X { fn drop() { body } }` | `defer { body }` at creation site |

### Iterator / Collection Trait Impls

| Rust | MoonBit |
|------|---------|
| `impl Iterator for X` | skipped (use `.iter()` / `for-in`) |
| `impl IntoIterator` | skipped |
| `impl Clone` / `Copy` | removed |
| `impl Index` | skipped (`op_get` / `op_set`) |
| `impl Extend` | skipped |

### Option / Result

| Rust | MoonBit |
|------|---------|
| `.map(f)` | `.map(f)` |
| `.and_then(f)` | `.bind(f)` |
| `.unwrap_or(v)` | `.unwrap_or(v)` |
| `.unwrap_or_else(f)` | `.or_else(f)` |
| `.flatten()` | `.flatten()` |
| `.is_some()` / `.is_none()` | same |
| `.is_ok()` / `.is_err()` | same |
| `.map_err(f)` | `.map_err(f)` |

### Unsupported (emits WARNING comments)

- `macro_rules!` definitions — use `cargo expand` or `rs2mbt --expand`
- `unsafe` blocks — stripped with comment
- Blanket trait impls (`impl<T> Foo for T`)
- Complex trait objects / dynamic dispatch

## Testing

```bash
# All tests
just test-all

# Individual suites
just test            # mbt2rs MoonBit tests (100)
just cargo-test      # rs2mbt Rust tests (139)
just behavioral-test # Behavioral equivalence (Rust 159 + MoonBit 161)
```

The behavioral test runs the same assertions in both Rust (`cargo test`) and MoonBit (`moon test`) to verify that converted code produces identical results.

## CI

GitHub Actions with 3 jobs:
- **MoonBit**: lint + type check + mbt2rs tests
- **Rust**: rs2mbt unit tests + roundtrip tests
- **Behavioral equivalence**: Rust cargo test + converted MoonBit moon test

## License

MIT

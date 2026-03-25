# mbt_rs

Bidirectional transpiler between MoonBit and Rust.

Focuses on syntax-level conversion with standard library API mapping, ownership/lifetime stripping, and async syntax conversion. Does not aim for perfect conversion — converts as much as possible and leaves the rest as TODO comments for manual fixing.

## Install

```bash
# rs2mbt (Rust → MoonBit) CLI
cd rs2mbt && cargo build --release
# Binary: rs2mbt/target/release/rs2mbt
```

mbt2rs (MoonBit → Rust) is available as a MoonBit library.

## Usage

### Rust → MoonBit

```bash
# Single file conversion
rs2mbt input.rs > output.mbt

# Convert with macro expansion (recommended for real projects)
just expand-and-convert path/to/rust/crate > output.mbt

# Quality report
rs2mbt --report input.rs
```

### MoonBit → Rust

```moonbit
let rust_code = @to_rust.to_rust(impls)
```

### Quality Report

```bash
just quality-report path/to/file.rs
just quality-report-all
```

Results on real projects:

| File | Lines | Conversion | Notes |
|------|-------|-----------|-------|
| calculator.rs | 211 | 100% | Expression tree + eval + simplify |
| stack.rs | 114 | 100% | Generic stack |
| quality_test.rs | 181 | 97% | Mixed patterns |
| linear_map.rs | 606 | 93% | Vec-based map |
| After `cargo expand` | — | 99% | Macro expansion eliminates most TODOs |

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
| `#[derive(Debug, Eq)]` | `} derive(Show, Eq)` |
| `if let Some(x) = opt` | `if opt is Some(x)` |
| `expr.await` | `expr` (no await needed) |
| `vec![1,2,3]` | `[1,2,3]` |
| `format!("{} {}", a, b)` | `"\{a} \{b}"` |
| `assert_eq!(a, b)` | `assert_eq(a, b)` |
| `println!("hi")` | `println("hi")` |
| `String::from("s")` | `"s"` |
| `Vec::new()` | `[]` |
| `Box::new(x)` | `x` |
| `x.clone()` | `x` (GC'd) |
| `x.borrow()` | `x` (GC'd) |
| `x.len()` | `x.length()` |
| `x.to_lowercase()` | `x.to_lower()` |
| `#[test] fn test_foo()` | `test "foo" { }` |
| `matches!(x, Pat)` | `x is Pat` |
| `!expr` | `not(expr)` |
| `loop { }` | `while true { }` |
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

### Iterator Trait Impls

| Rust | MoonBit |
|------|---------|
| `impl Iterator for X` | skipped (use `.iter()` / `for-in`) |
| `impl IntoIterator` | skipped |
| `impl Clone` / `Copy` | removed |
| `impl Index` | skipped (`op_get` / `op_set`) |
| `impl Extend` | skipped |

### Unsupported (emits TODO comments)

- `macro_rules!` definitions — use `cargo expand` to resolve
- `unsafe` blocks — stripped with comment
- `tokio::select!` — no MoonBit equivalent
- Complex trait objects / dynamic dispatch

## Testing

```bash
# Run all tests
just test-all

# Individual suites
just test            # mbt2rs MoonBit tests (100)
just cargo-test      # rs2mbt Rust tests (139)
just behavioral-test # Behavioral equivalence (Rust 72 + MoonBit 74)
```

The behavioral test runs the same assertions in both Rust (`cargo test`) and MoonBit (`moon test`) to verify that converted code produces identical results.

## CI

GitHub Actions with 3 jobs:
- **MoonBit**: lint + type check + mbt2rs tests
- **Rust**: rs2mbt unit tests + roundtrip tests
- **Behavioral equivalence**: Rust cargo test + converted MoonBit moon test

## License

MIT

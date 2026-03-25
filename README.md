# mbt_rs

MoonBit と Rust の双方向トランスパイラ。

構文レベルの変換を中心に、標準ライブラリの API マッピング、所有権/ライフタイムの除去、async 構文の変換などを行う。完全な変換は目指さず、できる限り変換した上で、残りは TODO コメントとしてユーザーに委ねる方針。

## インストール

```bash
# rs2mbt (Rust → MoonBit) CLI
cd rs2mbt && cargo build --release
# バイナリ: rs2mbt/target/release/rs2mbt
```

mbt2rs (MoonBit → Rust) は MoonBit ライブラリとして利用。

## 使い方

### Rust → MoonBit

```bash
# 単一ファイル変換
rs2mbt input.rs > output.mbt

# マクロ展開してから変換 (推奨)
just expand-and-convert path/to/rust/crate > output.mbt

# 変換品質レポート
rs2mbt --report input.rs
```

### MoonBit → Rust

```moonbit
let rust_code = @to_rust.to_rust(impls)
```

### 品質レポート

```bash
just quality-report path/to/file.rs
just quality-report-all  # fixtures/real_projects/ 全体
```

実プロジェクトでの変換率:

| ファイル | 行数 | 変換率 | 備考 |
|---------|------|--------|------|
| calculator.rs | 211 | 100% | 式木 + eval + simplify |
| stack.rs | 114 | 100% | ジェネリックスタック |
| quality_test.rs | 181 | 97% | 混合パターン |
| linear_map.rs | 606 | 93% | Vec ベース Map |
| `cargo expand` 後 | - | 99% | マクロ展開で向上 |

## 変換対応表

### 型

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
| `Box<T>` / `Rc<T>` / `Arc<T>` | `T` (GC で不要) |

### 構文

| Rust | MoonBit |
|------|---------|
| `fn foo<T>(x: T) -> T` | `fn[T] foo(x: T) -> T` |
| `impl<T> S<T> { fn m(&self) }` | `fn S::m(self: S[T])` |
| `#[derive(Debug, Eq)]` | `} derive(Show, Eq)` |
| `if let Some(x) = opt` | `if opt is Some(x)` |
| `expr.await` | `expr` (await 不要) |
| `vec![1,2,3]` | `[1,2,3]` |
| `format!("{} {}", a, b)` | `"\{a} \{b}"` |
| `assert_eq!(a, b)` | `assert_eq(a, b)` |
| `println!("hi")` | `println("hi")` |
| `String::from("s")` | `"s"` |
| `Vec::new()` | `[]` |
| `Box::new(x)` | `x` |
| `x.clone()` | `x` (GC) |
| `x.borrow()` | `x` (GC) |
| `x.len()` | `x.length()` |
| `x.to_lowercase()` | `x.to_lower()` |
| `#[test] fn test_foo()` | `test "foo" { }` |
| `matches!(x, Pat)` | `x is Pat` |
| `!expr` | `not(expr)` |
| `loop { }` | `while true { }` |
| `for (a,b) in v` | `for _item in v { let (a,b) = _item }` |

### 所有権 / ライフタイム

全て除去。MoonBit は GC。

| Rust | MoonBit |
|------|---------|
| `&T` / `&mut T` | `T` |
| `*const T` / `*mut T` | `T` |
| `Box<T>` | `T` |
| `Rc<T>` / `Arc<T>` | `T` (NOTE コメント付き) |
| `Cell<T>` / `RefCell<T>` | `T` |
| `'a` ライフタイム | 除去 |
| `*expr` (deref) | `expr` |

### Iterator 系 trait impl

| Rust | MoonBit |
|------|---------|
| `impl Iterator for X` | スキップ (`.iter()` / `for-in` で対応) |
| `impl IntoIterator` | スキップ |
| `impl Clone` / `Copy` | 除去 |
| `impl Index` | スキップ (`op_get` / `op_set`) |
| `impl Extend` | スキップ |

### 未対応 (TODO コメント出力)

- `macro_rules!` 定義 → `cargo expand` で解決
- `unsafe` ブロック → コメント付きで除去
- `tokio::select!` → MoonBit に対応なし
- 複雑な trait object / dynamic dispatch

## テスト

```bash
# 全テスト実行
just test-all

# 内訳
just test          # mbt2rs MoonBit テスト (100)
just cargo-test    # rs2mbt Rust テスト (139)
just behavioral-test  # 双方向 behavioral equivalence (Rust 72 + MoonBit 74)
```

behavioral test は同じ関数に対して Rust と MoonBit で同じアサーションを実行し、変換後のコードが元と同じ結果を返すことを検証する。

## CI

GitHub Actions で 3 ジョブ:
- **MoonBit**: lint + type check + mbt2rs テスト
- **Rust**: rs2mbt ユニットテスト + ラウンドトリップテスト
- **Behavioral equivalence**: Rust cargo test + 変換後 MoonBit moon test

## ライセンス

MIT

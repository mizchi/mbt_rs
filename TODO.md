# TODO

## Completed

- [x] `format!` マクロ → MoonBit string interpolation
- [x] `impl` メソッドの `self` パラメータに型注釈付与
- [x] `mut` パラメータ展開 (`fn foo(mut x: i32)` → body先頭に `let mut x = x`)
- [x] `String::from("lit")` → `"lit"`, `Vec::new()` → `[]` 等コンストラクタ変換
- [x] `cargo expand` 連携
- [x] `moon fmt` 自動適用 (`just convert <file>`)
- [x] struct variant パターンマッチ (`Instr::Store { addr, val }` → `Store(addr~, val~)`)
- [x] `let mut` 不要な mut 除去 (コレクション初期化時)
- [x] Self ジェネリクス解決 (`impl<T> Stack<T>` → `self : Stack[T]`)
- [x] WARNING コメント付き仮変換 (macro_rules, extern, union 等)
- [x] async fn / .await 基本変換
- [x] 品質レポート (`rs2mbt --report`)
- [x] Bytes API マッピング (&[u8] → Bytes)
- [x] Behavioral equivalence testing (Rust = MoonBit 同一結果検証)
- [x] README.md

## Known Conversion Issues (moon check で失敗するパターン)

### `let mut` + Array element set
```rust
let mut arr = arr;  // Rust needs mut for arr[i] = x
arr[j] = value;     // MoonBit Array doesn't need mut for element set
```
**影響**: insertion_sort 等のインプレースアルゴリズム
**対策**: `let mut` → `let` に変換すべきだが、他の mutation (再代入) と区別が必要

### `as` 型キャスト
```rust
total as f64 / arr.len() as f64  // Rust cast
```
**MoonBit**: `total.to_double()` / `arr.length().to_double()` に変換すべき
**現状**: `(total : Double)` と出力 (型制約構文で、キャストではない)

### `.push_str()` on String
```rust
result.push_str(s);  // Rust String method
```
**MoonBit**: String は immutable、`+` で連結。StringBuilder なら `.write_string()`
**現状**: `.push()` にマッピングされるが MoonBit String に push がない

### Iterator chain `.collect()`
```rust
vec![1,2,3].iter().map(|x| x * 2).collect::<Vec<i32>>()
```
**MoonBit**: `.iter().map(fn(x) { x * 2 }).to_array()`
**問題**: クロージャの戻り値型が `Unit` に推論される (Rust 側の型情報がない)

### `break` in `for` loop
```rust
for x in arr { if cond { break; } }
```
**MoonBit**: `for` body の最後の式が `break` だと Unit 型の不一致が起こる場合あり

### `fn(mut param)`
```rust
fn count_digits(mut n: i32) -> i32 { while n > 0 { n = n / 10; } }
```
**現状**: `let mut n = n` を先頭に挿入するが、展開された `mut` と配列操作の `mut` が混在すると moon check で警告

## Medium Priority (未実装)

- [ ] `Iterator` チェーン最適化 (`.iter().map(f).collect()` → `.map(f)`)
  - 特に `.iter()` 除去 (MoonBit Array は直接 `.map()` 等を持つ)
  - クロージャ戻り値型の推定
- [ ] `as` キャスト → `.to_int()` / `.to_double()` 等のメソッド呼び出しに変換
- [ ] `String::push_str` → `+` 連結または `StringBuilder` パターン検出
- [ ] `#[cfg(test)] mod tests { ... }` → テスト関数の抽出
- [ ] `while let Some(x) = iter.next()` → `for x in iter` パターン検出
- [ ] `trait` impl の精度向上 (trait メソッドの where 句解決)
- [ ] `Fn`/`FnMut`/`FnOnce` trait bound → クロージャ型のコンテキスト判定

## Low Priority (発展的)

- [ ] `async`/`await` : tokio ↔ moonbitlang/async 詳細変換
- [ ] `enum` の `#[repr(u8)]` 等のアトリビュート対応
- [ ] `const fn` → MoonBit の const 対応
- [ ] `impl` ブロック内のジェネリクス制約伝播
- [ ] mbt2rs CLI バイナリ (現在は rs2mbt のみ)
- [ ] mooncakes / crates.io パッケージ公開

## Async: tokio ↔ moonbitlang/async

| Rust (tokio) | MoonBit (moonbitlang/async) | 変換難易度 |
|---|---|---|
| `async fn foo() -> T` | `async fn foo() -> T` | 低 |
| `foo().await` | `foo()` | 低 (await 除去) |
| `#[tokio::main] async fn main()` | `fn main { @async.run(async_main) }` | 中 |
| `tokio::spawn(async { ... })` | `@async.spawn(async { ... })` | 低〜中 |
| `tokio::join!(a, b)` | `@async.join(a, b)` | 中 |
| `tokio::select!` | 未対応 | 高 |
| `tokio::sync::Mutex` | 不要 (GC) | 除去 |

## Known Limitations

- Rust のライフタイム・所有権は全て除去 (MoonBit は GC)
- `unsafe` ブロックはコメント付きで除去
- `macro_rules!` 定義は変換不可 (`cargo expand` で解決)
- 型推論は行わない (syn はパースのみ)
- `dyn Trait` → trait 名のみ
- Rust の `impl` ブロック内ジェネリクス制約は部分的にのみ伝播

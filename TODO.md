# TODO

## High Priority (すぐできる・高コスパ)

- [x] `format!` マクロ → MoonBit string interpolation (`format!("x={}", x)` → `"x=\{x}"`)
- [x] `impl` メソッドの `self` パラメータに型注釈付与 (`self : Type`)
- [x] `mut` パラメータ展開 (`fn foo(mut x: i32)` → body先頭に `let mut x = x`)
- [x] `String::from("lit")` → `"lit"` コンストラクタ簡略化
- [x] `Vec::new()` → `[]`, `String::new()` → `""`, `HashMap::new()` → `{}` コンストラクタ変換
- [ ] 生成 MoonBit コードに `moon fmt` 自動適用 (justfile タスク)

## Medium Priority (中程度の労力)

- [ ] `Iterator` チェーンの最適化 (`.iter().map(f).collect()` → `.map(f)`)
- [ ] `#[cfg(test)] mod tests { ... }` → テスト関数の抽出と `test` ブロック変換
- [ ] `while let Some(x) = iter.next()` → `for x in iter` パターン検出
- [ ] `trait` + `impl` → MoonBit `impl Trait for Type with method` の精度向上 (self 型解決)
- [ ] struct variant のパターンマッチ (`Instruction::Store { addr, value }`)
- [ ] `let mut` の不要な `mut` 検出・除去 (MoonBit は Array.push 等で mut 不要)
- [ ] `Fn`/`FnMut`/`FnOnce` trait bound → MoonBit クロージャ型のコンテキスト判定

## Low Priority (発展的)

- [ ] README.md (対応パターン一覧・使い方・制限事項)
- [ ] エラーメッセージ改善 (変換できなかった箇所に理由コメント)
- [ ] `async`/`await` : tokio ↔ moonbitlang/async 変換 (下記参照)
- [ ] `enum` の `#[repr(u8)]` 等のアトリビュート対応
- [ ] `const fn` → MoonBit の const 対応
- [ ] `impl` ブロック内のジェネリクス制約伝播
- [ ] mbt2rs CLI バイナリ (現在は rs2mbt のみ)
- [ ] mooncakes パッケージとして公開

## Async: tokio ↔ moonbitlang/async 変換

tokio と moonbitlang/async は共に「async fn + await + タスクスポーン + イベントループ」モデル。
構文レベルで対応可能な範囲が大きい。

### 対応関係

| Rust (tokio) | MoonBit (moonbitlang/async) | 変換難易度 |
|---|---|---|
| `async fn foo() -> T` | `async fn foo() -> T` | 低 (キーワード同じ) |
| `foo().await` | `foo()` | 低 (MoonBit は await 不要、普通に呼ぶ) |
| `#[tokio::main] async fn main()` | `fn main { @async.run(async_main) }` | 中 (エントリポイント変換) |
| `tokio::spawn(async { ... })` | `@async.spawn(async { ... })` | 低〜中 |
| `tokio::join!(a, b)` | `@async.join(a, b)` | 中 |
| `tokio::select!` | 未対応 (MoonBit に直接対応なし) | 高 |
| `tokio::time::sleep(dur)` | `@async.sleep(dur)` | 低 |
| `tokio::sync::Mutex` | MoonBit は GC + シングルスレッドなので不要 | 除去 |
| `tokio::sync::mpsc::channel` | `@channel` 等 | 中 |
| `tokio::fs::read_to_string` | `@fs.read_to_string` | 中 (API 対応) |

### 変換戦略

1. `#[tokio::main]` → `fn main { @async.run(...) }` ラッパー生成
2. `.await` → 除去 (MoonBit は await 不要)
3. `tokio::spawn` → `@async.spawn`
4. `tokio::sync::*` → GC 前提で簡略化 (Mutex → 除去, channel → @channel)
5. エラー型: `Result<T, Box<dyn Error>>` → `T!Error`

### 前提条件

- moonbitlang/async の API が安定していること
- MoonBit 側の async/await 構文の確定

## Known Limitations (既知の制限)

- Rust のライフタイム・所有権は全て除去 (MoonBit は GC)
- `unsafe` ブロックはコメント付きで除去
- `macro_rules!` は変換不可 (TODO コメント出力)
- Rust の `trait object` (`dyn Trait`) → trait 名のみ
- `async`/`await` は未対応 (tokio ↔ moonbitlang/async の計画あり、上記参照)
- 型推論は行わない (syn はパースのみ)

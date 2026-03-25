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
- [ ] `async`/`await` → MoonBit async 対応
- [ ] `enum` の `#[repr(u8)]` 等のアトリビュート対応
- [ ] `const fn` → MoonBit の const 対応
- [ ] `impl` ブロック内のジェネリクス制約伝播
- [ ] mbt2rs CLI バイナリ (現在は rs2mbt のみ)
- [ ] mooncakes パッケージとして公開

## Known Limitations (既知の制限)

- Rust のライフタイム・所有権は全て除去 (MoonBit は GC)
- `unsafe` ブロックはコメント付きで除去
- `macro_rules!` は変換不可 (TODO コメント出力)
- Rust の `trait object` (`dyn Trait`) → trait 名のみ
- `async`/`await` は未対応
- 型推論は行わない (syn はパースのみ)

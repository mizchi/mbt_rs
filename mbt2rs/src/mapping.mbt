///|
/// Conversion kind for function/method mappings.
pub enum RustCallKind {
  /// Convert to Rust macro: `name!(args)`
  Macro(String)
  /// Convert to different function: `name(args)`
  Func(String)
  /// Prefix operator: `!x`
  PrefixOp(String)
  /// Statement form: `let _ = x`
  Statement(String)
  /// No mapping (use as-is)
  None
} derive(Eq)

///|
/// Lookup table for MoonBit free function → Rust equivalent.
pub fn rust_lookup_call(name : String) -> RustCallKind {
  match name {
    // Assertions
    "assert_eq" => Macro("assert_eq")
    "assert_ne" => Macro("assert_ne")
    "assert_true" => Macro("assert")
    "assert_false" => Macro("assert")
    // IO
    "println" => Macro("println")
    "print" => Macro("print")
    "eprintln" => Macro("eprintln")
    "eprint" => Macro("eprint")
    "debug" => Macro("dbg")
    // Panic / abort
    "panic" | "abort" => Macro("panic")
    // Logic
    "not" => PrefixOp("!")
    // Discard
    "ignore" => Statement("let _ =")
    // Numeric
    "abs" => Func("abs")
    "min" => Func("std::cmp::min")
    "max" => Func("std::cmp::max")
    // Default
    _ => None
  }
}

///|
/// Lookup table for MoonBit method name → Rust method name.
/// Returns empty string if no mapping (use as-is).
pub fn rust_lookup_method(method_name : String) -> String {
  match method_name {
    // === Length / size ===
    "length" => "len"
    "size" => "len"
    // === String ===
    "to_string" => "to_string"
    "contains" => "contains"
    "starts_with" => "starts_with"
    "ends_with" => "ends_with"
    "trim" => "trim"
    "trim_start" => "trim_start"
    "trim_end" => "trim_end"
    "split" => "split"
    "to_lower" => "to_lowercase"
    "to_upper" => "to_uppercase"
    "replace" => "replace"
    // === Array / Vec ===
    "push" => "push"
    "pop" => "pop"
    "get" => "get"
    "set" => "insert"
    "first" => "first"
    "last" => "last"
    "rev" => "reverse"
    "sort" => "sort"
    "sort_by" => "sort_by"
    "is_empty" => "is_empty"
    "map" => "iter().map"
    "mapi" => "iter().enumerate().map"
    "filter" => "iter().filter"
    "fold" => "iter().fold"
    "reduce" => "iter().reduce"
    "flat_map" => "iter().flat_map"
    "flatten" => "into_iter().flatten"
    "zip" => "iter().zip"
    "each" => "iter().for_each"
    "eachi" => "iter().enumerate().for_each"
    "any" => "iter().any"
    "all" => "iter().all"
    "find" => "iter().find"
    "find_index" => "iter().position"
    "iter" => "iter"
    "to_array" => "collect::<Vec<_>>"
    "append" => "extend"
    "clear" => "clear"
    "truncate" => "truncate"
    "retain" => "retain"
    "swap" => "swap"
    "take" => "take"
    "drop" => "skip"
    "minimum" => "min"
    "maximum" => "max"
    // === Option ===
    "unwrap" => "unwrap"
    "or" => "unwrap_or"
    "or_else" => "unwrap_or_else"
    "or_default" => "unwrap_or_default"
    "is_some" => "is_some"
    "is_none" => "is_none"
    "map_or" => "map_or"
    "ok_or" => "ok_or"
    "ok_or_else" => "ok_or_else"
    // === Result ===
    "is_ok" => "is_ok"
    "is_err" => "is_err"
    "ok" => "ok"
    "err" => "err"
    "map_err" => "map_err"
    // === Numeric ===
    "abs" => "abs"
    "pow" => "pow"
    "sqrt" => "sqrt"
    "floor" => "floor"
    "ceil" => "ceil"
    "round" => "round"
    "to_int" => "try_into().unwrap_or(0)"
    "to_double" => "into"
    // === Clone / copy ===
    "copy" => "clone"
    _ => ""
  }
}

///|
/// Map MoonBit derive trait name to Rust derive trait name(s).
pub fn rust_lookup_derive(name : String) -> String {
  match name {
    "Show" => "Debug"
    "Eq" => "PartialEq, Eq"
    "Compare" => "PartialOrd, Ord"
    "Hash" => "Hash"
    "Default" => "Default"
    _ => name
  }
}

///|
/// Map MoonBit type name to Rust type name.
pub fn rust_lookup_type(name : String) -> String {
  match name {
    // Primitives
    "Int" => "i32"
    "UInt" => "u32"
    "Int64" => "i64"
    "UInt64" => "u64"
    "Float" => "f32"
    "Double" => "f64"
    "Bool" => "bool"
    "String" => "String"
    "Byte" => "u8"
    "Unit" => "()"
    "Char" => "char"
    // Collections
    "Array" => "Vec"
    "FixedArray" => "Vec"
    "Map" => "HashMap"
    "Set" => "HashSet"
    // Standard types
    "Option" => "Option"
    "Result" => "Result"
    "Iter" => "impl Iterator"
    // String builders
    "Buffer" | "StringBuilder" => "String"
    "Bytes" => "Vec<u8>"
    // Traits
    "Show" => "Debug"
    "Compare" => "Ord"
    "Eq" => "Eq"
    "Hash" => "Hash"
    "Default" => "Default"
    _ => name
  }
}

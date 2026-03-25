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
    "assert_false" => Macro("assert") // ideally assert!(!x), close enough
    // IO
    "println" => Macro("println")
    "print" => Macro("print")
    "eprintln" => Macro("eprintln")
    "eprint" => Macro("eprint")
    // Panic / abort
    "panic" | "abort" => Macro("panic")
    // Logic
    "not" => PrefixOp("!")
    // Discard
    "ignore" => Statement("let _ =")
    // Numeric
    "abs" => Func("abs") // works as method in Rust: x.abs()
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
    // Length / size
    "length" => "len"
    "size" => "len"
    // String
    "to_string" => "to_string"
    "contains" => "contains"
    "starts_with" => "starts_with"
    "ends_with" => "ends_with"
    "trim" => "trim"
    "split" => "split"
    "to_lower" => "to_lowercase"
    "to_upper" => "to_uppercase"
    "substring" => "get" // approximate: str.substring(start, end) → &str[start..end]
    // Array / Vec
    "push" => "push"
    "pop" => "pop"
    "get" => "get"
    "set" => "insert" // approximate
    "map" => "iter().map"
    "filter" => "iter().filter"
    "fold" => "iter().fold"
    "iter" => "iter"
    "rev" => "rev"
    "sort" => "sort"
    "is_empty" => "is_empty"
    "flatten" => "into_iter().flatten"
    "zip" => "iter().zip"
    "each" => "iter().for_each"
    "any" => "iter().any"
    "all" => "iter().all"
    "find" => "iter().find"
    // Option
    "unwrap" => "unwrap"
    "or" => "or"
    "or_else" => "or_else"
    "map_or" => "map_or"
    "is_some" => "is_some"
    "is_none" => "is_none"
    // Result
    "is_ok" => "is_ok"
    "is_err" => "is_err"
    // Conversion
    "to_int" => "try_into().unwrap_or(0)" // approximate
    "to_double" => "into"
    // Clone
    "copy" => "clone"
    // Default (no mapping)
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
    "Array" => "Vec"
    "FixedArray" => "Vec"
    "Map" => "HashMap"
    "Iter" => "impl Iterator"
    "Char" => "char"
    "Bytes" => "Vec<u8>"
    "Buffer" | "StringBuilder" => "String"
    _ => name
  }
}

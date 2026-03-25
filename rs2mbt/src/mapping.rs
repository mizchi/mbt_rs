/// Map Rust type name to MoonBit type name.
pub fn lookup_type(name: &str) -> &str {
    match name {
        "i32" => "Int",
        "u32" => "UInt",
        "i64" => "Int64",
        "u64" => "UInt64",
        "f32" => "Float",
        "f64" => "Double",
        "bool" => "Bool",
        "String" => "String",
        "u8" => "Byte",
        "char" => "Char",
        "Vec" => "Array",
        "HashMap" => "Map",
        "Option" => "Option",
        "Result" => "Result",
        "Box" => "",     // unwrap: Box<T> → T
        "Rc" => "",      // unwrap: Rc<T> → T
        "Arc" => "",     // unwrap: Arc<T> → T
        "Cow" => "",     // unwrap: Cow<'_, T> → T
        "Cell" => "",    // unwrap: Cell<T> → T
        "RefCell" => "", // unwrap: RefCell<T> → T
        "Mutex" => "",   // unwrap: Mutex<T> → T
        "Pin" => "",     // unwrap: Pin<T> → T
        "usize" => "Int",
        "isize" => "Int",
        "str" => "String",
        _ => name,
    }
}

/// Map Rust method name to MoonBit method name.
pub fn lookup_method(name: &str) -> &str {
    match name {
        "len" => "length",
        "to_string" => "to_string",
        "contains" => "contains",
        "starts_with" => "starts_with",
        "ends_with" => "ends_with",
        "trim" => "trim",
        "to_lowercase" => "to_lower",
        "to_uppercase" => "to_upper",
        "push" => "push",
        "pop" => "pop",
        "is_empty" => "is_empty",
        "unwrap" => "unwrap",
        "clone" => "copy",
        _ => name,
    }
}

/// Map Rust macro to MoonBit function.
pub fn lookup_macro(name: &str) -> Option<&str> {
    match name {
        "assert_eq" => Some("assert_eq"),
        "assert_ne" => Some("assert_ne"),
        "assert" => Some("assert_true"),
        "println" => Some("println"),
        "print" => Some("print"),
        "eprintln" => Some("eprintln"),
        "panic" => Some("abort"),
        "matches" => None, // handled specially as `is` expression
        "todo" => None, // MoonBit uses `_` or `...`
        "vec" => None,  // handled specially
        "format" => None, // handled as string interpolation
        _ => None,
    }
}

/// Map Rust derive trait to MoonBit derive trait.
pub fn lookup_derive(name: &str) -> &str {
    match name {
        "Debug" => "Show",
        "PartialEq" => "", // skip, Eq covers both
        "Eq" => "Eq",
        "PartialOrd" => "", // skip, Compare covers both
        "Ord" => "Compare",
        "Hash" => "Hash",
        "Default" => "Default",
        "Clone" => "", // MoonBit has no Clone derive
        "Copy" => "",  // MoonBit has no Copy derive
        _ => name,
    }
}

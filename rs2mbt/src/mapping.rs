/// Map Rust type name to MoonBit type name.
pub fn lookup_type(name: &str) -> &str {
    match name {
        // Primitives
        "i8" | "i16" | "i32" | "isize" => "Int",
        "u8" => "Byte",
        "u16" | "u32" => "UInt",
        "i64" => "Int64",
        "u64" => "UInt64",
        "f32" => "Float",
        "f64" => "Double",
        "bool" => "Bool",
        "char" => "Char",
        "usize" => "Int",

        // String types
        "String" => "String",
        "str" => "String",
        "OsStr" | "OsString" => "String",
        "CStr" | "CString" => "String",

        // Collections
        "Vec" => "Array",
        "VecDeque" => "Array",   // approximate
        "LinkedList" => "Array", // approximate
        "HashMap" => "Map",
        "BTreeMap" => "Map",
        "HashSet" => "Set",      // MoonBit @hashset
        "BTreeSet" => "Set",

        // Standard types
        "Option" => "Option",
        "Result" => "Result",

        // Trait names (used in bounds)
        "PartialOrd" => "Compare",
        "Ord" => "Compare",
        "PartialEq" => "Eq",
        "Display" => "Show",
        "From" => "From",
        "Into" => "Into",
        "Iterator" => "Iter",
        "IntoIterator" => "Iter",

        // Smart pointers → unwrap
        "Box" => "",
        "Rc" => "",
        "Arc" => "",
        "Cow" => "",
        "Cell" => "",
        "RefCell" => "",
        "Mutex" => "",
        "RwLock" => "",
        "Pin" => "",
        "MutexGuard" => "",
        "Ref" => "",     // std::cell::Ref
        "RefMut" => "",  // std::cell::RefMut

        _ => name,
    }
}

/// Map Rust method name to MoonBit method name.
/// Returns "" for identity operations (to be stripped).
pub fn lookup_method(name: &str) -> &str {
    match name {
        // === Length / size ===
        "len" => "length",
        "count" => "length",
        "size" => "length",

        // === Bytes / binary ===
        "as_bytes" => "to_bytes",          // String → Bytes
        "into_bytes" => "to_bytes",        // String → Bytes
        "from_utf8" => "to_string",        // Bytes → String (static method, approximate)
        "from_utf8_lossy" => "to_string",  // Bytes → String
        "to_vec" => "to_array",            // &[u8] → Array[Byte]

        // === String methods ===
        "to_string" => "to_string",
        "as_str" => "",               // identity: &String → &str
        "to_owned" => "",             // identity in GC
        "contains" => "contains",
        "starts_with" => "starts_with",
        "ends_with" => "ends_with",
        "trim" => "trim",
        "trim_start" => "trim_start",
        "trim_end" => "trim_end",
        "to_lowercase" => "to_lower",
        "to_uppercase" => "to_upper",
        "replace" => "replace",
        "split" => "split",
        "chars" => "iter",
        "bytes" => "iter",
        "lines" => "split",          // approximate
        "repeat" => "to_string", // approximate
        "is_empty" => "is_empty",
        "push_str" => "write_string", // MoonBit: StringBuilder.write_string(s)

        // === Vec / Array methods ===
        "push" => "push",
        "pop" => "pop",
        "get" => "get",
        "first" => "first",
        "last" => "last",
        "insert" => "insert",
        "remove" => "remove",
        "swap" => "swap",
        "reverse" => "rev",
        "sort" => "sort",
        "sort_by" => "sort_by",
        "clear" => "clear",
        "truncate" => "truncate",
        "extend" => "append",         // approximate
        "drain" => "drain",
        "retain" => "retain",
        "iter" => "",             // MoonBit Array has methods directly, no .iter() needed
        "iter_mut" => "",
        "into_iter" => "",
        "map" => "map",
        "filter" => "filter",
        "flat_map" => "flat_map",
        "fold" => "fold",
        "reduce" => "reduce",
        "zip" => "zip",
        "enumerate" => "mapi",        // MoonBit: arr.mapi(fn(i, x) { ... })
        "for_each" => "each",
        "any" => "any",
        "all" => "all",
        "find" => "find",
        "position" => "find_index",   // MoonBit: arr.find_index(...)
        "skip" => "drop",             // MoonBit Iter: iter.drop(n)
        "take" => "take",
        "flatten" => "flatten",
        "collect" => "",               // MoonBit: .map()/.filter() already return Array
        "sum" => "fold(init=0, fn(a, b) { a + b })", // approximate
        "min" => "minimum",
        "max" => "maximum",
        "cloned" => "",               // identity in GC
        "copied" => "",               // identity in GC
        "as_slice" => "",             // identity
        "as_mut_slice" => "",         // identity

        // === Option methods ===
        "unwrap" => "unwrap",
        "unwrap_or" => "or",          // MoonBit: opt.or(default)
        "unwrap_or_else" => "or_else",
        "unwrap_or_default" => "or_default",
        "is_some" => "is_some",       // MoonBit: opt is Some(_) or method
        "is_none" => "is_none",
        "and_then" => "map",
        "or" => "or",
        "or_else" => "or_else",
        "expect" => "unwrap",         // MoonBit has no expect, use unwrap
        "ok_or" => "ok_or",
        "ok_or_else" => "ok_or_else",
        "transpose" => "flatten",

        // === Result methods ===
        "is_ok" => "is_ok",
        "is_err" => "is_err",
        "ok" => "ok",
        "err" => "err",
        "map_err" => "map_err",

        // === Numeric methods ===
        "abs" => "abs",
        "pow" => "pow",
        "sqrt" => "sqrt",
        "floor" => "floor",
        "ceil" => "ceil",
        "round" => "round",
        "signum" => "signum",
        "to_be_bytes" | "to_le_bytes" | "to_ne_bytes" => "to_bytes",

        // === Ownership/lifetime identity ops ===
        "clone" => "",
        "borrow" => "",
        "borrow_mut" => "",
        "as_ref" => "",
        "as_mut" => "",
        "deref" => "",
        "deref_mut" => "",
        "into_inner" => "",            // unwrap wrapper
        "lock" => "",                  // Mutex::lock → identity in GC
        "read" => "",                  // RwLock::read
        "write" => "",                 // RwLock::write

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
        "eprint" => Some("eprint"),
        "panic" | "unreachable" => Some("abort"),
        "dbg" => Some("debug"),
        "matches" => None, // handled specially as `is` expression
        "todo" => None,    // MoonBit uses `...`
        "vec" => None,     // handled specially as array literal
        "format" => None,  // handled as string interpolation
        "write" | "writeln" => None, // handled specially
        _ => None,
    }
}

/// Check if a type name is a wrapper that should be documented when stripped.
pub fn is_wrapper_type(name: &str) -> bool {
    matches!(name, "Rc" | "Arc" | "RefCell" | "Mutex" | "RwLock")
}

/// Check if a function call is a wrapper constructor that should be unwrapped.
/// Check if a call is a string conversion that can be simplified.
/// e.g., String::from("lit") → "lit", "lit".to_string() → "lit"
pub fn is_string_constructor(path: &str) -> bool {
    matches!(path, "String::from" | "String::to_string")
}

pub fn is_wrapper_constructor(path: &str) -> bool {
    matches!(
        path,
        "Box::new" | "Rc::new" | "Arc::new" | "Cell::new" | "RefCell::new"
            | "Mutex::new" | "RwLock::new" | "Pin::new"
    )
}

/// Check if a call path is a type constructor that should be simplified.
/// e.g., Vec::new() → [], String::new() → "", HashMap::new() → {}
pub fn lookup_constructor(path: &str) -> Option<&str> {
    match path {
        "Array::new" | "Vec::new" => Some("[]"),
        "String::new" => Some("\"\""),
        "Map::new" | "HashMap::new" => Some("{}"),
        "Option::None" | "None" => Some("None"),
        "Bytes::new" => Some("Bytes::new(0)"),
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
        "Clone" | "Copy" => "", // GC'd, not needed
        "Serialize" | "Deserialize" => "", // serde, skip
        _ => name,
    }
}

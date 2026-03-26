///|
/// Print a constant value as Rust.
pub fn rust_print_constant(buf : StringBuilder, c : @syntax.Constant) -> Unit {
  match c {
    Bool(b) => buf.write_string(if b { "true" } else { "false" })
    Byte(repr) => {
      buf.write_string("b'")
      buf.write_string(repr)
      buf.write_char('\'')
    }
    Bytes(repr) => {
      buf.write_string("b\"")
      buf.write_string(repr)
      buf.write_char('"')
    }
    Char(repr) => {
      buf.write_char('\'')
      buf.write_string(repr)
      buf.write_char('\'')
    }
    Int(s) => buf.write_string(s)
    Int64(s) => {
      buf.write_string(s)
      buf.write_string("i64")
    }
    UInt(s) => {
      buf.write_string(s)
      buf.write_string("u32")
    }
    UInt64(s) => {
      buf.write_string(s)
      buf.write_string("u64")
    }
    Float(s) => {
      buf.write_string(s)
      if not(s.contains(".")) {
        buf.write_string(".0")
      }
      buf.write_string("f32")
    }
    Double(s) => {
      buf.write_string(s)
      if not(s.contains(".")) {
        buf.write_string(".0")
      }
      buf.write_string("f64")
    }
    String(repr) => {
      buf.write_char('"')
      buf.write_string(repr)
      buf.write_char('"')
    }
    BigInt(s) => {
      buf.write_string(
        "/* TODO(transpile): BigInt literal\n * Reason: Rust has no built-in BigInt type.\n * Fix: Use `num_bigint::BigInt::from(",
      )
      buf.write_string(s)
      buf.write_string(")` from the `num-bigint` crate. */ ")
      buf.write_string(s)
    }
  }
}

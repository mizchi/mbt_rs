/// Hex encoding/decoding - adapted from Rust hex crate (0.4.3) via rs2mbt
/// This file passes moon check + moon test.

pub enum FromHexError {
  InvalidHexCharacter(c~ : Char, index~ : Int)
  OddLength
  InvalidStringLength
} derive(Show, Eq)

let hex_chars_lower : Bytes = b"0123456789abcdef"
let hex_chars_upper : Bytes = b"0123456789ABCDEF"

fn val(c : Byte, idx : Int) -> Result[Byte, FromHexError] {
  if c >= b'A' && c <= b'F' {
    Ok(c - b'A' + 10)
  } else if c >= b'a' && c <= b'f' {
    Ok(c - b'a' + 10)
  } else if c >= b'0' && c <= b'9' {
    Ok(c - b'0')
  } else {
    Err(InvalidHexCharacter(c=c.to_char(), index=idx))
  }
}

fn byte2hex(byte : Byte, table : Bytes) -> (Byte, Byte) {
  let high = table[(byte.to_int() & 0xf0) >> 4]
  let low = table[byte.to_int() & 0x0f]
  (high, low)
}

pub fn hex_encode(input : Bytes) -> String {
  let buf = StringBuilder::new()
  for i in 0..<input.length() {
    let (high, low) = byte2hex(input[i], hex_chars_lower)
    buf.write_string(high.to_char().to_string())
    buf.write_string(low.to_char().to_string())
  }
  buf.to_string()
}

pub fn hex_encode_upper(input : Bytes) -> String {
  let buf = StringBuilder::new()
  for i in 0..<input.length() {
    let (high, low) = byte2hex(input[i], hex_chars_upper)
    buf.write_string(high.to_char().to_string())
    buf.write_string(low.to_char().to_string())
  }
  buf.to_string()
}

pub fn hex_decode(hex : String) -> Result[Array[Byte], FromHexError] {
  let len = hex.length()
  if len % 2 != 0 {
    return Err(OddLength)
  }
  let out_len = len / 2
  let result : Array[Byte] = Array::make(out_len, b'\x00')
  for i in 0..<out_len {
    let c1 = hex[i * 2].to_int().to_byte()
    let c2 = hex[i * 2 + 1].to_int().to_byte()
    let high = match val(c1, i * 2) {
      Ok(v) => v
      Err(e) => return Err(e)
    }
    let low = match val(c2, i * 2 + 1) {
      Ok(v) => v
      Err(e) => return Err(e)
    }
    result[i] = ((high.to_int() << 4) | low.to_int()).to_byte()
  }
  Ok(result)
}

test "val digits" {
  assert_eq(val(b'0', 0), Ok(b'\x00'))
  assert_eq(val(b'9', 0), Ok(b'\x09'))
  assert_eq(val(b'a', 0), Ok(b'\x0a'))
  assert_eq(val(b'f', 0), Ok(b'\x0f'))
  assert_eq(val(b'A', 0), Ok(b'\x0a'))
  assert_eq(val(b'F', 0), Ok(b'\x0f'))
}

test "val invalid" {
  assert_true(val(b'g', 3) is Err(_))
}

test "hex_encode" {
  assert_eq(hex_encode(b"foobar"), "666f6f626172")
  assert_eq(hex_encode(b""), "")
}

test "hex_encode_upper" {
  assert_eq(hex_encode_upper(b"foobar"), "666F6F626172")
}

test "hex_decode valid" {
  assert_eq(hex_decode("666f6f626172"), Ok([b'\x66', b'\x6f', b'\x6f', b'\x62', b'\x61', b'\x72']))
  assert_eq(hex_decode(""), Ok([]))
}

test "hex_decode uppercase" {
  assert_eq(hex_decode("666F6F626172"), Ok([b'\x66', b'\x6f', b'\x6f', b'\x62', b'\x61', b'\x72']))
}

test "hex_decode odd length" {
  assert_eq(hex_decode("1"), Err(OddLength))
}

test "hex_decode invalid char" {
  assert_true(hex_decode("66ag") is Err(InvalidHexCharacter(..)))
}

test "roundtrip" {
  let original = b"Hello"
  let encoded = hex_encode(original)
  assert_eq(encoded, "48656c6c6f")
  let decoded = hex_decode(encoded)
  assert_eq(decoded, Ok([b'\x48', b'\x65', b'\x6c', b'\x6c', b'\x6f']))
}

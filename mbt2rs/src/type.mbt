///|
/// Check if a type is Unit.
pub fn rust_is_unit_type(ty : @syntax.Type) -> Bool {
  match ty {
    Name(constr_id~, ..) =>
      match constr_id.id {
        Ident(name~) => name == "Unit"
        _ => false
      }
    _ => false
  }
}

///|
/// Map MoonBit type name to Rust type name.
/// Delegates to the centralized mapping table.
fn rust_map_type_name(name : String) -> String {
  rust_lookup_type(name)
}

///|
/// Print a type expression as Rust.
pub fn rust_print_type(buf : StringBuilder, ty : @syntax.Type) -> Unit {
  match ty {
    Any(..) => buf.write_char('_')
    Arrow(args~, res~, err~, ..) => {
      buf.write_string("fn(")
      rust_print_type_list(buf, args)
      buf.write_string(") -> ")
      match err {
        ErrorType(ty~) | MaybeError(ty~) => {
          buf.write_string("Result<")
          rust_print_type(buf, res)
          buf.write_string(", ")
          rust_print_type(buf, ty)
          buf.write_char('>')
        }
        DefaultErrorType(..) => {
          buf.write_string("Result<")
          rust_print_type(buf, res)
          buf.write_string(", Box<dyn std::error::Error>>")
        }
        _ => rust_print_type(buf, res)
      }
    }
    Tuple(tys~, ..) => {
      buf.write_char('(')
      rust_print_type_list(buf, tys)
      buf.write_char(')')
    }
    Name(constr_id~, tys~, ..) => {
      rust_print_constr_id(buf, constr_id)
      if not(tys.is_empty()) {
        buf.write_char('<')
        rust_print_type_list(buf, tys)
        buf.write_char('>')
      }
    }
    Option(ty~, ..) => {
      buf.write_string("Option<")
      rust_print_type(buf, ty)
      buf.write_char('>')
    }
    Object(constr_id) => {
      buf.write_string("dyn ")
      rust_print_constr_id(buf, constr_id)
    }
  }
}

///|
fn rust_print_type_list(
  buf : StringBuilder,
  tys : @list.List[@syntax.Type],
) -> Unit {
  let mut first = true
  for ty in tys {
    if not(first) {
      buf.write_string(", ")
    }
    first = false
    rust_print_type(buf, ty)
  }
}

///|
fn rust_print_constr_id(buf : StringBuilder, id : @syntax.ConstrId) -> Unit {
  rust_print_long_ident(buf, id.id)
}

///|
fn rust_print_long_ident(buf : StringBuilder, id : @syntax.LongIdent) -> Unit {
  match id {
    Ident(name~) => buf.write_string(rust_map_type_name(name))
    Dot(pkg~, id~) => {
      buf.write_string(pkg)
      buf.write_string("::")
      buf.write_string(id)
    }
  }
}

///|
fn rust_print_type_name(buf : StringBuilder, tn : @syntax.TypeName) -> Unit {
  if tn.is_object {
    buf.write_string("dyn ")
  }
  rust_print_long_ident(buf, tn.name)
}

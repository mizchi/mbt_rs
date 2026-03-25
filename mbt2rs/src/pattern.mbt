///|
/// Print a pattern as Rust.
pub fn rust_print_pattern(buf : StringBuilder, pat : @syntax.Pattern) -> Unit {
  match pat {
    Alias(pat~, alias_~, ..) => {
      // Rust uses `pat @ name` instead of `pat as name`
      rust_print_pattern(buf, pat)
      buf.write_string(" @ ")
      buf.write_string(alias_.name)
    }
    Any(..) => buf.write_char('_')
    Array(pats~, ..) => {
      buf.write_char('[')
      rust_print_array_patterns(buf, pats)
      buf.write_char(']')
    }
    Constant(c~, ..) => rust_print_constant(buf, c)
    Constraint(pat~, ty~, ..) => {
      // Rust doesn't have pattern type constraints; emit pattern with comment
      rust_print_pattern(buf, pat)
      buf.write_string(" /* : ")
      rust_print_type(buf, ty)
      buf.write_string(" */")
    }
    Constr(constr~, args~, is_open~, ..) => {
      rust_print_constructor(buf, constr)
      match args {
        Some(args) if not(args.is_empty()) => {
          // Check if any arg has a label -> struct variant pattern
          let has_labels = args
            .iter()
            .any(fn(a) {
              match a.kind {
                Labelled(_)
                | LabelledPun(_)
                | LabelledOption(..)
                | LabelledOptionPun(..) => true
                _ => false
              }
            })
          if has_labels {
            buf.write_string(" { ")
            rust_print_constr_pat_args_struct(buf, args)
            if is_open {
              buf.write_string(", ..")
            }
            buf.write_string(" }")
          } else {
            buf.write_char('(')
            rust_print_constr_pat_args(buf, args)
            if is_open {
              buf.write_string(", ..")
            }
            buf.write_char(')')
          }
        }
        _ => if is_open { buf.write_string("(..)") }
      }
    }
    Or(pat1~, pat2~, ..) => {
      rust_print_pattern(buf, pat1)
      buf.write_string(" | ")
      rust_print_pattern(buf, pat2)
    }
    Tuple(pats~, ..) => {
      buf.write_char('(')
      let mut first = true
      for p in pats {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        rust_print_pattern(buf, p)
      }
      buf.write_char(')')
    }
    Var(binder) => buf.write_string(binder.name)
    Record(fields~, is_closed~, ..) => {
      buf.write_string("{ ")
      let mut first = true
      for f in fields {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        if f.is_pun {
          buf.write_string(f.label.name)
        } else {
          buf.write_string(f.label.name)
          buf.write_string(": ")
          rust_print_pattern(buf, f.pattern)
        }
      }
      if not(is_closed) {
        if not(fields.is_empty()) {
          buf.write_string(", ")
        }
        buf.write_string("..")
      }
      buf.write_string(" }")
    }
    Map(..) =>
      rust_todo(
        buf,
        "map pattern matching",
        reason="MoonBit can pattern match on map keys `{ \"key\": value }`. Rust HashMap has no pattern matching support.",
        hint="Use `if let Some(value) = map.get(\"key\")` or extract values with `.get()` before matching.",
      )
    Range(lhs~, rhs~, kind~, ..) => {
      rust_print_pattern(buf, lhs)
      match kind {
        Inclusive | InclusiveMissingEqual => buf.write_string("..=")
        Exclusive => buf.write_string("..")
      }
      rust_print_pattern(buf, rhs)
    }
    SpecialConstr(binder~, args~, ..) => {
      buf.write_string(binder.name)
      buf.write_char('(')
      rust_print_constr_pat_args(buf, args)
      buf.write_char(')')
    }
  }
}

///|
fn rust_print_array_patterns(
  buf : StringBuilder,
  pats : @syntax.ArrayPatterns,
) -> Unit {
  match pats {
    Closed(pats) => {
      let mut first = true
      for p in pats {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        match p {
          Pattern(pat) => rust_print_pattern(buf, pat)
          _ =>
            rust_todo(
              buf,
              "array spread pattern (string/bytes/const)",
              reason="MoonBit supports spreading strings/bytes/constants in array patterns.",
              hint="Extract the spread value before matching and compare manually.",
            )
        }
      }
    }
    Open(before, after, binder) => {
      let mut first = true
      for p in before {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        match p {
          Pattern(pat) => rust_print_pattern(buf, pat)
          _ =>
            rust_todo(
              buf,
              "array spread pattern (string/bytes/const)",
              reason="MoonBit supports spreading strings/bytes/constants in array patterns.",
              hint="Extract the spread value before matching and compare manually.",
            )
        }
      }
      if not(first) {
        buf.write_string(", ")
      }
      match binder {
        NoBinder | Underscore => buf.write_string("..")
        BinderAs(b) | Binder(b) => {
          buf.write_string(b.name)
          buf.write_string(" @ ..")
        }
      }
      for p in after {
        buf.write_string(", ")
        match p {
          Pattern(pat) => rust_print_pattern(buf, pat)
          _ =>
            rust_todo(
              buf,
              "array spread pattern (string/bytes/const)",
              reason="MoonBit supports spreading strings/bytes/constants in array patterns.",
              hint="Extract the spread value before matching and compare manually.",
            )
        }
      }
    }
  }
}

///|
fn rust_print_constr_pat_args(
  buf : StringBuilder,
  args : @list.List[@syntax.ConstrPatArg],
) -> Unit {
  let mut first = true
  for arg in args {
    if not(first) {
      buf.write_string(", ")
    }
    first = false
    rust_print_pattern(buf, arg.pat)
  }
}

///|
fn rust_print_constr_pat_args_struct(
  buf : StringBuilder,
  args : @list.List[@syntax.ConstrPatArg],
) -> Unit {
  let mut first = true
  for arg in args {
    if not(first) {
      buf.write_string(", ")
    }
    first = false
    match arg.kind {
      Labelled(label) => {
        buf.write_string(label.name)
        buf.write_string(": ")
        rust_print_pattern(buf, arg.pat)
      }
      LabelledPun(label) => buf.write_string(label.name)
      LabelledOption(label~, ..) => {
        buf.write_string(label.name)
        buf.write_string(": ")
        rust_print_pattern(buf, arg.pat)
      }
      LabelledOptionPun(label~, ..) => buf.write_string(label.name)
      Positional => rust_print_pattern(buf, arg.pat)
    }
  }
}

///|
fn rust_print_constructor(buf : StringBuilder, c : @syntax.Constructor) -> Unit {
  match c.extra_info {
    TypeName(tn) => {
      rust_print_type_name(buf, tn)
      buf.write_string("::")
    }
    Package(pkg) => {
      buf.write_string(pkg)
      buf.write_string("::")
    }
    NoExtraInfo => ()
  }
  buf.write_string(c.name.name)
}

///|
/// Print an expression as Rust.
pub fn rust_print_expr(
  buf : StringBuilder,
  expr : @syntax.Expr,
  indent? : Int = 0,
) -> Unit {
  match expr {
    // Literals
    Constant(c~, ..) => rust_print_constant(buf, c)
    Unit(..) => buf.write_string("()")

    // Identifiers
    Ident(id~, ..) => rust_print_var(buf, id)
    Constr(constr~, ..) => rust_print_constructor(buf, constr)

    // Collections
    Array(exprs~, ..) => {
      buf.write_string("vec![")
      let mut first = true
      for e in exprs {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        rust_print_expr(buf, e)
      }
      buf.write_char(']')
    }
    ArraySpread(elems~, ..) => {
      buf.write_string("vec![")
      let mut first = true
      for e in elems {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        match e {
          Regular(expr) => rust_print_expr(buf, expr)
          Spread(..) =>
            rust_todo(
              buf,
              "array spread in literal",
              reason="MoonBit `[..expr]` spread has no direct vec! equivalent.",
              hint="Use `let mut v = vec![...]; v.extend(other);` to concatenate.",
            )
        }
      }
      buf.write_char(']')
    }
    Tuple(exprs~, ..) => {
      buf.write_char('(')
      let mut first = true
      for e in exprs {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        rust_print_expr(buf, e)
      }
      buf.write_char(')')
    }

    // Function application
    Apply(func~, args~, attr~, ..) => {
      // Check if this is a MoonBit builtin with a Rust mapping
      let call_kind = match func {
        Ident(id~, ..) =>
          match id.name {
            Ident(name~) => rust_lookup_call(name)
            _ => RustCallKind::None
          }
        _ => RustCallKind::None
      }
      match call_kind {
        Macro(macro_name) => {
          buf.write_string(macro_name)
          buf.write_string("!(")
          rust_print_arguments(buf, args)
          buf.write_char(')')
        }
        Func(func_name) => {
          buf.write_string(func_name)
          buf.write_char('(')
          rust_print_arguments(buf, args)
          buf.write_char(')')
        }
        PrefixOp(op) => {
          buf.write_string(op)
          rust_print_arguments(buf, args)
        }
        Statement(stmt) => {
          buf.write_string(stmt)
          buf.write_char(' ')
          rust_print_arguments(buf, args)
        }
        None => {
          rust_print_expr(buf, func)
          buf.write_char('(')
          rust_print_arguments(buf, args)
          buf.write_char(')')
        }
      }
      match attr {
        NoAttr => ()
        Exclamation =>
          buf.write_string(
            " /* NOTE: `!` = may raise error. Wrap in Result and use `?` or `.unwrap()`. */",
          )
        Question => buf.write_char('?')
      }
    }

    // Operators
    Infix(op~, lhs~, rhs~, ..) => {
      rust_print_expr(buf, lhs, indent~)
      buf.write_char(' ')
      rust_print_var(buf, op)
      buf.write_char(' ')
      rust_print_expr(buf, rhs, indent~)
    }
    Unary(op~, expr~, ..) => {
      rust_print_var(buf, op)
      rust_print_expr(buf, expr)
    }
    Pipe(lhs~, rhs~, ..) => {
      // Desugar pipe: lhs |> rhs  =>  rhs(lhs)
      rust_print_expr(buf, rhs)
      buf.write_char('(')
      rust_print_expr(buf, lhs)
      buf.write_char(')')
    }

    // Array operations
    ArrayGet(array~, index~, ..) => {
      rust_print_expr(buf, array)
      buf.write_char('[')
      rust_print_expr(buf, index)
      buf.write_char(']')
    }
    ArrayGetSlice(array~, start_index~, end_index~, ..) => {
      rust_print_expr(buf, array)
      buf.write_char('[')
      match start_index {
        Some(e) => rust_print_expr(buf, e)
        None => ()
      }
      buf.write_string("..")
      match end_index {
        Some(e) => rust_print_expr(buf, e)
        None => ()
      }
      buf.write_char(']')
    }
    ArraySet(array~, index~, value~, ..) => {
      rust_print_expr(buf, array)
      buf.write_char('[')
      rust_print_expr(buf, index)
      buf.write_string("] = ")
      rust_print_expr(buf, value)
    }
    ArrayAugmentedSet(op~, array~, index~, value~, ..) => {
      rust_print_expr(buf, array)
      buf.write_char('[')
      rust_print_expr(buf, index)
      buf.write_string("] ")
      rust_print_var(buf, op)
      buf.write_string("= ")
      rust_print_expr(buf, value)
    }

    // Control flow
    If(cond~, ifso~, ifnot~, ..) => {
      buf.write_string("if ")
      rust_print_expr(buf, cond)
      buf.write_string(" {\n")
      rust_print_indent(buf, indent + 1)
      rust_print_expr(buf, ifso, indent=indent + 1)
      buf.write_char('\n')
      rust_print_indent(buf, indent)
      buf.write_char('}')
      match ifnot {
        Some(e) => {
          buf.write_string(" else {\n")
          rust_print_indent(buf, indent + 1)
          rust_print_expr(buf, e, indent=indent + 1)
          buf.write_char('\n')
          rust_print_indent(buf, indent)
          buf.write_char('}')
        }
        None => ()
      }
    }
    Match(expr~, cases~, ..) => {
      buf.write_string("match ")
      rust_print_expr(buf, expr, indent~)
      buf.write_string(" {\n")
      for c in cases {
        rust_print_indent(buf, indent + 1)
        rust_print_pattern(buf, c.pattern)
        match c.guard_ {
          Some(g) => {
            buf.write_string(" if ")
            rust_print_expr(buf, g, indent~)
          }
          None => ()
        }
        buf.write_string(" => ")
        rust_print_expr(buf, c.body, indent=indent + 1)
        buf.write_string(",\n")
      }
      rust_print_indent(buf, indent)
      buf.write_char('}')
    }
    While(loop_cond~, loop_body~, while_else~, label~, ..) => {
      match label {
        Some(l) =>
          buf.write_string(
            "/* NOTE: MoonBit loop label `\{l.name}~` has no Rust equivalent. Use Rust loop labels `'\{l.name}: while` if needed. */ ",
          )
        None => ()
      }
      buf.write_string("while ")
      rust_print_expr(buf, loop_cond, indent~)
      buf.write_string(" {\n")
      rust_print_indent(buf, indent + 1)
      rust_print_expr(buf, loop_body, indent=indent + 1)
      buf.write_char('\n')
      rust_print_indent(buf, indent)
      buf.write_char('}')
      match while_else {
        Some(e) => {
          buf.write_string(
            " /* while-else: Rust has no while-else. This block runs when the loop condition is initially false or loop completes without break. */;\n",
          )
          rust_print_indent(buf, indent)
          rust_print_expr(buf, e, indent~)
        }
        None => ()
      }
    }
    ForEach(binders~, expr~, body~, else_block~, label~, ..) => {
      match label {
        Some(l) =>
          buf.write_string(
            "/* NOTE: MoonBit loop label `\{l.name}~`. Use Rust loop label `'\{l.name}: for` if needed. */ ",
          )
        None => ()
      }
      buf.write_string("for ")
      let binder_count = binders.length()
      if binder_count > 1 {
        buf.write_char('(')
      }
      let mut first = true
      for b in binders {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        match b {
          Some(binder) => buf.write_string(binder.name)
          None => buf.write_char('_')
        }
      }
      if binder_count > 1 {
        buf.write_char(')')
      }
      buf.write_string(" in ")
      rust_print_expr(buf, expr, indent~)
      buf.write_string(" {\n")
      rust_print_indent(buf, indent + 1)
      rust_print_expr(buf, body, indent=indent + 1)
      buf.write_char('\n')
      rust_print_indent(buf, indent)
      buf.write_char('}')
      match else_block {
        Some(e) => {
          buf.write_string(
            " /* for-else: Rust has no for-else. This block runs when the iterator is empty or loop completes without break. */;\n",
          )
          rust_print_indent(buf, indent)
          rust_print_expr(buf, e, indent~)
        }
        None => ()
      }
    }
    For(binders~, condition~, continue_block~, body~, for_else~, label~, ..) => {
      // Desugar: for i = 0; cond; i = i + 1 { body }
      //       → { let mut i = 0; while cond { body; i = i + 1; } }
      match label {
        Some(l) =>
          buf.write_string(
            "/* NOTE: loop label `\{l.name}~`. Use `'\{l.name}: while` in Rust. */ ",
          )
        None => ()
      }
      // Emit binders as let mut
      for b in binders {
        buf.write_string("let mut ")
        buf.write_string(b.0.name)
        buf.write_string(" = ")
        rust_print_expr(buf, b.1, indent~)
        buf.write_string(";\n")
        rust_print_indent(buf, indent)
      }
      buf.write_string("while ")
      match condition {
        Some(c) => rust_print_expr(buf, c, indent~)
        None => buf.write_string("true")
      }
      buf.write_string(" {\n")
      rust_print_indent(buf, indent + 1)
      rust_print_expr(buf, body, indent=indent + 1)
      // Emit continue block (update expressions)
      if not(continue_block.is_empty()) {
        buf.write_string(";\n")
        for cb in continue_block {
          rust_print_indent(buf, indent + 1)
          buf.write_string(cb.0.name)
          buf.write_string(" = ")
          rust_print_expr(buf, cb.1, indent=indent + 1)
          buf.write_string(";\n")
        }
      } else {
        buf.write_char('\n')
      }
      rust_print_indent(buf, indent)
      buf.write_char('}')
      match for_else {
        Some(e) => {
          buf.write_string(" /* for-else: */ ;\n")
          rust_print_indent(buf, indent)
          rust_print_expr(buf, e, indent~)
        }
        None => ()
      }
    }
    Loop(arg~, body~, label~, ..) => {
      // Desugar: loop arg { pat => expr }
      //       → { let mut _loop_arg = arg; loop { match _loop_arg { pat => { ... } } } }
      match label {
        Some(l) =>
          buf.write_string(
            "/* NOTE: loop label `\{l.name}~`. Use `'\{l.name}: loop` in Rust. */ ",
          )
        None => ()
      }
      buf.write_string("{\n")
      rust_print_indent(buf, indent + 1)
      buf.write_string("let mut _loop_arg = ")
      rust_print_expr(buf, arg, indent=indent + 1)
      buf.write_string(";\n")
      rust_print_indent(buf, indent + 1)
      buf.write_string("loop {\n")
      rust_print_indent(buf, indent + 2)
      buf.write_string("match _loop_arg {\n")
      for c in body {
        rust_print_indent(buf, indent + 3)
        rust_print_pattern(buf, c.pattern)
        match c.guard_ {
          Some(g) => {
            buf.write_string(" if ")
            rust_print_expr(buf, g, indent=indent + 3)
          }
          None => ()
        }
        buf.write_string(" => ")
        rust_print_expr(buf, c.body, indent=indent + 3)
        buf.write_string(",\n")
      }
      rust_print_indent(buf, indent + 2)
      buf.write_string("}\n")
      rust_print_indent(buf, indent + 1)
      buf.write_string("}\n")
      rust_print_indent(buf, indent)
      buf.write_char('}')
    }

    // Functions / closures
    Function(func~, ..) => {
      buf.write_string("|")
      rust_print_closure_params(buf, func.parameters)
      buf.write_string("| ")
      match func.return_type {
        Some(t) => {
          buf.write_string("-> ")
          rust_print_type(buf, t)
          buf.write_char(' ')
        }
        None => ()
      }
      buf.write_string("{\n")
      rust_print_indent(buf, indent + 1)
      rust_print_expr(buf, func.body, indent=indent + 1)
      buf.write_char('\n')
      rust_print_indent(buf, indent)
      buf.write_char('}')
    }

    // Let bindings
    Let(pattern~, expr~, body~, ..) => {
      buf.write_string("let ")
      // Handle type-annotated patterns: let x : T = ... => let x: T = ...
      match pattern {
        Constraint(pat~, ty~, ..) => {
          rust_print_pattern(buf, pat)
          buf.write_string(": ")
          rust_print_type(buf, ty)
        }
        _ => rust_print_pattern(buf, pattern)
      }
      buf.write_string(" = ")
      rust_print_expr(buf, expr, indent~)
      buf.write_string(";\n")
      rust_print_indent(buf, indent)
      rust_print_expr(buf, body, indent~)
    }
    LetMut(binder~, ty~, expr~, body~, ..) => {
      buf.write_string("let mut ")
      buf.write_string(binder.name)
      match ty {
        Some(t) => {
          buf.write_string(": ")
          rust_print_type(buf, t)
        }
        None => ()
      }
      buf.write_string(" = ")
      rust_print_expr(buf, expr)
      buf.write_string(";\n")
      rust_print_indent(buf, indent)
      rust_print_expr(buf, body, indent~)
    }
    LetFn(name~, func~, body~, ..) => {
      buf.write_string("let ")
      buf.write_string(name.name)
      buf.write_string(" = |")
      rust_print_closure_params(buf, func.parameters)
      buf.write_string("| ")
      match func.return_type {
        Some(t) => {
          buf.write_string("-> ")
          rust_print_type(buf, t)
          buf.write_char(' ')
        }
        None => ()
      }
      buf.write_string("{\n")
      rust_print_indent(buf, indent + 1)
      rust_print_expr(buf, func.body, indent=indent + 1)
      buf.write_char('\n')
      rust_print_indent(buf, indent)
      buf.write_string("};\n")
      rust_print_indent(buf, indent)
      rust_print_expr(buf, body, indent~)
    }
    LetRec(..) =>
      rust_todo(
        buf,
        "mutually recursive let bindings",
        reason="MoonBit `fn`/`fn` mutual recursion in let scope has no Rust let equivalent.",
        hint="Move recursive functions to module-level `fn` definitions.",
      )
    LetAnd(..) =>
      rust_todo(
        buf,
        "simultaneous let bindings (let ... and ...)",
        reason="MoonBit `let ... and ...` binds multiple values simultaneously.",
        hint="Split into sequential `let` bindings if they don't depend on each other, or use a tuple `let (a, b) = ...`.",
      )

    // Sequences
    Sequence(exprs~, last_expr~, ..) => {
      for e in exprs {
        rust_print_expr(buf, e, indent~)
        buf.write_string(";\n")
        rust_print_indent(buf, indent)
      }
      rust_print_expr(buf, last_expr, indent~)
    }

    // Records
    Record(type_name~, fields~, ..) => {
      match type_name {
        Some(tn) => {
          rust_print_type_name(buf, tn)
          buf.write_char(' ')
        }
        None => ()
      }
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
          rust_print_expr(buf, f.expr)
        }
      }
      buf.write_string(" }")
    }
    RecordUpdate(type_name~, record~, fields~, ..) => {
      match type_name {
        Some(tn) => {
          rust_print_type_name(buf, tn)
          buf.write_char(' ')
        }
        None => ()
      }
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
          rust_print_expr(buf, f.expr)
        }
      }
      buf.write_string(", ..")
      rust_print_expr(buf, record)
      buf.write_string(" }")
    }
    Field(record~, accessor~, ..) => {
      rust_print_expr(buf, record)
      buf.write_char('.')
      match accessor {
        Label(label) => buf.write_string(label.name)
        Index(tuple_index~, ..) => buf.write_string(tuple_index.to_string())
        Newtype(..) => buf.write_string("0") // newtype field access -> .0
      }
    }
    Method(type_name~, method_name~, ..) => {
      rust_print_type_name(buf, type_name)
      buf.write_string("::")
      buf.write_string(method_name.name)
    }
    DotApply(self~, method_name~, args~, return_self~, attr~, ..) => {
      rust_print_expr(buf, self)
      buf.write_char('.')
      let mapped = rust_lookup_method(method_name.name)
      if mapped.length() > 0 {
        buf.write_string(mapped)
      } else {
        buf.write_string(method_name.name)
      }
      buf.write_char('(')
      rust_print_arguments(buf, args)
      buf.write_char(')')
      match attr {
        NoAttr => ()
        Exclamation =>
          buf.write_string(
            " /* NOTE: `!` = may raise error. Wrap in Result and use `?` or `.unwrap()`. */",
          )
        Question => buf.write_char('?')
      }
      if return_self {
        buf.write_string(
          " /* NOTE: MoonBit `.method().` (trailing dot) returns self for chaining. In Rust, the method must return Self. */",
        )
      }
    }

    // Mutation and assignment
    Mutate(record~, accessor~, field~, augmented_by~, ..) => {
      rust_print_expr(buf, record)
      buf.write_char('.')
      match accessor {
        Label(label) => buf.write_string(label.name)
        Index(tuple_index~, ..) => buf.write_string(tuple_index.to_string())
        Newtype(..) => buf.write_string("0")
      }
      match augmented_by {
        Some(op) => {
          buf.write_char(' ')
          rust_print_var(buf, op)
          buf.write_string("= ")
        }
        None => buf.write_string(" = ")
      }
      rust_print_expr(buf, field)
    }
    Assign(var_~, expr~, augmented_by~, ..) => {
      rust_print_var(buf, var_)
      match augmented_by {
        Some(op) => {
          buf.write_char(' ')
          rust_print_var(buf, op)
          buf.write_string("= ")
        }
        None => buf.write_string(" = ")
      }
      rust_print_expr(buf, expr)
    }

    // Type constraints
    Constraint(expr~, ty~, ..) => {
      rust_print_expr(buf, expr)
      buf.write_string(" as ")
      rust_print_type(buf, ty)
    }
    As(expr~, trait_~, ..) => {
      rust_print_expr(buf, expr)
      buf.write_string(" as ")
      rust_print_type_name(buf, trait_)
    }
    Is(expr~, pat~, ..) => {
      buf.write_string("matches!(")
      rust_print_expr(buf, expr)
      buf.write_string(", ")
      rust_print_pattern(buf, pat)
      buf.write_char(')')
    }

    // Control flow keywords
    Return(return_value~, ..) => {
      buf.write_string("return")
      match return_value {
        Some(e) => {
          buf.write_char(' ')
          rust_print_expr(buf, e)
        }
        None => ()
      }
    }
    Raise(err_value~, ..) => {
      buf.write_string("return Err(")
      rust_print_expr(buf, err_value)
      buf.write_char(')')
    }
    Break(arg~, ..) => {
      buf.write_string("break")
      match arg {
        Some(e) => {
          buf.write_char(' ')
          rust_print_expr(buf, e)
        }
        None => ()
      }
    }
    Continue(..) => buf.write_string("continue")

    // Error handling
    Try(body~, catch_~, try_else~, ..) =>
      if not(catch_.is_empty()) || try_else is Some(_) {
        buf.write_string("match ")
        rust_print_expr(buf, body, indent~)
        buf.write_string(" {\n")
        if not(catch_.is_empty()) {
          rust_print_indent(buf, indent + 1)
          buf.write_string("Err(e) => match e {\n")
          for c in catch_ {
            rust_print_indent(buf, indent + 2)
            rust_print_pattern(buf, c.pattern)
            buf.write_string(" => ")
            rust_print_expr(buf, c.body, indent=indent + 2)
            buf.write_string(",\n")
          }
          rust_print_indent(buf, indent + 1)
          buf.write_string("},\n")
        }
        match try_else {
          Some(cases) => {
            rust_print_indent(buf, indent + 1)
            buf.write_string("Ok(v) => match v {\n")
            for c in cases {
              rust_print_indent(buf, indent + 2)
              rust_print_pattern(buf, c.pattern)
              buf.write_string(" => ")
              rust_print_expr(buf, c.body, indent=indent + 2)
              buf.write_string(",\n")
            }
            rust_print_indent(buf, indent + 1)
            buf.write_string("},\n")
          }
          None => {
            rust_print_indent(buf, indent + 1)
            buf.write_string("Ok(v) => v,\n")
          }
        }
        rust_print_indent(buf, indent)
        buf.write_char('}')
      } else {
        rust_print_expr(buf, body, indent~)
      }
    TryOperator(body~, kind~, ..) => {
      // Unwrap Group(Brace) wrapper that parser adds around try body
      let inner = match body {
        Group(expr~, group~, ..) if group is Brace => expr
        _ => body
      }
      match kind {
        Question => {
          rust_print_expr(buf, inner)
          buf.write_char('?')
        }
        Exclamation => {
          rust_print_expr(buf, inner)
          buf.write_string(".unwrap()")
        }
      }
    }

    // Guard
    Guard(cond~, otherwise~, body~, ..) => {
      buf.write_string("if !(")
      rust_print_expr(buf, cond)
      buf.write_string(") {\n")
      match otherwise {
        Some(e) => {
          rust_print_indent(buf, indent + 1)
          rust_print_expr(buf, e, indent=indent + 1)
        }
        None => {
          rust_print_indent(buf, indent + 1)
          buf.write_string("panic!(\"guard failed\")")
        }
      }
      buf.write_char('\n')
      rust_print_indent(buf, indent)
      buf.write_string("}\n")
      rust_print_indent(buf, indent)
      rust_print_expr(buf, body, indent~)
    }

    // Defer
    Defer(..) =>
      rust_todo(
        buf,
        "defer expression",
        reason="MoonBit `defer` runs code at scope exit. Rust has no built-in defer.",
        hint="Use a Drop guard: `let _guard = scopeguard::guard((), |_| { cleanup_code });` (scopeguard crate), or implement Drop on a wrapper struct.",
      )

    // Hole
    Hole(kind~, ..) =>
      match kind {
        Synthesized => buf.write_string("todo!()")
        Incomplete => buf.write_string("todo!(\"...\")")
        Todo => buf.write_string("todo!()")
      }

    // Map
    Map(elems~, ..) => {
      buf.write_string("HashMap::from([")
      let mut first = true
      for e in elems {
        if not(first) {
          buf.write_string(", ")
        }
        first = false
        buf.write_char('(')
        rust_print_constant(buf, e.key)
        buf.write_string(", ")
        rust_print_expr(buf, e.expr)
        buf.write_char(')')
      }
      buf.write_string("])")
    }

    // Group
    Group(expr~, group~, ..) =>
      match group {
        Paren => {
          buf.write_char('(')
          rust_print_expr(buf, expr)
          buf.write_char(')')
        }
        Brace => {
          buf.write_string("{\n")
          rust_print_indent(buf, indent + 1)
          rust_print_expr(buf, expr, indent=indent + 1)
          buf.write_char('\n')
          rust_print_indent(buf, indent)
          buf.write_char('}')
        }
      }

    // String interpolation
    Interp(elems~, ..) => {
      buf.write_string("format!(\"")
      let args : Array[StringBuilder] = []
      for e in elems {
        match e {
          Literal(repr~, ..) => buf.write_string(repr)
          Expr(expr~, ..) => {
            buf.write_string("{}")
            let arg_buf = StringBuilder::new()
            rust_print_expr(arg_buf, expr)
            args.push(arg_buf)
          }
          Source(src) => {
            buf.write_string("{}")
            let arg_buf = StringBuilder::new()
            arg_buf.write_string(src.source)
            args.push(arg_buf)
          }
        }
      }
      buf.write_char('"')
      for arg in args {
        buf.write_string(", ")
        buf.write_string(arg.to_string())
      }
      buf.write_char(')')
    }
    MultilineString(elems~, ..) => {
      // Convert to concat! of lines or a single string
      buf.write_string("concat!(")
      let mut first = true
      for e in elems {
        if not(first) {
          buf.write_string(", \"\\n\", ")
        }
        first = false
        match e {
          String(s) => {
            buf.write_char('"')
            buf.write_string(s)
            buf.write_char('"')
          }
          Interp(interp_elems) => {
            // Lines with interpolation become format!()
            buf.write_string("&format!(\"")
            let args : Array[StringBuilder] = []
            for ie in interp_elems {
              match ie {
                Literal(repr~, ..) => buf.write_string(repr)
                Expr(expr~, ..) => {
                  buf.write_string("{}")
                  let arg_buf = StringBuilder::new()
                  rust_print_expr(arg_buf, expr)
                  args.push(arg_buf)
                }
                Source(_) => ()
              }
            }
            buf.write_char('"')
            for arg in args {
              buf.write_string(", ")
              buf.write_string(arg.to_string())
            }
            buf.write_char(')')
          }
        }
      }
      buf.write_char(')')
    }
    LexMatch(..) =>
      rust_todo(
        buf,
        "lexmatch expression",
        reason="MoonBit `lexmatch` is a lexer DSL with regex patterns. No Rust equivalent.",
        hint="Use a lexer library (logos, nom) or manual regex matching with the `regex` crate.",
      )
    IsLexMatch(..) =>
      rust_todo(
        buf,
        "is lexmatch expression",
        reason="MoonBit `expr lexmatch? pat` checks if input matches a lex pattern.",
        hint="Use `regex::Regex::is_match()` or a manual prefix check.",
      )
    StaticAssert(body~, ..) => rust_print_expr(buf, body)
  }
}

///|
fn rust_print_var(buf : StringBuilder, v : @syntax.Var) -> Unit {
  match v.name {
    Ident(name~) => buf.write_string(name)
    Dot(pkg~, id~) => {
      buf.write_string(pkg)
      buf.write_string("::")
      buf.write_string(id)
    }
  }
}

///|
fn rust_print_arguments(
  buf : StringBuilder,
  args : @list.List[@syntax.Argument],
) -> Unit {
  let mut first = true
  for arg in args {
    if not(first) {
      buf.write_string(", ")
    }
    first = false
    match arg.kind {
      Positional => ()
      Labelled(_) => () // Rust has no labeled args; just pass the value
      LabelledPun(_) => () // For pun, the value IS the label name; just print value
      LabelledOption(..) => ()
      LabelledOptionPun(label~, ..) => {
        buf.write_string("Some(")
        buf.write_string(label.name)
        buf.write_char(')')
        continue // skip value printing
      }
    }
    rust_print_expr(buf, arg.value)
  }
}

///|
fn rust_print_closure_params(
  buf : StringBuilder,
  params : @list.List[@syntax.Parameter],
) -> Unit {
  let mut first = true
  for p in params {
    if not(first) {
      buf.write_string(", ")
    }
    first = false
    match p {
      Positional(binder~, ty~) | Labelled(binder~, ty~) => {
        buf.write_string(binder.name)
        match ty {
          Some(t) => {
            buf.write_string(": ")
            rust_print_type(buf, t)
          }
          None => ()
        }
      }
      DiscardPositional(ty~, ..) => {
        buf.write_char('_')
        match ty {
          Some(t) => {
            buf.write_string(": ")
            rust_print_type(buf, t)
          }
          None => ()
        }
      }
      Optional(binder~, ty~, ..) | QuestionOptional(binder~, ty~) => {
        buf.write_string(binder.name)
        match ty {
          Some(t) => {
            buf.write_string(": Option<")
            rust_print_type(buf, t)
            buf.write_char('>')
          }
          None => ()
        }
      }
    }
  }
}

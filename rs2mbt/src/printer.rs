use crate::mapping;
use syn::*;

/// Convert Rust source code to MoonBit source code.
pub fn to_moonbit(rust_src: &str) -> String {
    let file = syn::parse_file(rust_src).expect("Failed to parse Rust source");
    let mut buf = String::new();
    for item in &file.items {
        print_item(&mut buf, item, 0);
        buf.push('\n');
    }
    buf
}

fn indent(buf: &mut String, level: usize) {
    for _ in 0..level {
        buf.push_str("  ");
    }
}

fn print_item(buf: &mut String, item: &Item, level: usize) {
    match item {
        Item::Fn(f) => print_fn(buf, f, level),
        Item::Struct(s) => print_struct(buf, s, level),
        Item::Enum(e) => print_enum(buf, e, level),
        Item::Type(t) => print_type_alias(buf, t, level),
        Item::Const(c) => print_const(buf, c, level),
        Item::Trait(t) => print_trait(buf, t, level),
        Item::Impl(i) => print_impl(buf, i, level),
        Item::Use(_) => {} // skip use statements
        _ => {
            buf.push_str("// TODO(transpile): unsupported item\n");
        }
    }
}

fn print_visibility(buf: &mut String, vis: &Visibility) {
    match vis {
        Visibility::Public(_) => buf.push_str("pub "),
        Visibility::Restricted(_) => buf.push_str("pub "), // approximate
        Visibility::Inherited => {}
    }
}

fn print_fn(buf: &mut String, f: &ItemFn, level: usize) {
    indent(buf, level);
    print_visibility(buf, &f.vis);
    buf.push_str("fn ");
    buf.push_str(&f.sig.ident.to_string());
    print_generics(buf, &f.sig.generics);
    buf.push('(');
    print_fn_params(buf, &f.sig.inputs);
    buf.push(')');
    print_return_type(buf, &f.sig.output);
    buf.push_str(" {\n");
    print_block_body(buf, &f.block, level + 1);
    buf.push('\n');
    indent(buf, level);
    buf.push('}');
}

fn print_generics(buf: &mut String, generics: &Generics) {
    if generics.params.is_empty() {
        return;
    }
    buf.push('[');
    let mut first = true;
    for param in &generics.params {
        if !first {
            buf.push_str(", ");
        }
        first = false;
        match param {
            GenericParam::Type(t) => {
                buf.push_str(&t.ident.to_string());
                if !t.bounds.is_empty() {
                    buf.push_str(" : ");
                    let mut first_bound = true;
                    for bound in &t.bounds {
                        if !first_bound {
                            buf.push_str(" + ");
                        }
                        first_bound = false;
                        if let TypeParamBound::Trait(tb) = bound {
                            print_path(buf, &tb.path);
                        }
                    }
                }
            }
            GenericParam::Lifetime(_) => buf.push_str("/* lifetime */"),
            GenericParam::Const(_) => buf.push_str("/* const generic */"),
        }
    }
    buf.push(']');
}

fn print_fn_params(buf: &mut String, inputs: &punctuated::Punctuated<FnArg, token::Comma>) {
    let mut first = true;
    for arg in inputs {
        if !first {
            buf.push_str(", ");
        }
        first = false;
        match arg {
            FnArg::Receiver(_) => buf.push_str("self"),
            FnArg::Typed(pat_type) => {
                print_pat(buf, &pat_type.pat, 0);
                buf.push_str(" : ");
                print_type(buf, &pat_type.ty);
            }
        }
    }
}

fn print_return_type(buf: &mut String, ret: &ReturnType) {
    match ret {
        ReturnType::Default => {}
        ReturnType::Type(_, ty) => {
            // Check if return type is ()
            if let Type::Tuple(t) = ty.as_ref() {
                if t.elems.is_empty() {
                    return; // Unit → omit
                }
            }
            buf.push_str(" -> ");
            print_type(buf, ty);
        }
    }
}

fn print_type(buf: &mut String, ty: &Type) {
    match ty {
        Type::Path(tp) => {
            if let Some(seg) = tp.path.segments.last() {
                let ident_str = seg.ident.to_string();
                let name = mapping::lookup_type(&ident_str);
                buf.push_str(name);
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    buf.push('[');
                    let mut first = true;
                    for arg in &args.args {
                        if !first {
                            buf.push_str(", ");
                        }
                        first = false;
                        if let GenericArgument::Type(t) = arg {
                            print_type(buf, t);
                        }
                    }
                    buf.push(']');
                }
            }
        }
        Type::Tuple(t) => {
            if t.elems.is_empty() {
                buf.push_str("Unit");
            } else {
                buf.push('(');
                let mut first = true;
                for elem in &t.elems {
                    if !first {
                        buf.push_str(", ");
                    }
                    first = false;
                    print_type(buf, elem);
                }
                buf.push(')');
            }
        }
        Type::Reference(r) => {
            // Drop reference, MoonBit is GC'd
            print_type(buf, &r.elem);
        }
        Type::Slice(s) => {
            buf.push_str("Array[");
            print_type(buf, &s.elem);
            buf.push(']');
        }
        Type::BareFn(f) => {
            buf.push('(');
            let mut first = true;
            for arg in &f.inputs {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_type(buf, &arg.ty);
            }
            buf.push_str(") -> ");
            print_return_type_inner(buf, &f.output);
        }
        Type::Infer(_) => buf.push('_'),
        _ => buf.push_str("_"),
    }
}

fn print_return_type_inner(buf: &mut String, ret: &ReturnType) {
    match ret {
        ReturnType::Default => buf.push_str("Unit"),
        ReturnType::Type(_, ty) => print_type(buf, ty),
    }
}

fn print_struct(buf: &mut String, s: &ItemStruct, level: usize) {
    indent(buf, level);
    print_derive_attrs(buf, &s.attrs);
    print_visibility(buf, &s.vis);
    buf.push_str("struct ");
    buf.push_str(&s.ident.to_string());
    print_generics(buf, &s.generics);
    match &s.fields {
        Fields::Named(fields) => {
            buf.push_str(" {\n");
            for f in &fields.named {
                indent(buf, level + 1);
                if let Some(ident) = &f.ident {
                    buf.push_str(&ident.to_string());
                    buf.push_str(" : ");
                    print_type(buf, &f.ty);
                }
                buf.push('\n');
            }
            indent(buf, level);
            buf.push('}');
        }
        Fields::Unnamed(fields) => {
            buf.push('(');
            let mut first = true;
            for f in &fields.unnamed {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_type(buf, &f.ty);
            }
            buf.push(')');
        }
        Fields::Unit => {}
    }
}

fn print_enum(buf: &mut String, e: &ItemEnum, level: usize) {
    indent(buf, level);
    print_derive_attrs(buf, &e.attrs);
    print_visibility(buf, &e.vis);
    buf.push_str("enum ");
    buf.push_str(&e.ident.to_string());
    print_generics(buf, &e.generics);
    buf.push_str(" {\n");
    for variant in &e.variants {
        indent(buf, level + 1);
        buf.push_str(&variant.ident.to_string());
        match &variant.fields {
            Fields::Unnamed(fields) => {
                buf.push('(');
                let mut first = true;
                for f in &fields.unnamed {
                    if !first {
                        buf.push_str(", ");
                    }
                    first = false;
                    print_type(buf, &f.ty);
                }
                buf.push(')');
            }
            Fields::Named(fields) => {
                buf.push('(');
                let mut first = true;
                for f in &fields.named {
                    if !first {
                        buf.push_str(", ");
                    }
                    first = false;
                    if let Some(ident) = &f.ident {
                        buf.push_str(&ident.to_string());
                        buf.push_str("~ : ");
                        print_type(buf, &f.ty);
                    }
                }
                buf.push(')');
            }
            Fields::Unit => {}
        }
        buf.push('\n');
    }
    indent(buf, level);
    buf.push('}');
}

fn print_type_alias(buf: &mut String, t: &ItemType, level: usize) {
    indent(buf, level);
    print_visibility(buf, &t.vis);
    buf.push_str("type ");
    buf.push_str(&t.ident.to_string());
    print_generics(buf, &t.generics);
    buf.push_str(" = ");
    print_type(buf, &t.ty);
}

fn print_const(buf: &mut String, c: &ItemConst, level: usize) {
    indent(buf, level);
    print_visibility(buf, &c.vis);
    buf.push_str("const ");
    buf.push_str(&c.ident.to_string());
    buf.push_str(" : ");
    print_type(buf, &c.ty);
    buf.push_str(" = ");
    print_expr(buf, &c.expr, level);
}

fn print_trait(buf: &mut String, t: &ItemTrait, level: usize) {
    indent(buf, level);
    print_visibility(buf, &t.vis);
    buf.push_str("trait ");
    buf.push_str(&t.ident.to_string());
    if !t.supertraits.is_empty() {
        buf.push_str(" : ");
        let mut first = true;
        for bound in &t.supertraits {
            if !first {
                buf.push_str(" + ");
            }
            first = false;
            if let TypeParamBound::Trait(tb) = bound {
                print_path(buf, &tb.path);
            }
        }
    }
    buf.push_str(" {\n");
    for item in &t.items {
        if let TraitItem::Fn(method) = item {
            indent(buf, level + 1);
            buf.push_str(&method.sig.ident.to_string());
            buf.push('(');
            print_fn_params(buf, &method.sig.inputs);
            buf.push(')');
            print_return_type(buf, &method.sig.output);
            buf.push('\n');
        }
    }
    indent(buf, level);
    buf.push('}');
}

fn print_impl(buf: &mut String, i: &ItemImpl, level: usize) {
    indent(buf, level);
    if let Some((_, path, _)) = &i.trait_ {
        // impl Trait for Type { methods }
        for item in &i.items {
            if let ImplItem::Fn(method) = item {
                buf.push_str("impl ");
                print_path(buf, path);
                buf.push_str(" for ");
                print_type(buf, &i.self_ty);
                buf.push_str(" with ");
                buf.push_str(&method.sig.ident.to_string());
                buf.push('(');
                // Skip &self param for MoonBit
                let params: Vec<_> = method
                    .sig
                    .inputs
                    .iter()
                    .filter(|a| !matches!(a, FnArg::Receiver(_)))
                    .collect();
                let mut first = true;
                // Add self as first param
                buf.push_str("self");
                if !params.is_empty() {
                    buf.push_str(", ");
                }
                for arg in params {
                    if !first {
                        buf.push_str(", ");
                    }
                    first = false;
                    if let FnArg::Typed(pat_type) = arg {
                        print_pat(buf, &pat_type.pat, 0);
                        buf.push_str(" : ");
                        print_type(buf, &pat_type.ty);
                    }
                }
                buf.push(')');
                print_return_type(buf, &method.sig.output);
                buf.push_str(" {\n");
                print_block_body(buf, &method.block, level + 1);
                buf.push('\n');
                indent(buf, level);
                buf.push('}');
                buf.push('\n');
            }
        }
    }
}

fn print_derive_attrs(buf: &mut String, attrs: &[Attribute]) {
    for attr in attrs {
        if attr.path().is_ident("derive") {
            // Parse derive traits
            let mut derives = Vec::new();
            let _ = attr.parse_nested_meta(|meta| {
                let name = meta.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
                let mapped = mapping::lookup_derive(&name);
                if !mapped.is_empty() {
                    derives.push(mapped.to_string());
                }
                Ok(())
            });
            if !derives.is_empty() {
                buf.push_str(" derive(");
                buf.push_str(&derives.join(", "));
                buf.push(')');
            }
        }
    }
    // Insert newline if any derive was printed
}

fn print_path(buf: &mut String, path: &Path) {
    let mut first = true;
    for seg in &path.segments {
        if !first {
            buf.push('.');
        }
        first = false;
        let seg_str = seg.ident.to_string();
        let name = mapping::lookup_type(&seg_str);
        buf.push_str(name);
    }
}

fn print_block_body(buf: &mut String, block: &Block, level: usize) {
    let stmts = &block.stmts;
    for (i, stmt) in stmts.iter().enumerate() {
        let is_last = i == stmts.len() - 1;
        indent(buf, level);
        match stmt {
            Stmt::Local(local) => {
                print_local(buf, local, level);
            }
            Stmt::Expr(expr, semi) => {
                print_expr(buf, expr, level);
                if semi.is_some() && !is_last {
                    // statement with semicolon (not last)
                }
            }
            Stmt::Item(item) => print_item(buf, item, level),
            Stmt::Macro(m) => {
                print_macro_stmt(buf, m, level);
            }
        }
        if !is_last {
            buf.push('\n');
        }
    }
}

fn print_local(buf: &mut String, local: &Local, level: usize) {
    let is_mut = match &local.pat {
        Pat::Ident(pi) => pi.mutability.is_some(),
        Pat::Type(pt) => {
            if let Pat::Ident(pi) = pt.pat.as_ref() {
                pi.mutability.is_some()
            } else {
                false
            }
        }
        _ => false,
    };
    if is_mut {
        buf.push_str("let mut ");
    } else {
        buf.push_str("let ");
    }
    // Handle typed pattern: let x: Type = ...
    match &local.pat {
        Pat::Type(pt) => {
            print_pat(buf, &pt.pat, level);
            buf.push_str(" : ");
            print_type(buf, &pt.ty);
        }
        _ => print_pat(buf, &local.pat, level),
    }
    if let Some(init) = &local.init {
        buf.push_str(" = ");
        print_expr(buf, &init.expr, level);
    }
}

fn print_expr(buf: &mut String, expr: &Expr, level: usize) {
    match expr {
        Expr::Lit(lit) => print_lit(buf, &lit.lit),
        Expr::Path(p) => print_path(buf, &p.path),
        Expr::Unary(u) => {
            match u.op {
                UnOp::Neg(_) => buf.push('-'),
                UnOp::Not(_) => buf.push_str("not("),
                UnOp::Deref(_) | _ => {} // drop deref for MoonBit
            }
            print_expr(buf, &u.expr, level);
            if matches!(u.op, UnOp::Not(_)) {
                buf.push(')');
            }
        }
        Expr::Binary(b) => {
            print_expr(buf, &b.left, level);
            buf.push(' ');
            print_binop(buf, &b.op);
            buf.push(' ');
            print_expr(buf, &b.right, level);
        }
        Expr::Call(c) => {
            print_expr(buf, &c.func, level);
            buf.push('(');
            let mut first = true;
            for arg in &c.args {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_expr(buf, arg, level);
            }
            buf.push(')');
        }
        Expr::MethodCall(m) => {
            print_expr(buf, &m.receiver, level);
            buf.push('.');
            let method_str = m.method.to_string();
            let method = mapping::lookup_method(&method_str);
            buf.push_str(method);
            buf.push('(');
            let mut first = true;
            for arg in &m.args {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_expr(buf, arg, level);
            }
            buf.push(')');
        }
        Expr::If(i) => {
            // if let Some(x) = expr → match expr { Some(x) => ..., _ => ... }
            if let Expr::Let(let_expr) = i.cond.as_ref() {
                buf.push_str("match ");
                print_expr(buf, &let_expr.expr, level);
                buf.push_str(" {\n");
                indent(buf, level + 1);
                print_pat(buf, &let_expr.pat, level);
                buf.push_str(" => {\n");
                print_block_body(buf, &i.then_branch, level + 2);
                buf.push('\n');
                indent(buf, level + 1);
                buf.push('}');
                if let Some((_, else_expr)) = &i.else_branch {
                    buf.push('\n');
                    indent(buf, level + 1);
                    buf.push_str("_ => {\n");
                    if let Expr::Block(block) = else_expr.as_ref() {
                        print_block_body(buf, &block.block, level + 2);
                    } else {
                        indent(buf, level + 2);
                        print_expr(buf, else_expr, level + 2);
                    }
                    buf.push('\n');
                    indent(buf, level + 1);
                    buf.push('}');
                } else {
                    buf.push('\n');
                    indent(buf, level + 1);
                    buf.push_str("_ => ()");
                }
                buf.push('\n');
                indent(buf, level);
                buf.push('}');
            } else {
            buf.push_str("if ");
            print_expr(buf, &i.cond, level);
            buf.push_str(" {\n");
            print_block_body(buf, &i.then_branch, level + 1);
            buf.push('\n');
            indent(buf, level);
            buf.push('}');
            if let Some((_, else_expr)) = &i.else_branch {
                buf.push_str(" else {\n");
                if let Expr::Block(block) = else_expr.as_ref() {
                    print_block_body(buf, &block.block, level + 1);
                } else {
                    indent(buf, level + 1);
                    print_expr(buf, else_expr, level + 1);
                }
                buf.push('\n');
                indent(buf, level);
                buf.push('}');
            }
            } // close else branch of if-let check
        }
        Expr::Match(m) => {
            buf.push_str("match ");
            print_expr(buf, &m.expr, level);
            buf.push_str(" {\n");
            for arm in &m.arms {
                indent(buf, level + 1);
                print_pat(buf, &arm.pat, level);
                if let Some((_, guard)) = &arm.guard {
                    buf.push_str(" if ");
                    print_expr(buf, guard, level);
                }
                buf.push_str(" => ");
                print_expr(buf, &arm.body, level + 1);
                buf.push('\n');
            }
            indent(buf, level);
            buf.push('}');
        }
        Expr::Block(b) => {
            print_block_body(buf, &b.block, level);
        }
        Expr::Return(r) => {
            buf.push_str("return");
            if let Some(expr) = &r.expr {
                buf.push(' ');
                print_expr(buf, expr, level);
            }
        }
        Expr::Break(b) => {
            buf.push_str("break");
            if let Some(expr) = &b.expr {
                buf.push(' ');
                print_expr(buf, expr, level);
            }
        }
        Expr::Continue(_) => buf.push_str("continue"),
        Expr::Tuple(t) => {
            buf.push('(');
            let mut first = true;
            for elem in &t.elems {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_expr(buf, elem, level);
            }
            buf.push(')');
        }
        Expr::Array(a) => {
            buf.push('[');
            let mut first = true;
            for elem in &a.elems {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_expr(buf, elem, level);
            }
            buf.push(']');
        }
        Expr::Index(i) => {
            print_expr(buf, &i.expr, level);
            buf.push('[');
            print_expr(buf, &i.index, level);
            buf.push(']');
        }
        Expr::Field(f) => {
            print_expr(buf, &f.base, level);
            buf.push('.');
            match &f.member {
                Member::Named(ident) => buf.push_str(&ident.to_string()),
                Member::Unnamed(idx) => buf.push_str(&idx.index.to_string()),
            }
        }
        Expr::Assign(a) => {
            print_expr(buf, &a.left, level);
            buf.push_str(" = ");
            print_expr(buf, &a.right, level);
        }
        Expr::While(w) => {
            buf.push_str("while ");
            print_expr(buf, &w.cond, level);
            buf.push_str(" {\n");
            print_block_body(buf, &w.body, level + 1);
            buf.push('\n');
            indent(buf, level);
            buf.push('}');
        }
        Expr::ForLoop(f) => {
            buf.push_str("for ");
            print_pat(buf, &f.pat, level);
            buf.push_str(" in ");
            print_expr(buf, &f.expr, level);
            buf.push_str(" {\n");
            print_block_body(buf, &f.body, level + 1);
            buf.push('\n');
            indent(buf, level);
            buf.push('}');
        }
        Expr::Closure(c) => {
            buf.push_str("fn(");
            let mut first = true;
            for input in &c.inputs {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_pat(buf, input, level);
            }
            buf.push_str(") { ");
            print_expr(buf, &c.body, level);
            buf.push_str(" }");
        }
        Expr::Struct(s) => {
            buf.push_str("{ ");
            // MoonBit puts ..base before fields
            if let Some(rest) = &s.rest {
                buf.push_str("..");
                print_expr(buf, rest, level);
                if !s.fields.is_empty() {
                    buf.push_str(", ");
                }
            }
            let mut first = true;
            for f in &s.fields {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                buf.push_str(&f.member.to_token_stream().to_string());
                buf.push_str(": ");
                print_expr(buf, &f.expr, level);
            }
            buf.push_str(" }");
        }
        Expr::Paren(p) => {
            buf.push('(');
            print_expr(buf, &p.expr, level);
            buf.push(')');
        }
        Expr::Reference(r) => {
            // Drop & for MoonBit
            print_expr(buf, &r.expr, level);
        }
        Expr::Try(t) => {
            // expr? → try? { expr }
            // simplified: just print as-is with ! suffix
            print_expr(buf, &t.expr, level);
            buf.push('!');
        }
        Expr::Cast(c) => {
            buf.push('(');
            print_expr(buf, &c.expr, level);
            buf.push_str(" : ");
            print_type(buf, &c.ty);
            buf.push(')');
        }
        Expr::Macro(m) => {
            print_expr_macro(buf, m, level);
        }
        Expr::Range(r) => {
            // Rust range → MoonBit range (used in for loops or slicing)
            if let Some(start) = &r.start {
                print_expr(buf, start, level);
            }
            match &r.limits {
                RangeLimits::HalfOpen(_) => buf.push_str("..<"),
                RangeLimits::Closed(_) => buf.push_str("..="),
            }
            if let Some(end) = &r.end {
                print_expr(buf, end, level);
            }
        }
        Expr::Repeat(r) => {
            // [expr; n] → Array::make(n, expr)
            buf.push_str("Array::make(");
            print_expr(buf, &r.len, level);
            buf.push_str(", ");
            print_expr(buf, &r.expr, level);
            buf.push(')');
        }
        Expr::Let(l) => {
            // if let pattern = expr (used in if/while let)
            print_pat(buf, &l.pat, level);
            buf.push_str(" = ");
            print_expr(buf, &l.expr, level);
        }
        Expr::Loop(l) => {
            // loop { body } → while true { body }
            buf.push_str("while true {\n");
            print_block_body(buf, &l.body, level + 1);
            buf.push('\n');
            indent(buf, level);
            buf.push('}');
        }
        Expr::Unsafe(u) => {
            // unsafe { body } → body (drop unsafe, add comment)
            buf.push_str("// NOTE: was unsafe block\n");
            indent(buf, level);
            print_block_body(buf, &u.block, level);
        }
        Expr::Group(g) => {
            print_expr(buf, &g.expr, level);
        }
        Expr::Infer(_) => {
            buf.push('_');
        }
        _ => {
            buf.push_str("_");
            buf.push_str(" // TODO(transpile): unsupported expr");
        }
    }
}

fn print_expr_macro(buf: &mut String, mac: &ExprMacro, _level: usize) {
    let name = mac.mac.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
    if let Some(mbt_name) = mapping::lookup_macro(&name) {
        buf.push_str(mbt_name);
        buf.push('(');
        // Parse macro tokens as comma-separated exprs
        let tokens = mac.mac.tokens.to_string();
        buf.push_str(&tokens);
        buf.push(')');
    } else if name == "vec" {
        buf.push('[');
        let tokens = mac.mac.tokens.to_string();
        buf.push_str(&tokens);
        buf.push(']');
    } else if name == "format" {
        // format!("...", args) → string interpolation
        buf.push('"');
        let tokens = mac.mac.tokens.to_string();
        buf.push_str(&tokens);
        buf.push('"');
        buf.push_str(" // TODO: convert to string interpolation");
    } else if name == "todo" {
        buf.push_str("...");
    } else {
        buf.push_str(&name);
        buf.push('(');
        let tokens = mac.mac.tokens.to_string();
        buf.push_str(&tokens);
        buf.push(')');
    }
}

fn print_macro_stmt(buf: &mut String, mac: &StmtMacro, _level: usize) {
    let name = mac.mac.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
    if let Some(mbt_name) = mapping::lookup_macro(&name) {
        buf.push_str(mbt_name);
        buf.push('(');
        let tokens = mac.mac.tokens.to_string();
        buf.push_str(&tokens);
        buf.push(')');
    } else {
        buf.push_str(&name);
        buf.push('(');
        let tokens = mac.mac.tokens.to_string();
        buf.push_str(&tokens);
        buf.push(')');
    }
}

fn print_lit(buf: &mut String, lit: &Lit) {
    match lit {
        Lit::Int(i) => {
            let s = i.to_string();
            // Strip Rust suffixes
            let s = s.trim_end_matches("i32").trim_end_matches("i64")
                .trim_end_matches("u32").trim_end_matches("u64")
                .trim_end_matches("f32").trim_end_matches("f64")
                .trim_end_matches("usize").trim_end_matches("isize");
            buf.push_str(s);
            // Add MoonBit suffix if needed
            let suffix = i.suffix();
            match suffix {
                "i64" => buf.push('L'),
                "u32" => buf.push('U'),
                "u64" => buf.push_str("UL"),
                _ => {}
            }
        }
        Lit::Float(f) => {
            let s = f.to_string();
            let s = s.trim_end_matches("f32").trim_end_matches("f64");
            buf.push_str(s);
        }
        Lit::Str(s) => {
            buf.push('"');
            buf.push_str(&s.value());
            buf.push('"');
        }
        Lit::Char(c) => {
            buf.push('\'');
            buf.push(c.value());
            buf.push('\'');
        }
        Lit::Bool(b) => {
            buf.push_str(if b.value() { "true" } else { "false" });
        }
        Lit::ByteStr(b) => {
            buf.push_str("b\"");
            for byte in b.value() {
                buf.push(byte as char);
            }
            buf.push('"');
        }
        Lit::Byte(b) => {
            buf.push_str("b'");
            buf.push(b.value() as char);
            buf.push('\'');
        }
        _ => buf.push_str("_"),
    }
}

fn print_pat(buf: &mut String, pat: &Pat, level: usize) {
    match pat {
        Pat::Ident(pi) => {
            buf.push_str(&pi.ident.to_string());
        }
        Pat::Wild(_) => buf.push('_'),
        Pat::Tuple(t) => {
            buf.push('(');
            let mut first = true;
            for p in &t.elems {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_pat(buf, p, level);
            }
            buf.push(')');
        }
        Pat::TupleStruct(ts) => {
            print_path(buf, &ts.path);
            buf.push('(');
            let mut first = true;
            for p in &ts.elems {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_pat(buf, p, level);
            }
            buf.push(')');
        }
        Pat::Struct(s) => {
            buf.push_str("{ ");
            let mut first = true;
            for f in &s.fields {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                let member = f.member.to_token_stream().to_string();
                buf.push_str(&member);
                // Check if pun (field name == pattern ident)
                let is_pun = if let Pat::Ident(pi) = &*f.pat {
                    pi.ident == member.as_str()
                } else {
                    false
                };
                if !is_pun {
                    buf.push_str(": ");
                    print_pat(buf, &f.pat, level);
                }
            }
            if let Some(rest) = &s.rest {
                if !s.fields.is_empty() {
                    buf.push_str(", ");
                }
                let _ = rest;
                buf.push_str("..");
            }
            buf.push_str(" }");
        }
        Pat::Lit(l) => print_lit(buf, &l.lit),
        Pat::Type(pt) => {
            print_pat(buf, &pt.pat, level);
            buf.push_str(" : ");
            print_type(buf, &pt.ty);
        }
        Pat::Or(o) => {
            let mut first = true;
            for p in &o.cases {
                if !first {
                    buf.push_str(" | ");
                }
                first = false;
                print_pat(buf, p, level);
            }
        }
        Pat::Range(r) => {
            if let Some(lo) = &r.start {
                print_expr(buf, lo, level);
            }
            buf.push_str("..=");
            if let Some(hi) = &r.end {
                print_expr(buf, hi, level);
            }
        }
        Pat::Slice(s) => {
            buf.push('[');
            let mut first = true;
            for p in &s.elems {
                if !first {
                    buf.push_str(", ");
                }
                first = false;
                print_pat(buf, p, level);
            }
            buf.push(']');
        }
        _ => buf.push('_'),
    }
}

fn print_binop(buf: &mut String, op: &BinOp) {
    buf.push_str(match op {
        BinOp::Add(_) => "+",
        BinOp::Sub(_) => "-",
        BinOp::Mul(_) => "*",
        BinOp::Div(_) => "/",
        BinOp::Rem(_) => "%",
        BinOp::And(_) => "&&",
        BinOp::Or(_) => "||",
        BinOp::BitXor(_) => "^",
        BinOp::BitAnd(_) => "&",
        BinOp::BitOr(_) => "|",
        BinOp::Shl(_) => "<<",
        BinOp::Shr(_) => ">>",
        BinOp::Eq(_) => "==",
        BinOp::Lt(_) => "<",
        BinOp::Le(_) => "<=",
        BinOp::Ne(_) => "!=",
        BinOp::Ge(_) => ">=",
        BinOp::Gt(_) => ">",
        BinOp::AddAssign(_) => "+=",
        BinOp::SubAssign(_) => "-=",
        BinOp::MulAssign(_) => "*=",
        BinOp::DivAssign(_) => "/=",
        BinOp::RemAssign(_) => "%=",
        _ => "/* unknown op */",
    });
}

use quote::ToTokens;

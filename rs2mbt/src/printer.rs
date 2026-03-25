use crate::mapping;
use std::cell::RefCell;
use std::collections::BTreeSet;
use syn::*;

thread_local! {
    /// Track wrapper types (Rc, Arc, RefCell, Mutex) used during conversion.
    static USED_WRAPPER_TYPES: RefCell<BTreeSet<String>> = RefCell::new(BTreeSet::new());
    /// Current impl block's self type name (for resolving Self).
    static CURRENT_SELF_TYPE: RefCell<String> = RefCell::new(String::new());
}

fn record_wrapper_type(name: &str) {
    USED_WRAPPER_TYPES.with(|set| {
        set.borrow_mut().insert(name.to_string());
    });
}

fn take_wrapper_types() -> BTreeSet<String> {
    USED_WRAPPER_TYPES.with(|set| std::mem::take(&mut *set.borrow_mut()))
}

fn set_self_type(name: &str) {
    CURRENT_SELF_TYPE.with(|s| *s.borrow_mut() = name.to_string());
}

fn clear_self_type() {
    CURRENT_SELF_TYPE.with(|s| s.borrow_mut().clear());
}

fn get_self_type() -> String {
    CURRENT_SELF_TYPE.with(|s| s.borrow().clone())
}

/// Convert Rust source code to MoonBit source code.
pub fn to_moonbit(rust_src: &str) -> String {
    // Clear any leftover state
    let _ = take_wrapper_types();

    let file = syn::parse_file(rust_src).expect("Failed to parse Rust source");
    let mut body = String::new();
    for item in &file.items {
        print_item(&mut body, item, 0);
        body.push('\n');
    }

    // Collect wrapper types used and generate aliases
    let wrappers = take_wrapper_types();
    let mut buf = String::new();
    if !wrappers.is_empty() {
        buf.push_str("// NOTE: The following Rust ownership/synchronization types were stripped\n");
        buf.push_str("// during conversion because MoonBit is garbage-collected:\n");
        for w in &wrappers {
            let desc = match w.as_str() {
                "Rc" => "Rc<T> (shared ownership, reference counted) → T",
                "Arc" => "Arc<T> (thread-safe shared ownership) → T",
                "RefCell" => "RefCell<T> (interior mutability, runtime borrow check) → T",
                "Mutex" => "Mutex<T> (thread-safe interior mutability) → T",
                _ => "",
            };
            buf.push_str(&format!("//   {desc}\n"));
        }
        buf.push('\n');
    }
    // Trim leading empty lines from skipped items (use, cfg, etc.)
    let trimmed = body.trim_start_matches('\n');
    buf.push_str(trimmed);
    buf
}

fn indent(buf: &mut String, level: usize) {
    for _ in 0..level {
        buf.push_str("  ");
    }
}

fn print_item(buf: &mut String, item: &Item, level: usize) {
    match item {
        Item::Fn(f) => {
            // Check for #[test] attribute → emit MoonBit test block
            let is_test = f.attrs.iter().any(|a| a.path().is_ident("test"));
            if is_test {
                print_test_fn(buf, f, level);
            } else {
                print_fn(buf, f, level);
            }
        }
        Item::Struct(s) => print_struct(buf, s, level),
        Item::Enum(e) => print_enum(buf, e, level),
        Item::Type(t) => print_type_alias(buf, t, level),
        Item::Const(c) => print_const(buf, c, level),
        Item::Trait(t) => print_trait(buf, t, level),
        Item::Impl(i) => print_impl(buf, i, level),
        Item::Static(s) => print_static(buf, s, level),
        Item::Use(_) => {} // skip use statements
        Item::Mod(m) => {
            // module → just print inner items
            if let Some((_, items)) = &m.content {
                for item in items {
                    print_item(buf, item, level);
                    buf.push('\n');
                }
            }
        }
        _ => {
            buf.push_str("// TODO(transpile): unsupported item\n");
        }
    }
}

fn print_test_fn(buf: &mut String, f: &ItemFn, level: usize) {
    indent(buf, level);
    // Convert fn name: test_foo_bar → "foo bar"
    let name = f.sig.ident.to_string();
    let test_name = name
        .strip_prefix("test_")
        .unwrap_or(&name)
        .replace('_', " ");
    buf.push_str("test \"");
    buf.push_str(&test_name);
    buf.push_str("\" {\n");
    print_block_body(buf, &f.block, level + 1);
    buf.push('\n');
    indent(buf, level);
    buf.push('}');
}

fn print_visibility(buf: &mut String, vis: &Visibility) {
    match vis {
        Visibility::Public(_) => buf.push_str("pub "),
        Visibility::Restricted(_) => buf.push_str("pub "), // approximate
        Visibility::Inherited => {}
    }
}

fn print_fn(buf: &mut String, f: &ItemFn, level: usize) {
    // Extract where clause fn types for parameter resolution
    set_where_fn_types(extract_where_fn_types(&f.sig.generics));
    indent(buf, level);
    print_visibility(buf, &f.vis);
    if f.sig.asyncness.is_some() {
        buf.push_str("async ");
    }
    buf.push_str("fn");
    print_generics(buf, &f.sig.generics);
    buf.push(' ');
    buf.push_str(&f.sig.ident.to_string());
    buf.push('(');
    print_fn_params(buf, &f.sig.inputs);
    buf.push(')');
    print_return_type(buf, &f.sig.output);
    buf.push_str(" {\n");
    // Emit `let mut x = x` for mut params (MoonBit params are always immutable)
    for arg in &f.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pi) = pat_type.pat.as_ref() {
                if pi.mutability.is_some() {
                    indent(buf, level + 1);
                    buf.push_str("let mut ");
                    buf.push_str(&pi.ident.to_string());
                    buf.push_str(" = ");
                    buf.push_str(&pi.ident.to_string());
                    buf.push('\n');
                }
            }
        }
    }
    print_block_body(buf, &f.block, level + 1);
    buf.push('\n');
    indent(buf, level);
    buf.push('}');
}

fn print_generics(buf: &mut String, generics: &Generics) {
    // Filter out lifetime params
    let type_params: Vec<_> = generics
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Lifetime(_)))
        .collect();
    if type_params.is_empty() {
        return;
    }
    buf.push('[');
    let mut first = true;
    for param in type_params {
        if !first {
            buf.push_str(", ");
        }
        first = false;
        match param {
            GenericParam::Type(t) => {
                buf.push_str(&t.ident.to_string());
                // Filter out lifetime bounds and ?Sized, keep only trait bounds
                let trait_bounds: Vec<_> = t
                    .bounds
                    .iter()
                    .filter_map(|b| {
                        if let TypeParamBound::Trait(tb) = b {
                            // Skip ?Sized
                            if matches!(tb.modifier, TraitBoundModifier::Maybe(_)) {
                                return None;
                            }
                            Some(tb)
                        } else {
                            None
                        }
                    })
                    .collect();
                if !trait_bounds.is_empty() {
                    buf.push_str(" : ");
                    let mut first_bound = true;
                    for tb in trait_bounds {
                        if !first_bound {
                            buf.push_str(" + ");
                        }
                        first_bound = false;
                        print_path(buf, &tb.path);
                    }
                }
            }
            GenericParam::Lifetime(_) => {} // filtered above, shouldn't reach here
            GenericParam::Const(_) => buf.push_str("/* const generic */"),
        }
    }
    buf.push(']');
}

/// Extract where clause Fn/FnMut/FnOnce bounds as a map from type param name to closure type string.
fn extract_where_fn_types(generics: &Generics) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if let Some(where_clause) = &generics.where_clause {
        for pred in &where_clause.predicates {
            if let WherePredicate::Type(pt) = pred {
                let param_name = pt.bounded_ty.to_token_stream().to_string();
                for bound in &pt.bounds {
                    if let TypeParamBound::Trait(tb) = bound {
                        let trait_name = tb
                            .path
                            .segments
                            .last()
                            .map(|s| s.ident.to_string())
                            .unwrap_or_default();
                        if matches!(trait_name.as_str(), "Fn" | "FnMut" | "FnOnce") {
                            // Extract (Args) -> Ret from the parenthesized arguments
                            if let Some(seg) = tb.path.segments.last() {
                                if let PathArguments::Parenthesized(paren) = &seg.arguments {
                                    let mut fn_type = String::new();
                                    fn_type.push('(');
                                    let mut first = true;
                                    for input in &paren.inputs {
                                        if !first {
                                            fn_type.push_str(", ");
                                        }
                                        first = false;
                                        let mut tbuf = String::new();
                                        print_type(&mut tbuf, input);
                                        fn_type.push_str(&tbuf);
                                    }
                                    fn_type.push_str(") -> ");
                                    match &paren.output {
                                        ReturnType::Default => fn_type.push_str("Unit"),
                                        ReturnType::Type(_, ty) => {
                                            let mut tbuf = String::new();
                                            print_type(&mut tbuf, ty);
                                            fn_type.push_str(&tbuf);
                                        }
                                    }
                                    map.insert(param_name.clone(), fn_type);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Also check generic params directly for bounds like F: FnMut(...)
    for param in &generics.params {
        if let GenericParam::Type(t) = param {
            let param_name = t.ident.to_string();
            for bound in &t.bounds {
                if let TypeParamBound::Trait(tb) = bound {
                    let trait_name = tb
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident.to_string())
                        .unwrap_or_default();
                    if matches!(trait_name.as_str(), "Fn" | "FnMut" | "FnOnce") {
                        if let Some(seg) = tb.path.segments.last() {
                            if let PathArguments::Parenthesized(paren) = &seg.arguments {
                                let mut fn_type = String::new();
                                fn_type.push('(');
                                let mut first = true;
                                for input in &paren.inputs {
                                    if !first {
                                        fn_type.push_str(", ");
                                    }
                                    first = false;
                                    let mut tbuf = String::new();
                                    print_type(&mut tbuf, input);
                                    fn_type.push_str(&tbuf);
                                }
                                fn_type.push_str(") -> ");
                                match &paren.output {
                                    ReturnType::Default => fn_type.push_str("Unit"),
                                    ReturnType::Type(_, ty) => {
                                        let mut tbuf = String::new();
                                        print_type(&mut tbuf, ty);
                                        fn_type.push_str(&tbuf);
                                    }
                                }
                                map.insert(param_name.clone(), fn_type);
                            }
                        }
                    }
                }
            }
        }
    }
    map
}

thread_local! {
    /// Where clause Fn type mappings for the current function.
    static WHERE_FN_TYPES: RefCell<std::collections::HashMap<String, String>> =
        RefCell::new(std::collections::HashMap::new());
}

fn set_where_fn_types(map: std::collections::HashMap<String, String>) {
    WHERE_FN_TYPES.with(|m| *m.borrow_mut() = map);
}

fn get_where_fn_type(name: &str) -> Option<String> {
    WHERE_FN_TYPES.with(|m| m.borrow().get(name).cloned())
}

fn print_fn_params(buf: &mut String, inputs: &punctuated::Punctuated<FnArg, token::Comma>) {
    let mut first = true;
    for arg in inputs {
        if !first {
            buf.push_str(", ");
        }
        first = false;
        match arg {
            FnArg::Receiver(_) => {
                let self_ty = get_self_type();
                if !self_ty.is_empty() {
                    buf.push_str("self : ");
                    buf.push_str(&self_ty);
                } else {
                    buf.push_str("self");
                }
            }
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
        ReturnType::Default => {
            buf.push_str(" -> Unit");
        }
        ReturnType::Type(_, ty) => {
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
                // Resolve Self to the actual type name from enclosing impl block
                let ident_str = if ident_str == "Self" {
                    let self_ty = get_self_type();
                    if !self_ty.is_empty() { self_ty } else { ident_str }
                } else {
                    ident_str
                };
                // Check if this is a where-clause Fn type parameter (F: FnMut(...) -> ...)
                if let Some(fn_type) = get_where_fn_type(&ident_str) {
                    buf.push_str(&fn_type);
                    return;
                }
                let name = mapping::lookup_type(&ident_str);
                if name.is_empty() {
                    // Wrapper type (Box, Rc, Arc, etc.) → unwrap to inner type
                    // Record for documentation comment
                    if mapping::is_wrapper_type(&ident_str) {
                        record_wrapper_type(&ident_str);
                    }
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        // Find first type argument, skip lifetimes
                        for arg in &args.args {
                            if let GenericArgument::Type(t) = arg {
                                print_type(buf, t);
                                return;
                            }
                        }
                    }
                    // Fallback: print as-is
                    buf.push_str(&ident_str);
                } else {
                    buf.push_str(name);
                    if let PathArguments::AngleBracketed(args) = &seg.arguments {
                        // Collect only type args, skip lifetime args
                        let type_args: Vec<_> = args
                            .args
                            .iter()
                            .filter_map(|a| {
                                if let GenericArgument::Type(t) = a {
                                    Some(t)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if !type_args.is_empty() {
                            buf.push('[');
                            let mut first = true;
                            for t in type_args {
                                if !first {
                                    buf.push_str(", ");
                                }
                                first = false;
                                print_type(buf, t);
                            }
                            buf.push(']');
                        }
                    }
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
            // Drop reference (&T, &mut T), MoonBit is GC'd
            print_type(buf, &r.elem);
        }
        Type::Ptr(p) => {
            // Drop raw pointer (*const T, *mut T), MoonBit is GC'd
            print_type(buf, &p.elem);
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
        Type::TraitObject(t) => {
            // dyn Trait → just use the trait name
            if let Some(TypeParamBound::Trait(tb)) = t.bounds.first() {
                print_path(buf, &tb.path);
            } else {
                buf.push_str("_");
            }
        }
        Type::ImplTrait(t) => {
            // impl Trait → just use the trait name
            if let Some(TypeParamBound::Trait(tb)) = t.bounds.first() {
                print_path(buf, &tb.path);
            } else {
                buf.push_str("_");
            }
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
    let derive_str = extract_derive_str(&s.attrs);
    indent(buf, level);
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
    buf.push_str(&derive_str);
}

fn print_enum(buf: &mut String, e: &ItemEnum, level: usize) {
    let derive_str = extract_derive_str(&e.attrs);
    indent(buf, level);
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
    buf.push_str(&derive_str);
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

fn print_static(buf: &mut String, s: &ItemStatic, level: usize) {
    indent(buf, level);
    print_visibility(buf, &s.vis);
    if matches!(s.mutability, StaticMutability::Mut(_)) {
        buf.push_str("let mut ");
    } else {
        buf.push_str("const ");
    }
    buf.push_str(&s.ident.to_string());
    buf.push_str(" : ");
    print_type(buf, &s.ty);
    buf.push_str(" = ");
    print_expr(buf, &s.expr, level);
}

fn print_impl(buf: &mut String, i: &ItemImpl, level: usize) {
    indent(buf, level);
    // Set Self type for resolution within the impl block
    if let Type::Path(tp) = i.self_ty.as_ref() {
        if let Some(seg) = tp.path.segments.last() {
            set_self_type(&seg.ident.to_string());
        }
    }
    if let Some((_, path, _)) = &i.trait_ {
        let trait_name = path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();

        // Skip iterator-related trait impls (MoonBit Iter handles this automatically)
        if matches!(trait_name.as_str(),
            "Iterator" | "DoubleEndedIterator" | "ExactSizeIterator"
        ) {
            // Convert Iterator impl to iter() method
            // impl Iterator for X { fn next(&mut self) -> Option<T> { ... } }
            // → fn X::iter(self : X) -> Iter[T] { ... }  (comment only for now)
            buf.push_str("// Iterator impl converted: use .iter() method or for-in loop\n");
            clear_self_type();
            return;
        }

        // Skip IntoIterator (MoonBit collections already support for-in)
        if trait_name == "IntoIterator" {
            buf.push_str("// IntoIterator: MoonBit collections support for-in natively\n");
            clear_self_type();
            return;
        }

        // Skip Clone/Copy impls (GC'd, not needed)
        if matches!(trait_name.as_str(), "Clone" | "Copy") {
            clear_self_type();
            return;
        }

        // Skip From/Into when trivial
        if matches!(trait_name.as_str(), "From" | "Into") {
            // Still emit these as they may be useful
        }

        // Skip Extend/FromIterator (MoonBit has different collection building patterns)
        if matches!(trait_name.as_str(), "Extend" | "FromIterator") {
            buf.push_str("// Collection builder trait: use MoonBit Array methods directly\n");
            clear_self_type();
            return;
        }

        // Skip Index/IndexMut (MoonBit uses op_get/op_set)
        if matches!(trait_name.as_str(), "Index" | "IndexMut") {
            buf.push_str("// Index: MoonBit uses op_get/op_set for [] syntax\n");
            clear_self_type();
            return;
        }

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
    } else {
        // inherent impl: impl<T> Type<T> { methods } → fn Type::method(self, ...) { ... }
        for item in &i.items {
            if let ImplItem::Fn(method) = item {
                // Set where clause fn types for this method
                set_where_fn_types(extract_where_fn_types(&method.sig.generics));
                print_visibility(buf, &method.vis);
                buf.push_str("fn ");
                // Print type name without generic params for method definitions
                if let Type::Path(tp) = i.self_ty.as_ref() {
                    if let Some(seg) = tp.path.segments.last() {
                        buf.push_str(&seg.ident.to_string());
                    }
                } else {
                    print_type(buf, &i.self_ty);
                }
                buf.push_str("::");
                buf.push_str(&method.sig.ident.to_string());
                buf.push('(');
                print_fn_params(buf, &method.sig.inputs);
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
    clear_self_type();
}

/// Extract derive string from attributes. Returns e.g. " derive(Show, Eq)" or empty.
fn extract_derive_str(attrs: &[Attribute]) -> String {
    for attr in attrs {
        if attr.path().is_ident("derive") {
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
                return format!(" derive({})", derives.join(", "));
            }
        }
    }
    String::new()
}

fn print_path(buf: &mut String, path: &Path) {
    let mut first = true;
    for seg in &path.segments {
        if !first {
            buf.push_str("::");
        }
        first = false;
        let seg_str = seg.ident.to_string();
        // Resolve Self
        let seg_str = if seg_str == "Self" {
            let self_ty = get_self_type();
            if !self_ty.is_empty() {
                self_ty
            } else {
                seg_str
            }
        } else {
            seg_str
        };
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
            // Check if this is a wrapper constructor: Box::new(x) → x
            let call_path = {
                let mut p = String::new();
                print_expr(&mut p, &c.func, level);
                p
            };
            if mapping::is_string_constructor(&call_path) && c.args.len() == 1 {
                // String::from("lit") → "lit"
                print_expr(buf, &c.args[0], level);
            } else if let Some(literal) = mapping::lookup_constructor(&call_path) {
                // Type constructor → literal: Vec::new() → [], String::new() → ""
                buf.push_str(literal);
            } else if mapping::is_wrapper_constructor(&call_path) && c.args.len() == 1 {
                // Unwrap: Box::new(x) → x
                print_expr(buf, &c.args[0], level);
            } else if call_path == "::new" || call_path.ends_with("::::new") {
                // Broken path from empty type mapping: just print args
                if c.args.len() == 1 {
                    print_expr(buf, &c.args[0], level);
                } else {
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
            } else {
                buf.push_str(&call_path);
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
        }
        Expr::MethodCall(m) => {
            let method_str = m.method.to_string();
            let method = mapping::lookup_method(&method_str);
            if method.is_empty() {
                // Identity method (borrow, as_ref, deref) → just print receiver
                print_expr(buf, &m.receiver, level);
            } else {
                print_expr(buf, &m.receiver, level);
                buf.push('.');
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
        }
        Expr::If(i) => {
            // if let Some(x) = expr → if expr is Some(x) { ... } else { ... }
            if let Expr::Let(let_expr) = i.cond.as_ref() {
                buf.push_str("if ");
                print_expr(buf, &let_expr.expr, level);
                buf.push_str(" is ");
                print_pat(buf, &let_expr.pat, level);
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
                // If arm body is a block, print with braces
                match &*arm.body {
                    Expr::Block(b) if b.block.stmts.len() > 1 => {
                        buf.push_str("{\n");
                        print_block_body(buf, &b.block, level + 2);
                        buf.push('\n');
                        indent(buf, level + 1);
                        buf.push('}');
                    }
                    _ => print_expr(buf, &arm.body, level + 1),
                }
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
            // while let Some(x) = iter.next() → MoonBit has no while let, desugar to loop+match
            if let Expr::Let(let_expr) = w.cond.as_ref() {
                buf.push_str("// while let → loop+match\n");
                indent(buf, level);
                buf.push_str("while true {\n");
                indent(buf, level + 1);
                buf.push_str("match ");
                print_expr(buf, &let_expr.expr, level + 1);
                buf.push_str(" {\n");
                indent(buf, level + 2);
                print_pat(buf, &let_expr.pat, level);
                buf.push_str(" => {\n");
                print_block_body(buf, &w.body, level + 3);
                buf.push('\n');
                indent(buf, level + 2);
                buf.push('}');
                buf.push('\n');
                indent(buf, level + 2);
                buf.push_str("_ => break\n");
                indent(buf, level + 1);
                buf.push_str("}\n");
                indent(buf, level);
                buf.push('}');
            } else {
                buf.push_str("while ");
                print_expr(buf, &w.cond, level);
                buf.push_str(" {\n");
                print_block_body(buf, &w.body, level + 1);
                buf.push('\n');
                indent(buf, level);
                buf.push('}');
            }
        }
        Expr::ForLoop(f) => {
            // MoonBit for-each doesn't support tuple destructure directly
            // for (a, b) in v → for _item in v { let (a, b) = _item; ... }
            let is_complex_pat = matches!(&*f.pat, Pat::Tuple(_) | Pat::Struct(_) | Pat::TupleStruct(_));
            if is_complex_pat {
                buf.push_str("for _item in ");
                print_expr(buf, &f.expr, level);
                buf.push_str(" {\n");
                indent(buf, level + 1);
                buf.push_str("let ");
                print_pat(buf, &f.pat, level);
                buf.push_str(" = _item\n");
                print_block_body(buf, &f.body, level + 1);
            } else {
                buf.push_str("for ");
                print_pat(buf, &f.pat, level);
                buf.push_str(" in ");
                print_expr(buf, &f.expr, level);
                buf.push_str(" {\n");
                print_block_body(buf, &f.body, level + 1);
            }
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
            buf.push(')');
            print_return_type(buf, &c.output);
            buf.push_str(" { ");
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
        Expr::Await(a) => {
            // expr.await → expr (MoonBit async doesn't need await/suffix)
            print_expr(buf, &a.base, level);
        }
        Expr::Async(a) => {
            // async { block } → async { block }
            buf.push_str("async {\n");
            print_block_body(buf, &a.block, level + 1);
            buf.push('\n');
            indent(buf, level);
            buf.push('}');
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

/// Try to parse macro tokens as comma-separated expressions and print them nicely.
fn print_macro_args(buf: &mut String, tokens: &proc_macro2::TokenStream, level: usize) {
    // Try to parse as comma-separated expressions (tuple-like)
    if let Ok(args) = syn::parse2::<syn::ExprTuple>(quote::quote!( (#tokens) )) {
        let mut first = true;
        for arg in &args.elems {
            if !first {
                buf.push_str(", ");
            }
            first = false;
            print_expr(buf, arg, level);
        }
    } else if let Ok(expr) = syn::parse2::<Expr>(tokens.clone()) {
        // Single expression (e.g., assert!(expr))
        print_expr(buf, &expr, level);
    } else {
        // Fallback: raw token string
        buf.push_str(&tokens.to_string());
    }
}

fn print_expr_macro(buf: &mut String, mac: &ExprMacro, level: usize) {
    let name = mac.mac.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
    if let Some(mbt_name) = mapping::lookup_macro(&name) {
        buf.push_str(mbt_name);
        buf.push('(');
        print_macro_args(buf, &mac.mac.tokens, level);
        buf.push(')');
    } else if name == "vec" {
        buf.push('[');
        print_macro_args(buf, &mac.mac.tokens, level);
        buf.push(']');
    } else if name == "format" {
        // format!("{} is {}", name, age) → "\{name} is \{age}"
        let tokens = &mac.mac.tokens;
        if let Ok(args) = syn::parse2::<syn::ExprTuple>(quote::quote!( (#tokens) )) {
            let elems: Vec<_> = args.elems.iter().collect();
            if let Some(Expr::Lit(ExprLit { lit: Lit::Str(fmt_str), .. })) = elems.first() {
                let fmt = fmt_str.value();
                let arg_exprs = &elems[1..];
                let mut arg_idx = 0;
                buf.push('"');
                let mut chars = fmt.chars().peekable();
                while let Some(ch) = chars.next() {
                    if ch == '{' && chars.peek() == Some(&'}') {
                        chars.next();
                        buf.push_str("\\{");
                        if arg_idx < arg_exprs.len() {
                            print_expr(buf, arg_exprs[arg_idx], level);
                            arg_idx += 1;
                        }
                        buf.push('}');
                    } else {
                        buf.push(ch);
                    }
                }
                buf.push('"');
            } else {
                buf.push_str("\"\""); // fallback
            }
        } else if let Ok(expr) = syn::parse2::<Expr>(tokens.clone()) {
            // format!("literal") with no args
            print_expr(buf, &expr, level);
        } else {
            buf.push_str("\"\"");
        }
    } else if name == "matches" {
        // matches!(expr, pattern) → expr is pattern
        let tokens = mac.mac.tokens.to_string();
        // Split on first comma
        if let Some(idx) = tokens.find(',') {
            let expr_part = tokens[..idx].trim();
            let pat_part = tokens[idx + 1..].trim();
            buf.push_str(expr_part);
            buf.push_str(" is ");
            buf.push_str(pat_part);
        } else {
            buf.push_str(&tokens);
        }
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

fn print_macro_stmt(buf: &mut String, mac: &StmtMacro, level: usize) {
    let name = mac.mac.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
    if let Some(mbt_name) = mapping::lookup_macro(&name) {
        buf.push_str(mbt_name);
        buf.push('(');
        print_macro_args(buf, &mac.mac.tokens, level);
        buf.push(')');
    } else {
        buf.push_str(&name);
        buf.push('(');
        print_macro_args(buf, &mac.mac.tokens, level);
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
        Pat::Path(p) => {
            print_path(buf, &p.path);
        }
        Pat::Rest(_) => buf.push_str(".."),
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

use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let report_mode = args.iter().any(|a| a == "--report");
    let file_arg = args.iter().skip(1).find(|a| !a.starts_with('-'));

    let input = if let Some(path) = file_arg {
        std::fs::read_to_string(path).expect("Failed to read input file")
    } else {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .expect("Failed to read stdin");
        buf
    };

    let output = rs2mbt::to_moonbit(&input);

    if report_mode {
        print_quality_report(&input, &output);
    } else {
        print!("{}", output);
    }
}

fn print_quality_report(rust_src: &str, mbt_output: &str) {
    let rust_lines: Vec<&str> = rust_src.lines().filter(|l| !l.trim().is_empty()).collect();
    let mbt_lines: Vec<&str> = mbt_output.lines().filter(|l| !l.trim().is_empty()).collect();

    let todo_lines: Vec<&&str> = mbt_lines
        .iter()
        .filter(|l| l.contains("TODO(transpile)"))
        .collect();
    let note_lines: Vec<&&str> = mbt_lines
        .iter()
        .filter(|l| l.contains("NOTE:") || l.contains("// was unsafe"))
        .collect();
    let unsupported_lines: Vec<&&str> = mbt_lines
        .iter()
        .filter(|l| l.contains("unsupported"))
        .collect();

    // Count Rust items
    let rust_fns = rust_src.matches("\nfn ").count() + if rust_src.starts_with("fn ") { 1 } else { 0 };
    let rust_structs = rust_src.matches("\nstruct ").count()
        + rust_src.matches("\npub struct ").count();
    let rust_enums =
        rust_src.matches("\nenum ").count() + rust_src.matches("\npub enum ").count();
    let rust_impls = rust_src.matches("\nimpl ").count();
    let rust_traits = rust_src.matches("\ntrait ").count();
    let rust_tests = rust_src.matches("#[test]").count();

    // Count MoonBit items
    let mbt_fns = mbt_output.matches("\nfn ").count()
        + mbt_output.matches("\npub fn ").count()
        + mbt_output.matches("\nasync fn ").count()
        + if mbt_output.starts_with("fn ") || mbt_output.starts_with("pub fn ") || mbt_output.starts_with("async fn ") { 1 } else { 0 };
    let mbt_tests = mbt_output.matches("\ntest ").count();

    let converted = mbt_lines.len() - todo_lines.len() - note_lines.len() - unsupported_lines.len();
    let total = mbt_lines.len();
    let pct = if total > 0 {
        (converted as f64 / total as f64 * 100.0) as i32
    } else {
        100
    };

    println!("=== Conversion Quality Report ===");
    println!();
    println!("Input (Rust):");
    println!("  Lines (non-empty): {}", rust_lines.len());
    println!("  Functions:         {}", rust_fns);
    println!("  Structs:           {}", rust_structs);
    println!("  Enums:             {}", rust_enums);
    println!("  Impl blocks:       {}", rust_impls);
    println!("  Traits:            {}", rust_traits);
    println!("  Tests:             {}", rust_tests);
    println!();
    println!("Output (MoonBit):");
    println!("  Lines (non-empty): {}", mbt_lines.len());
    println!("  Functions:         {}", mbt_fns);
    println!("  Test blocks:       {}", mbt_tests);
    println!();
    println!("Conversion:");
    println!("  Converted lines:   {} / {} ({}%)", converted, total, pct);
    println!("  TODO comments:     {}", todo_lines.len());
    println!("  NOTE comments:     {}", note_lines.len());
    println!("  Unsupported:       {}", unsupported_lines.len());
    println!();

    if !todo_lines.is_empty() || !unsupported_lines.is_empty() {
        println!("Manual fixes needed:");
        for line in &todo_lines {
            println!("  {}", line.trim());
        }
        for line in &unsupported_lines {
            println!("  {}", line.trim());
        }
        println!();
    }

    // Output the converted code
    println!("--- Generated MoonBit ---");
    print!("{}", mbt_output);
}

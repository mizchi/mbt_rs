# mbt_rs: MoonBit ↔ Rust transpiler

target := "js"

# Run all checks
default: check test

# MoonBit: type check
check:
    moon check --deny-warn --target {{target}}

# MoonBit: run tests
test:
    moon test --target {{target}}

# MoonBit: format
fmt:
    moon fmt

# MoonBit: format check
fmt-check:
    moon fmt --check

# MoonBit: generate mbti
info:
    moon info

# MoonBit: verify mbti
info-check:
    moon info
    git diff --exit-code -- ':(glob)**/*.generated.mbti'

# Rust: run rs2mbt tests
cargo-test:
    cd rs2mbt && cargo test

# Rust: check rs2mbt
cargo-check:
    cd rs2mbt && cargo check

# Verify rs2mbt output passes moon check + moon test
moon-check-rs2mbt:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT
    # Setup MoonBit project
    echo '{"name":"test_rs2mbt","version":"0.1.0"}' > "$tmpdir/moon.mod.json"
    mkdir -p "$tmpdir/src"
    touch "$tmpdir/src/moon.pkg"
    # Generate MoonBit from Rust fixtures + append test blocks
    "$root/rs2mbt/target/debug/rs2mbt" "$root/fixtures/input.rs" > "$tmpdir/src/generated.mbt"
    echo "" >> "$tmpdir/src/generated.mbt"
    cat "$root/fixtures/generated_test.mbt" >> "$tmpdir/src/generated.mbt"
    # Run moon check and test
    cd "$tmpdir"
    moon check
    moon test --target js

# Behavioral equivalence: run same assertions in Rust and MoonBit
behavioral-test:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    echo "=== Rust behavioral tests ==="
    cargo test --manifest-path "$root/fixtures/rust_verify/Cargo.toml" --lib -q
    echo "=== MoonBit behavioral tests (via rs2mbt) ==="
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT
    echo '{"name":"test_rs2mbt","version":"0.1.0"}' > "$tmpdir/moon.mod.json"
    mkdir -p "$tmpdir/src"
    touch "$tmpdir/src/moon.pkg"
    "$root/rs2mbt/target/debug/rs2mbt" "$root/fixtures/input.rs" > "$tmpdir/src/generated.mbt"
    echo "" >> "$tmpdir/src/generated.mbt"
    cat "$root/fixtures/generated_test.mbt" >> "$tmpdir/src/generated.mbt"
    cd "$tmpdir"
    moon check
    moon test --target js
    echo "=== Both pass: behavioral equivalence confirmed ==="

# Convert a Rust file and apply moon fmt
convert file:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    tmpdir=$(mktemp -d)
    trap 'rm -rf "$tmpdir"' EXIT
    echo '{"name":"t","version":"0.1.0"}' > "$tmpdir/moon.mod.json"
    mkdir -p "$tmpdir/src"
    touch "$tmpdir/src/moon.pkg"
    "$root/rs2mbt/target/debug/rs2mbt" "{{file}}" > "$tmpdir/src/out.mbt"
    cd "$tmpdir" && moon fmt 2>/dev/null
    cat "$tmpdir/src/out.mbt"

# Conversion quality report for a Rust file
quality-report file:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    "$root/rs2mbt/target/debug/rs2mbt" --report "{{file}}"

# Quality report for all real projects
quality-report-all:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    for f in "$root"/fixtures/real_projects/*.rs; do
        echo ""
        echo "================================================================"
        echo "File: $(basename $f)"
        echo "================================================================"
        "$root/rs2mbt/target/debug/rs2mbt" --report "$f" 2>&1 | head -25
    done

# Convert a Rust crate with macro expansion (requires cargo-expand)
# Usage: just expand-and-convert path/to/rust/crate
expand-and-convert crate_path:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    echo "Expanding macros in {{crate_path}}..."
    expanded=$(cd "{{crate_path}}" && cargo expand 2>/dev/null)
    echo "$expanded" | "$root/rs2mbt/target/debug/rs2mbt"

# Convert with macro expansion + quality report
expand-and-report crate_path:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    echo "Expanding macros in {{crate_path}}..."
    expanded=$(cd "{{crate_path}}" && cargo expand 2>/dev/null)
    echo "$expanded" | "$root/rs2mbt/target/debug/rs2mbt" --report

# Generate MoonBit from real Rust projects (for inspection, not CI)
generate-real-projects:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{justfile_directory()}}"
    cargo build -q --manifest-path "$root/rs2mbt/Cargo.toml"
    for f in "$root"/fixtures/real_projects/*.rs; do
        base=$(basename "$f" .rs)
        echo "Generating $base.mbt..."
        "$root/rs2mbt/target/debug/rs2mbt" "$f" > "$root/fixtures/real_projects/${base}.mbt"
    done
    echo "Done. Check fixtures/real_projects/*.mbt"

# Run all tests (MoonBit + Rust + behavioral equivalence)
test-all: test cargo-test behavioral-test

# CI checks
ci: fmt-check info-check check test cargo-test behavioral-test

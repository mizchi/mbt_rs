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

# Run all tests (MoonBit + Rust + behavioral equivalence)
test-all: test cargo-test behavioral-test

# CI checks
ci: fmt-check info-check check test cargo-test behavioral-test

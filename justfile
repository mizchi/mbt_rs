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

# Run all tests (MoonBit + Rust)
test-all: test cargo-test

# CI checks
ci: fmt-check info-check check test cargo-test

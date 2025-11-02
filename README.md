# test-e0109

Minimal reproduction for rust-analyzer false positive E0109 error.

## Issue

rust-analyzer version 0.3.2660-standalone (7c810e9994 2025-10-27) reports E0109 "type arguments are not allowed on this type" on valid Rust code that compiles successfully.

## The Problem

When using a macro to generate trait implementations with const generics, rust-analyzer incorrectly flags the macro invocations with E0109 errors, even though:
- The code compiles with `cargo check`
- The code runs correctly with `cargo run`
- The expanded macro produces valid Rust code

The compiler seems to evade the problem by pre-parsing the tokens of the macro, while direct string substitution looks like we are treating the primitive type as a generic.

## Reproduction

```bash
cargo +1.77 check  # ✅ Compiles successfully
cargo +1.77 run    # ✅ Runs and passes all assertions
```

However, opening `src/main.rs` in VSCode with rust-analyzer shows 3 E0109 errors on the 3 lines 42-44.

## Environment

- **Rust:** 1.77 (edition 2018)
- **rust-analyzer:** 0.3.2660-standalone (7c810e9994 2025-10-27)
- **Editor:** Visual Studio Code
- **Extension:** rust-lang.rust-analyzer-0.3.2660-darwin-arm64

## Related Issue

This repository serves as a minimal reproduction for rust-analyzer issue [#20958](https://github.com/rust-lang/rust-analyzer/issues/20958).

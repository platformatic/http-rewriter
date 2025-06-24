# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Build & Test
- **Build**: `cargo build`
- **Test**: `cargo test` - Runs all unit tests and doc tests
- **Check**: `cargo check` - Quick compilation check
- **Lint**: `cargo clippy` - Enforces Rust best practices (zero warnings expected)
- **Run single test**: `cargo test test_name` or `cargo test module::test_name`
- **Doc tests only**: `cargo test --doc`

## Architecture Overview

This is a Rust library for HTTP request rewriting designed to work with the `http` crate. The architecture follows a trait-based, composable design pattern.

### Core Traits
1. **`Condition`**: Matches HTTP requests based on criteria (path, method, headers)
2. **`Rewriter`**: Transforms HTTP requests (modifies path, method, headers, href)

### Key Design Patterns
- **Composability**: Conditions combine with `and()`/`or()`, rewriters chain with `then()`
- **Conditional Application**: Use `when()` to apply rewriters only when conditions match
- **Type Safety**: Generic over request body type `<B>`, uses `Result` for error handling
- **Extension Traits**: `ConditionExt` and `RewriterExt` provide fluent API

### Module Structure
- `lib.rs` - Main entry point with re-exports and documentation
- `condition.rs` - Request matching logic (PathCondition, MethodCondition, etc.)
- `rewriter.rs` - Request transformation logic (PathRewriter, HeaderRewriter, etc.)
- `conditional_rewriter.rs` - Combines conditions with rewriters
- `document_root.rs` - Filesystem-based conditions
- `integration_tests.rs` - Real-world usage examples

### Important Implementation Details
- All public types implement both `Condition` and `Rewriter` traits where sensible
- Uses regex for pattern matching in path/header transformations
- Thread-safe (all traits require `Send + Sync`)
- No unsafe code except well-documented transmutations for closure implementations
- Comprehensive error handling with custom `RewriteError` type
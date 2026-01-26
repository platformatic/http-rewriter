# http-rewriter Source Code Architecture

This document provides a comprehensive overview of the `http-rewriter` codebase to help developers understand the architecture, design decisions, and implementation details.

## Table of Contents

1. [Overview](#overview)
2. [Core Architecture](#core-architecture)
3. [Module Structure](#module-structure)
4. [Design Patterns](#design-patterns)
5. [Key Components](#key-components)
6. [How HTTP Rewriting Works](#how-http-rewriting-works)
7. [Implementation Details](#implementation-details)
8. [Gotchas and Edge Cases](#gotchas-and-edge-cases)

## Overview

`http-rewriter` is a Rust library for transforming HTTP requests in a composable, type-safe manner. It integrates with the `http` crate and provides a flexible framework for:

- Matching requests based on criteria (path, method, headers, file existence)
- Transforming request properties (path, method, headers, URI)
- Composing complex rewrite rules using combinators
- Applying transformations conditionally

The library is designed to be:
- **Generic**: Works with any request body type (streaming, bytes, etc.)
- **Composable**: Conditions and rewriters can be combined and chained
- **Type-safe**: Leverages Rust's type system for correctness
- **Thread-safe**: All traits require `Send + Sync`
- **Ergonomic**: Fluent API with extension traits

## Core Architecture

The architecture is built around two main traits:

### 1. `Condition` Trait

```rust
pub trait Condition: Send + Sync {
    fn matches<B>(&self, request: &Request<B>) -> bool;
}
```

Conditions **match** requests based on criteria. They are:
- Generic over request body type `<B>`
- Immutable (take `&self` and `&Request<B>`)
- Return a boolean indicating match/no-match

### 2. `Rewriter` Trait

```rust
pub trait Rewriter: Send + Sync {
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError>;
}
```

Rewriters **transform** requests by consuming and returning them. They:
- Take ownership of the request
- Return a `Result` for error handling
- Preserve the request body while transforming metadata

### 3. Extension Traits

Two extension traits provide the fluent API:

- **`ConditionExt`**: Provides `and()` and `or()` for combining conditions
- **`RewriterExt`**: Provides `then()` for chaining and `when()` for conditional application

## Module Structure

### `lib.rs` (5.5 KB)
The main entry point containing:
- Module declarations and re-exports
- Top-level documentation with examples
- Public API surface

### `condition.rs` (24 KB)
Request matching logic including:
- `PathCondition`: Regex-based path matching
- `MethodCondition`: HTTP method matching
- `HeaderCondition`: Header value pattern matching
- `ExistenceCondition`: File existence checks
- `NonExistenceCondition`: File non-existence checks
- `GroupCondition<A, B>`: Logical AND/OR combinations
- `ConditionExt`: Extension trait for `and()` and `or()`
- Closure-based condition implementation

### `rewriter.rs` (28 KB)
Request transformation logic including:
- `PathRewriter`: Regex-based path transformation
- `MethodRewriter`: HTTP method changes
- `HeaderRewriter`: Header value transformation
- `HrefRewriter`: Path and query transformation
- `SequenceRewriter<R1, R2>`: Sequential composition
- `RewriterExt`: Extension trait for `then()` and `when()`
- `RewriteError`: Custom error type
- Closure-based rewriter implementation

### `conditional_rewriter.rs` (5 KB)
Combines conditions with rewriters:
- `ConditionalRewriter<R, C>`: Applies rewriter only when condition matches
- Created via `RewriterExt::when(condition)`

### `integration_tests.rs` (6 KB)
Real-world usage examples and tests covering:
- Fluent API usage
- Chained rewriters
- Complex condition combinations
- Closure-based conditions and rewriters
- File existence conditions with document roots
- Body preservation through transformations

### `napi.rs` (82 KB)
Node.js N-API bindings (optional, enabled via `napi-support` feature):
- Exposes all condition and rewriter types to JavaScript
- Provides type conversions between Rust and Node.js
- Allows usage from Node.js/TypeScript applications

## Design Patterns

### 1. Trait-Based Polymorphism

The core design uses traits to define behavior, allowing different implementations to be composed together. This enables:

```rust
// All of these implement Condition
let path_cond = PathCondition::new("^/api/.*").unwrap();
let method_cond = MethodCondition::new(Method::POST).unwrap();
let closure_cond = |req: &Request<()>| req.uri().path().len() > 10;

// They can all be combined
let combined = path_cond.and(method_cond).and(closure_cond);
```

### 2. Generic Programming

The library is generic over the request body type, making it compatible with any body representation:

```rust
// Works with ()
let req1: Request<()> = Request::builder().body(()).unwrap();

// Works with Bytes
let req2: Request<Bytes> = Request::builder().body(Bytes::from("data")).unwrap();

// Works with streaming bodies
let req3: Request<impl Stream> = Request::builder().body(stream).unwrap();
```

### 3. Builder Pattern via Extension Traits

Extension traits provide a fluent, chainable API:

```rust
PathRewriter::new("^/old/", "/new/").unwrap()
    .when(MethodCondition::new(Method::POST).unwrap())
    .then(HeaderRewriter::new("X-Version", ".*", "2.0").unwrap());
```

### 4. Combinator Pattern

Conditions and rewriters can be composed using combinators:

- **Conditions**: `and()`, `or()` create `GroupCondition<A, B>`
- **Rewriters**: `then()` creates `SequenceRewriter<R1, R2>`, `when()` creates `ConditionalRewriter<R, C>`

### 5. Type Erasure with Boxed Trait Objects

Combinators use `Box<dyn Trait>` internally to handle different concrete types:

```rust
pub enum GroupCondition<A, B>
where
    A: Condition + ?Sized,
    B: Condition + ?Sized,
{
    And(Box<A>, Box<B>),
    Or(Box<A>, Box<B>),
}
```

The `?Sized` bound allows the boxed types to be either sized or trait objects.

## Key Components

### Regex-Based Matching and Transformation

Many components use the `regex` crate for pattern matching:

```rust
pub struct PathCondition {
    pattern: Regex,  // Compiled once, reused for all requests
}

pub struct PathRewriter {
    pattern: Regex,
    replacement: String,  // Can include capture groups: $1, $2, etc.
}
```

**Key insight**: Patterns are compiled at construction time, not per-request, for performance.

### Document Root and File System Conditions

`ExistenceCondition` and `NonExistenceCondition` check if files exist:

```rust
impl Condition for ExistenceCondition {
    fn matches<B>(&self, request: &Request<B>) -> bool {
        if let Some(doc_root) = request.document_root() {
            let path = request.uri().path();
            let stripped = path.strip_prefix('/').unwrap_or(path);
            doc_root.join(stripped).exists()
        } else {
            false  // No document root = no match
        }
    }
}
```

**Important**: Document root must be set via request extensions using `RequestExt::set_document_root()` from the `http-handler` crate. Without a document root, these conditions always return `false`.

### Request Parts Manipulation

All rewriters follow the same pattern:

1. Decompose request into parts and body: `request.into_parts()`
2. Modify the parts (URI, method, headers, etc.)
3. Reconstruct the request: `Request::from_parts(parts, body)`

This ensures the body is preserved unchanged through transformations.

### Error Handling

Custom error type for rewrite operations:

```rust
pub struct RewriteError(String);

impl std::error::Error for RewriteError {}
impl std::fmt::Display for RewriteError { /* ... */ }
```

Errors occur when:
- Invalid URIs are produced after path rewriting
- Invalid header names/values are created
- Regex replacement produces malformed output

## How HTTP Rewriting Works

### Request Flow Through a Rewriter

```
Request<B> → Rewriter → Result<Request<B>, RewriteError>
```

1. **Input**: Request with some body type `B`
2. **Decomposition**: `into_parts()` separates metadata from body
3. **Transformation**: Metadata (URI, method, headers) is modified
4. **Reconstruction**: `from_parts()` creates new request with original body
5. **Output**: Transformed request or error

### Path Rewriting Example

```rust
PathRewriter::new("^/api/v1/(.*)$", "/api/v2/$1")
```

Given request: `GET /api/v1/users?page=2`

1. Extract path: `/api/v1/users`
2. Apply regex: `^/api/v1/(.*)$` matches, capture group 1 = `users`
3. Replace: `/api/v2/$1` becomes `/api/v2/users`
4. Preserve query: Rebuild URI as `/api/v2/users?page=2`
5. Preserve scheme/authority if present: `https://example.com/api/v2/users?page=2`

### PathRewriter vs HrefRewriter

Two rewriters handle URIs differently:

- **`PathRewriter`**: Matches and replaces only the **path** component
  - Preserves query string separately
  - Good for simple path transformations

- **`HrefRewriter`**: Matches and replaces **path and query together**
  - Pattern sees `/path?query=value` as a single string
  - Good for adding query parameters or complex rewrites
  - Example: `^(.*)$` → `/index.php?route=$1` captures entire path+query

### Conditional Rewriting

```rust
rewriter.when(condition)
```

Creates a `ConditionalRewriter<R, C>` that:

1. Checks if condition matches the request
2. If yes: applies the rewriter's transformation
3. If no: returns the request unchanged

This is the key to selective transformations.

### Chaining Rewriters

```rust
rewriter1.then(rewriter2)
```

Creates a `SequenceRewriter<R1, R2>` that:

1. Applies `rewriter1` to the request
2. If successful, applies `rewriter2` to the result
3. If either fails, propagates the error

### Combining Conditions

```rust
condition1.and(condition2)  // GroupCondition::And
condition1.or(condition2)   // GroupCondition::Or
```

- **AND**: Both conditions must match (short-circuits on first false)
- **OR**: At least one condition must match (short-circuits on first true)

Can be nested arbitrarily:

```rust
let complex = path_cond
    .and(method_cond.or(other_method_cond))
    .and(header_cond);
```

## Implementation Details

### Closure Support with Unsafe Transmutation

Both `Condition` and `Rewriter` traits are implemented for closures using `unsafe` transmutation:

```rust
impl<F> Condition for F
where
    F: Fn(&Request<()>) -> bool + Send + Sync,
{
    fn matches<B>(&self, request: &Request<B>) -> bool {
        unsafe {
            let request_ref: &Request<()> = std::mem::transmute(request);
            self(request_ref)
        }
    }
}
```

**Why this is safe**:
1. Only reading from the request (immutable borrow)
2. Closure only accesses metadata, never the body
3. Request structure layout is identical regardless of body type
4. We never actually access the body field through the transmuted reference

For rewriters, the body is explicitly separated before passing to the closure:

```rust
impl<F> Rewriter for F
where
    F: Fn(Request<()>) -> Result<Request<()>, RewriteError> + Send + Sync,
{
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        let (parts, body) = request.into_parts();
        let empty_request = Request::from_parts(parts, ());
        let transformed = self(empty_request)?;
        let (new_parts, _) = transformed.into_parts();
        Ok(Request::from_parts(new_parts, body))
    }
}
```

This approach preserves the body while allowing closures to work with `Request<()>`.

### URI Building Strategy

When rebuilding URIs after path transformation, the code preserves context:

```rust
// If original had scheme and authority, preserve them
if let (Some(scheme), Some(authority)) = (parts.uri.scheme(), parts.uri.authority()) {
    format!("{scheme}://{authority}{new_path}?{query}")
} else {
    // Relative URI
    format!("{new_path}?{query}")
}
```

This ensures backward compatibility with both full URLs and relative paths.

### GroupCondition Type Parameters

The `GroupCondition` enum uses `?Sized` bounds:

```rust
pub enum GroupCondition<A, B>
where
    A: Condition + ?Sized,
    B: Condition + ?Sized,
{
    And(Box<A>, Box<B>),
    Or(Box<A>, Box<B>),
}
```

This allows:
- Concrete types: `GroupCondition<PathCondition, MethodCondition>`
- Trait objects: `GroupCondition<dyn Condition, dyn Condition>`
- Mixed: `GroupCondition<PathCondition, dyn Condition>`

The `?Sized` relaxation is necessary because `Box<T>` can contain unsized types.

### Clone and Debug Derivation

Combinators implement `Clone` and `Debug` conditionally:

```rust
impl<A, B> Clone for GroupCondition<A, B>
where
    A: Condition + Clone,
    B: Condition + Clone,
{
    fn clone(&self) -> Self {
        match self {
            GroupCondition::And(a, b) => GroupCondition::And(a.clone(), b.clone()),
            GroupCondition::Or(a, b) => GroupCondition::Or(a.clone(), b.clone()),
        }
    }
}
```

This means `GroupCondition` is only `Clone` if both contained conditions are `Clone`. Same for `Debug`.

### Error Messages

The library uses descriptive error messages:

```rust
RewriteError("Invalid URI after path rewrite".to_string())
RewriteError("Invalid header name".to_string())
RewriteError("Invalid method specified for MethodRewriter".to_string())
```

These help with debugging rewrite failures in production.

## Gotchas and Edge Cases

### 1. Document Root Must Be Set

`ExistenceCondition` and `NonExistenceCondition` require the document root to be set in request extensions:

```rust
use http_handler::RequestExt;
request.set_document_root("/var/www/html".into());
```

Without this, these conditions **always return false**. This is by design to prevent filesystem access without explicit configuration.

### 2. Path Stripping Behavior

File existence conditions strip the leading `/` before joining with document root:

```rust
let stripped = path.strip_prefix('/').unwrap_or(path);
doc_root.join(stripped)
```

So `/index.html` with doc root `/var/www` checks `/var/www/index.html`, not `/var/www//index.html`.

### 3. Header Matching is Case-Insensitive

HTTP headers are case-insensitive per spec, so:

```rust
HeaderCondition::new("Content-Type", "...")
HeaderCondition::new("content-type", "...")
HeaderCondition::new("CONTENT-TYPE", "...")
```

All match the same header.

### 4. Regex Patterns Must Be Valid

Construction of conditions/rewriters can fail if regex is invalid:

```rust
PathCondition::new("[invalid")  // Returns Err(regex::Error)
```

Handle these errors at construction time, not per-request.

### 5. URI Parsing Can Fail

After path rewriting, the new URI must be valid:

```rust
PathRewriter::new(".*", "not a valid uri!!!").unwrap()
```

This will compile but fail at rewrite time with `RewriteError("Invalid URI after path rewrite")`.

### 6. Query String Handling in PathRewriter vs HrefRewriter

- **PathRewriter**: Query is preserved separately, pattern only sees path
  ```
  /api/users?page=2
  Pattern sees: /api/users
  Query preserved: ?page=2
  ```

- **HrefRewriter**: Query is part of the matched string
  ```
  /api/users?page=2
  Pattern sees: /api/users?page=2
  Can transform query in replacement
  ```

Choose based on whether you need to modify query parameters.

### 7. Method Conversion Can Fail

`MethodRewriter` accepts types that implement `TryInto<Method>`:

```rust
MethodRewriter::new(Method::POST)  // Always succeeds (Infallible)
MethodRewriter::new("POST")        // Can fail (InvalidMethod)
MethodRewriter::new("INVALID!!!")  // Will fail
```

Handle construction errors appropriately.

### 8. Closure Conditions Cannot Access Body

Closures for conditions receive `Request<()>`:

```rust
let condition = |req: &Request<()>| {
    // Can access: method, URI, headers, extensions
    // Cannot access: body (it's ())
    req.uri().path().starts_with("/api/")
};
```

This is intentional - conditions should only examine metadata.

### 9. Body Preservation is Guaranteed

The body is never modified during rewriting:

```rust
let request = Request::builder()
    .body(expensive_stream)
    .unwrap();

let result = rewriter.rewrite(request).unwrap();
// expensive_stream was moved but never read or cloned
```

This makes the library zero-copy for request bodies.

### 10. Thread Safety Requirements

All conditions and rewriters must be `Send + Sync`:

```rust
pub trait Condition: Send + Sync { /* ... */ }
```

This means they can be shared across threads safely. If you implement custom conditions/rewriters, ensure they don't use non-thread-safe types like `Rc` or `RefCell`.

### 11. Capture Group Syntax

Regex replacements use `$1`, `$2` syntax, not `\1`, `\2`:

```rust
PathRewriter::new(r"^/user/(\d+)$", "/users/$1")  // Correct
PathRewriter::new(r"^/user/(\d+)$", "/users/\\1") // Wrong
```

### 12. N-API Feature Flag

Node.js bindings are optional and enabled via feature flag:

```toml
[features]
napi-support = ["dep:napi", "dep:napi-derive", ...]
```

Without this feature, `napi.rs` is not compiled and the binary stays small for Rust-only usage.

## Development Workflow

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Linting
```bash
cargo clippy
```

### Documentation
```bash
cargo doc --open
```

### With N-API Support
```bash
cargo build --features napi-support
```

## Summary

The `http-rewriter` library provides a powerful, composable framework for HTTP request transformation. Key takeaways:

1. **Two core traits**: `Condition` for matching, `Rewriter` for transforming
2. **Generic over body type**: Works with any request body
3. **Fluent API**: Extension traits enable method chaining
4. **Composable**: Conditions and rewriters combine with `and`, `or`, `then`, `when`
5. **Type-safe**: Leverages Rust's type system for correctness
6. **Body preservation**: Request bodies are never copied or modified
7. **Regex-powered**: Pattern matching and replacement for paths and headers
8. **File system aware**: Optional file existence conditions for static file serving
9. **Extensible**: Closures can be used as conditions and rewriters
10. **Node.js compatible**: Optional N-API bindings for JavaScript usage

The architecture is designed to be intuitive while providing maximum flexibility for complex request transformation scenarios.

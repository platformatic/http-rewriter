# http-rewriter

> [!WARNING]
> Not yet released

A request rewriting framework for the [http](https://docs.rs/http) crate.
It is split into `Condition` and `Rewriter` traits which may be implemented
separately or together to give a type complex rewriting behaviour.

Several combinators are provided to allow for easy construction of complex
conditions including `when` to gate a rewriter on a condition, `then` to allow
sequencing of multiple distinct rewrite rules, and `and`/`or` to combine
conditions with logical operators.

## Install

```sh
cargo add http-rewriter
```

## Usage

```rust
use http::Request;
use http_rewriter::{MethodCondition, HeaderCondition, PathRewriter};

let request = Request::builder()
    .method("GET")
    .uri("http://example.com/api/v1/resource")
    .header("Accept", "application/json")
    .body(())
    .unwrap();

let rewriter = PathRewriter::new("^/api/v1", "^/api/v2")?
  .when(
    MethodCondition::new("GET")?
      .and(HeaderCondition::new("Accept", "application/json")?)
  ).then(
    PathRewriter::new("/resource$", "/json_resource")?
      .when(
        MethodCondition::new("POST")?
          .and(HeaderCondition::new("Content-Type", "application/json")?)
      )
  );

let new_request = rewriter.rewrite(request)
    .expect("rewriting should succeed");

assert_eq!(new_request.uri().path(), "/api/v2/resource");
```

//! Request rewriting for HTTP types
//!
//! This module provides a flexible and composable system for rewriting HTTP requests.
//! It works with any request body type and integrates seamlessly with the `http` crate.
//!
//! # Overview
//!
//! The rewrite module consists of three main components:
//!
//! - **Conditions**: Types that match against HTTP requests based on various criteria
//! - **Rewriters**: Types that transform HTTP requests in different ways
//! - **Conditional Rewriters**: Combines conditions and rewriters to apply transformations selectively
//!
//! All types are generic over the request body type, making them compatible with
//! streaming requests and different body representations.
//!
//! # Examples
//!
//! ## Basic Path Rewriting
//!
//! ```
//! use http_rewriter::{Rewriter, PathRewriter};
//! use http::Request;
//!
//! // Rewrite old API paths to new ones
//! let rewriter = PathRewriter::new("^/api/v1/", "/api/v2/").unwrap();
//!
//! let request = Request::builder()
//!     .uri("/api/v1/users")
//!     .body(())
//!     .unwrap();
//!
//! let result = rewriter.rewrite(request).unwrap();
//! assert_eq!(result.uri().path(), "/api/v2/users");
//! ```
//!
//! ## Conditional Rewriting
//!
//! ```
//! use http_rewriter::{
//!     Rewriter, PathRewriter, MethodCondition, RewriterExt
//! };
//! use http::{Request, Method};
//!
//! // Only rewrite POST requests
//! let rewriter = PathRewriter::new("^/form/", "/submit/").unwrap()
//!     .when(MethodCondition::new(Method::POST)
//!         .expect("Method::POST is always valid"));
//!
//! let post_req = Request::builder()
//!     .method(Method::POST)
//!     .uri("/form/contact")
//!     .body(())
//!     .unwrap();
//!
//! let result = rewriter.rewrite(post_req).unwrap();
//! assert_eq!(result.uri().path(), "/submit/contact");
//!
//! let get_req = Request::builder()
//!     .method(Method::GET)
//!     .uri("/form/contact")
//!     .body(())
//!     .unwrap();
//!
//! let result = rewriter.rewrite(get_req).unwrap();
//! assert_eq!(result.uri().path(), "/form/contact"); // Unchanged
//! ```
//!
//! ## Chaining Rewriters
//!
//! ```
//! use http_rewriter::{
//!     Rewriter, RewriterExt, PathRewriter, MethodRewriter, HeaderRewriter
//! };
//! use http::{Request, Method};
//!
//! // Build a complex transformation pipeline
//! let pipeline = PathRewriter::new("^/old/", "/new/").unwrap()
//!     .then(MethodRewriter::new(Method::POST).unwrap())
//!     .then(HeaderRewriter::new("X-Processed", ".*", "true").unwrap());
//!
//! let request = Request::builder()
//!     .method(Method::GET)
//!     .uri("/old/api")
//!     .header("X-Processed", "false")
//!     .body(())
//!     .unwrap();
//!
//! let result = pipeline.rewrite(request).unwrap();
//! assert_eq!(result.uri().path(), "/new/api");
//! assert_eq!(result.method(), Method::POST);
//! assert_eq!(result.headers().get("x-processed").unwrap(), "true");
//! ```
//!
//! ## Complex Conditions
//!
//! ```
//! use http_rewriter::{
//!     Condition, ConditionExt, PathCondition, HeaderCondition, MethodCondition
//! };
//! use http::{Request, Method};
//!
//! // Match API requests that accept JSON
//! let api_json = PathCondition::new("^/api/.*").unwrap()
//!     .and(HeaderCondition::new("Accept", ".*json.*").unwrap());
//!
//! // Match POST or PUT requests
//! let write_methods = MethodCondition::new(Method::POST)
//!         .expect("Method::POST is always valid")
//!     .or(MethodCondition::new(Method::PUT)
//!         .expect("Method::PUT is always valid"));
//!
//! // Combine: API JSON requests that are POST or PUT
//! let complex = api_json.and(write_methods);
//!
//! let request = Request::builder()
//!     .method(Method::POST)
//!     .uri("/api/users")
//!     .header("Accept", "application/json")
//!     .body(())
//!     .unwrap();
//!
//! assert!(complex.matches(&request));
//! ```
//!
//! ## Custom Rewriters with Closures
//!
//! ```
//! use http_rewriter::{Rewriter, RewriteError};
//! use http::Request;
//!
//! // Add authentication header
//! let add_auth = |mut request: Request<()>| -> Result<Request<()>, RewriteError> {
//!     request.headers_mut().insert(
//!         "Authorization",
//!         "Bearer secret-token".parse().unwrap()
//!     );
//!     Ok(request)
//! };
//!
//! let request = Request::builder()
//!     .uri("/protected")
//!     .body(())
//!     .unwrap();
//!
//! let result = add_auth.rewrite(request).unwrap();
//! assert_eq!(result.headers().get("authorization").unwrap(), "Bearer secret-token");
//! ```
//!
//! # Module Structure
//!
//! - [`condition`]: Types for matching requests (PathCondition, MethodCondition, etc.)
//! - [`rewriter`]: Types for transforming requests (PathRewriter, HeaderRewriter, etc.)
//! - [`conditional_rewriter`]: Combines conditions and rewriters

#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![warn(missing_docs)]

pub mod condition;
pub mod conditional_rewriter;
pub mod rewriter;

#[cfg(test)]
mod integration_tests;

pub use condition::{
    Condition, ConditionExt, ExistenceCondition, GroupCondition, HeaderCondition, MethodCondition,
    NonExistenceCondition, PathCondition,
};
pub use conditional_rewriter::ConditionalRewriter;
pub use rewriter::{
    HeaderRewriter, HrefRewriter, MethodRewriter, PathRewriter, RewriteError, Rewriter,
    RewriterExt, SequenceRewriter,
};

/// Provides N-API bindings to expose the `http_rewriter` crate types to Node.js.
#[cfg(feature = "napi-support")]
pub mod napi;

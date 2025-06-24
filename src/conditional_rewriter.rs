//! Conditional rewriter that applies rewrites based on conditions
//!
//! This module provides the [`ConditionalRewriter`] type which combines a rewriter
//! with a condition, allowing transformations to be applied only when specific
//! criteria are met.
//!
//! # Examples
//!
//! ```
//! use http_rewriter::{ConditionalRewriter, PathRewriter, PathCondition, Rewriter};
//! use http::Request;
//!
//! // Only rewrite API paths
//! let condition = PathCondition::new("^/api/.*").unwrap();
//! let rewriter = PathRewriter::new("^/api/v1/", "/api/v2/").unwrap();
//! let conditional = ConditionalRewriter::new(rewriter, condition);
//!
//! // This matches the condition and gets rewritten
//! let request = Request::builder()
//!     .uri("/api/v1/users")
//!     .body(())
//!     .unwrap();
//! let result = conditional.rewrite(request).unwrap();
//! assert_eq!(result.uri().path(), "/api/v2/users");
//!
//! // This doesn't match the condition and passes through unchanged
//! let request = Request::builder()
//!     .uri("/home")
//!     .body(())
//!     .unwrap();
//! let result = conditional.rewrite(request).unwrap();
//! assert_eq!(result.uri().path(), "/home");
//! ```

use http::Request;
use super::{condition::Condition, rewriter::{Rewriter, RewriteError}};

/// Rewriter that applies another rewriter conditionally based on a condition
///
/// This type combines a [`Rewriter`] with a [`Condition`], only applying the
/// rewriter's transformations when the condition matches the request. If the
/// condition doesn't match, the request passes through unchanged.
///
/// ConditionalRewriter is typically created using the [`RewriterExt::when`]
/// method rather than directly.
///
/// # Type Parameters
///
/// * `R` - The rewriter type to apply conditionally
/// * `C` - The condition type that determines when to apply the rewriter
///
/// # Examples
///
/// ```
/// use http_rewriter::{
///     ConditionalRewriter, PathRewriter, MethodCondition,
///     HeaderRewriter, Rewriter, ConditionExt
/// };
/// use http::{Request, Method};
///
/// // Only rewrite POST requests
/// let rewriter = PathRewriter::new("^/form/", "/submit/").unwrap();
/// let condition = MethodCondition::new(Method::POST)
///     .expect("Method::POST is always valid");
/// let conditional = ConditionalRewriter::new(rewriter, condition);
///
/// // POST request gets rewritten
/// let request = Request::builder()
///     .method(Method::POST)
///     .uri("/form/contact")
///     .body(())
///     .unwrap();
/// let result = conditional.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/submit/contact");
///
/// // GET request passes through unchanged
/// let request = Request::builder()
///     .method(Method::GET)
///     .uri("/form/contact")
///     .body(())
///     .unwrap();
/// let result = conditional.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/form/contact");
/// ```
///
/// ```
/// use http_rewriter::{
///     ConditionalRewriter, HeaderRewriter, HeaderCondition,
///     PathCondition, Rewriter, ConditionExt
/// };
/// use http::Request;
///
/// // Complex condition: JSON requests to API endpoints
/// let api_condition = PathCondition::new("^/api/.*").unwrap();
/// let json_condition = HeaderCondition::new("Content-Type", "application/json").unwrap();
/// let combined = api_condition.and(json_condition);
///
/// // Add API version header only for matching requests
/// let rewriter = HeaderRewriter::new("X-API-Version", ".*", "2.0").unwrap();
/// let conditional = ConditionalRewriter::new(rewriter, combined);
///
/// // Matching request gets the header added
/// let request = Request::builder()
///     .uri("/api/users")
///     .header("Content-Type", "application/json")
///     .header("X-API-Version", "1.0")
///     .body(())
///     .unwrap();
/// let result = conditional.rewrite(request).unwrap();
/// assert_eq!(result.headers().get("x-api-version").unwrap(), "2.0");
/// ```
pub struct ConditionalRewriter<R, C> {
    rewriter: R,
    condition: C,
}

impl<R: Rewriter, C: Condition> ConditionalRewriter<R, C> {
    /// Create a new conditional rewriter
    ///
    /// # Arguments
    ///
    /// * `rewriter` - The rewriter to apply when the condition matches
    /// * `condition` - The condition that determines when to apply the rewriter
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{
    ///     ConditionalRewriter, PathRewriter, MethodCondition
    /// };
    /// use http::Method;
    ///
    /// let rewriter = PathRewriter::new("/old/", "/new/").unwrap();
    /// let condition = MethodCondition::new(Method::POST)
    ///     .expect("Method::POST is always valid");
    /// let conditional = ConditionalRewriter::new(rewriter, condition);
    /// ```
    pub fn new(rewriter: R, condition: C) -> Self {
        Self { rewriter, condition }
    }
}

impl<R: Rewriter, C: Condition> Rewriter for ConditionalRewriter<R, C> {
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        if self.condition.matches(&request) {
            self.rewriter.rewrite(request)
        } else {
            Ok(request)
        }
    }
}

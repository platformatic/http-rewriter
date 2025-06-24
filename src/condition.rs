//! Conditions for matching HTTP requests
//!
//! This module provides various condition types that can be used to match HTTP requests
//! based on different criteria such as path patterns, HTTP methods, headers, and file existence.
//!
//! # Overview
//!
//! Conditions implement the [`Condition`] trait and can be combined using logical operators
//! (AND/OR) to create complex matching rules. All conditions work with any request body type.
//!
//! # Examples
//!
//! ```
//! use http_rewriter::{Condition, PathCondition, MethodCondition, ConditionExt};
//! use http::{Request, Method};
//!
//! // Match requests to /api/*
//! let api_condition = PathCondition::new("^/api/.*").unwrap();
//!
//! // Match POST requests
//! let post_condition = MethodCondition::new(Method::POST)
//!     .expect("Method::POST is always valid");
//!
//! // Combine conditions: POST requests to /api/*
//! let combined = api_condition.and(post_condition);
//!
//! let request = Request::builder()
//!     .method(Method::POST)
//!     .uri("/api/users")
//!     .body(())
//!     .unwrap();
//!
//! assert!(combined.matches(&request));
//! ```

use http::Request;
use http_handler::RequestExt;
use regex::Regex;

/// Trait for types that can match against HTTP requests
///
/// This trait is implemented by all condition types and allows them to test
/// whether a request matches certain criteria. The trait is generic over the
/// request body type, allowing conditions to work with streaming requests.
///
/// # Examples
///
/// ```
/// use http_rewriter::Condition;
/// use http::Request;
///
/// // Custom condition that matches requests with paths longer than 10 characters
/// struct LongPathCondition;
///
/// impl Condition for LongPathCondition {
///     fn matches<B>(&self, request: &Request<B>) -> bool {
///         request.uri().path().len() > 10
///     }
/// }
///
/// let condition = LongPathCondition;
/// let request = Request::builder()
///     .uri("/very/long/path/here")
///     .body(())
///     .unwrap();
///
/// assert!(condition.matches(&request));
/// ```
pub trait Condition: Send + Sync {
    /// Check if the condition matches the request
    ///
    /// Returns `true` if the request matches this condition's criteria,
    /// `false` otherwise.
    fn matches<B>(&self, request: &Request<B>) -> bool;
}

/// Condition that matches request paths against a regular expression pattern
///
/// This condition uses regular expressions to match against the request's URI path.
/// The pattern is compiled when the condition is created, providing efficient
/// matching for repeated use.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Condition, PathCondition};
/// use http::Request;
///
/// // Match requests to /api/* endpoints
/// let api_condition = PathCondition::new("^/api/.*").unwrap();
///
/// let request = Request::builder()
///     .uri("/api/users")
///     .body(())
///     .unwrap();
/// assert!(api_condition.matches(&request));
///
/// let request = Request::builder()
///     .uri("/home")
///     .body(())
///     .unwrap();
/// assert!(!api_condition.matches(&request));
/// ```
///
/// ```
/// use http_rewriter::{Condition, PathCondition};
/// use http::Request;
///
/// // Match specific file extensions
/// let image_condition = PathCondition::new(r"\.(jpg|jpeg|png|gif)$").unwrap();
///
/// let request = Request::builder()
///     .uri("/images/photo.jpg")
///     .body(())
///     .unwrap();
/// assert!(image_condition.matches(&request));
/// ```
#[derive(Debug, Clone)]
pub struct PathCondition {
    pattern: Regex,
}

impl PathCondition {
    /// Create a new path condition with the given regular expression pattern
    ///
    /// # Arguments
    ///
    /// * `pattern` - A regular expression pattern to match against request paths
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern is not a valid regular expression
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::PathCondition;
    ///
    /// // Match paths starting with /admin/
    /// let condition = PathCondition::new("^/admin/").unwrap();
    ///
    /// // Match paths ending with .html
    /// let condition = PathCondition::new(r"\.html$").unwrap();
    ///
    /// // Invalid regex will return an error
    /// assert!(PathCondition::new("[unclosed").is_err());
    /// ```
    pub fn new(pattern: impl AsRef<str>) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: Regex::new(pattern.as_ref())?,
        })
    }
}

impl Condition for PathCondition {
    fn matches<B>(&self, request: &Request<B>) -> bool {
        self.pattern.is_match(request.uri().path())
    }
}

/// Condition that matches requests based on their HTTP method
///
/// This condition checks if the request's HTTP method matches a specific method
/// such as GET, POST, PUT, DELETE, etc. It accepts both `Method` types and
/// string representations.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Condition, MethodCondition};
/// use http::{Request, Method};
///
/// // Match only POST requests using Method type
/// let post_only = MethodCondition::new(Method::POST)
///     .expect("Method::POST is always valid");
///
/// let post_request = Request::builder()
///     .method(Method::POST)
///     .uri("/submit")
///     .body(())
///     .unwrap();
/// assert!(post_only.matches(&post_request));
///
/// let get_request = Request::builder()
///     .method(Method::GET)
///     .uri("/submit")
///     .body(())
///     .unwrap();
/// assert!(!post_only.matches(&get_request));
/// ```
///
/// ```
/// use http_rewriter::{Condition, MethodCondition};
/// use http::Request;
///
/// // Create conditions using string representations
/// let get_condition = MethodCondition::new("GET")
///     .expect("GET is a valid HTTP method");
/// let post_condition = MethodCondition::new("POST")
///     .expect("POST is a valid HTTP method");
/// let custom_condition = MethodCondition::new("CUSTOM")
///     .expect("CUSTOM is a valid HTTP method");
///
/// let request = Request::builder()
///     .method("GET")
///     .uri("/data")
///     .body(())
///     .unwrap();
/// assert!(get_condition.matches(&request));
/// assert!(!post_condition.matches(&request));
/// ```
#[derive(Debug, Clone)]
pub struct MethodCondition {
    method: Regex,
}

impl MethodCondition {
    /// Create a new method condition for the specified HTTP method
    ///
    /// This method accepts any type that can be converted into a `Method` using `TryInto`.
    /// The error type depends on the input type:
    /// - `Method` → `Infallible` (never fails)
    /// - `&str` → `InvalidMethod` (can fail)
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to match against
    ///
    /// # Errors
    ///
    /// Returns the error from `TryInto<Method>` conversion, which is:
    /// - `Infallible` when passing a `Method` directly
    /// - `InvalidMethod` when passing a string that cannot be parsed
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::MethodCondition;
    /// use http::Method;
    ///
    /// // Using Method types - these return Result<_, Infallible>
    /// let get_condition = MethodCondition::new(Method::GET)
    ///     .expect("Method type always succeeds");
    /// let post_condition = MethodCondition::new(Method::POST)
    ///     .expect("Method type always succeeds");
    ///
    /// // Using strings - these return Result<_, InvalidMethod>
    /// let put_condition = MethodCondition::new("PUT")
    ///     .expect("PUT is a valid method");
    /// let delete_condition = MethodCondition::new("DELETE")
    ///     .expect("DELETE is a valid method");
    ///
    /// // Custom methods are also supported
    /// let custom_condition = MethodCondition::new("CUSTOM")
    ///     .expect("CUSTOM is a valid method");
    /// ```
    pub fn new(method: impl AsRef<str>) -> Result<Self, regex::Error> {
        Ok(Self {
            method: Regex::new(method.as_ref())?,
        })
    }
}

impl Condition for MethodCondition {
    fn matches<B>(&self, request: &Request<B>) -> bool {
        self.method.is_match(request.method().as_str())
    }
}

/// Condition that matches request headers against a regular expression pattern
///
/// This condition checks if a specific header exists and its value matches
/// a regular expression pattern. Header names are case-insensitive.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Condition, HeaderCondition};
/// use http::Request;
///
/// // Match requests with JSON content type
/// let json_condition = HeaderCondition::new("Content-Type", "application/json").unwrap();
///
/// let request = Request::builder()
///     .uri("/api/data")
///     .header("Content-Type", "application/json")
///     .body(())
///     .unwrap();
/// assert!(json_condition.matches(&request));
/// ```
///
/// ```
/// use http_rewriter::{Condition, HeaderCondition};
/// use http::Request;
///
/// // Match requests from specific user agents
/// let bot_condition = HeaderCondition::new("User-Agent", ".*(bot|crawler).*").unwrap();
///
/// let request = Request::builder()
///     .uri("/")
///     .header("User-Agent", "Googlebot/2.1")
///     .body(())
///     .unwrap();
/// assert!(bot_condition.matches(&request));
/// ```
#[derive(Debug, Clone)]
pub struct HeaderCondition {
    name: String,
    pattern: Regex,
}

impl HeaderCondition {
    /// Create a new header condition
    ///
    /// # Arguments
    ///
    /// * `name` - The header name to check (case-insensitive)
    /// * `pattern` - A regular expression pattern to match against the header value
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern is not a valid regular expression
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::HeaderCondition;
    ///
    /// // Exact match
    /// let exact = HeaderCondition::new("Accept", "text/html").unwrap();
    ///
    /// // Pattern match
    /// let pattern = HeaderCondition::new("Accept", "text/.*").unwrap();
    ///
    /// // Match any value containing "gzip"
    /// let encoding = HeaderCondition::new("Accept-Encoding", ".*gzip.*").unwrap();
    /// ```
    pub fn new(name: impl Into<String>, pattern: impl AsRef<str>) -> Result<Self, regex::Error> {
        Ok(Self {
            name: name.into(),
            pattern: Regex::new(pattern.as_ref())?,
        })
    }
}

impl Condition for HeaderCondition {
    fn matches<B>(&self, request: &Request<B>) -> bool {
        request
            .headers()
            .get(&self.name)
            .and_then(|value| value.to_str().ok())
            .map(|value| self.pattern.is_match(value))
            .unwrap_or(false)
    }
}

/// Condition that matches if a file exists on the filesystem
///
/// This condition checks if the request path, when resolved relative to the
/// document root stored in the request extensions, corresponds to an existing
/// file or directory. This is useful for implementing fallback behavior or
/// static file serving.
///
/// The document root must be set in the request extensions using the
/// `DocumentRoot` type. If no document root is set, the condition will
/// not match.
///
/// # Security Note
///
/// This condition performs filesystem access and should be used carefully.
/// The document root should be an absolute path to prevent directory traversal attacks.
///
/// # Examples
///
/// ```no_run
/// use http_handler::RequestBuilderExt;
/// use http_rewriter::{Condition, ExistenceCondition};
/// use http::Request;
///
/// let condition = ExistenceCondition::new();
///
/// // The framework should set the document root before checking
/// let mut request = Request::builder()
///     .uri("/index.html")
///     .document_root("/var/www/html".to_string().into())
///     .body(())
///     .unwrap();
///
/// // This would check for /var/www/html/index.html
/// let exists = condition.matches(&request);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ExistenceCondition;

impl ExistenceCondition {
    /// Create a new existence condition
    ///
    /// The document root must be provided via the request extensions
    /// using the `DocumentRoot` type.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::ExistenceCondition;
    ///
    /// let condition = ExistenceCondition::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Condition for ExistenceCondition {
    fn matches<B>(&self, request: &Request<B>) -> bool {
        if let Some(doc_root) = request.document_root() {
            let path = request.uri().path();
            let stripped = path.strip_prefix('/').unwrap_or(path);
            doc_root.join(stripped).exists()
        } else {
            // No document root set, cannot check existence
            false
        }
    }
}

/// Condition that matches if a file does NOT exist on the filesystem
///
/// This condition is the opposite of [`ExistenceCondition`] - it matches when
/// the request path does not correspond to an existing file or directory.
/// This is useful for implementing rewrite rules that only apply when a
/// file is missing, such as routing to a front controller.
///
/// The document root must be set in the request extensions using the
/// `DocumentRoot` type. If no document root is set, the condition will
/// not match.
///
/// # Examples
///
/// ```no_run
/// use http_handler::RequestBuilderExt;
/// use http_rewriter::{Condition, NonExistenceCondition, ConditionExt, PathCondition};
/// use http::Request;
///
/// // Rewrite non-existent paths to index.php (front controller pattern)
/// let not_file = NonExistenceCondition::new();
/// let not_asset = PathCondition::new(r"^(?!.*\.(js|css|jpg|png)).*").unwrap();
///
/// // Only rewrite if file doesn't exist AND it's not an asset
/// let condition = not_file.and(not_asset);
///
/// let mut request = Request::builder()
///     .uri("/some/route")
///     .document_root("/var/www/html".to_string().into())
///     .body(())
///     .unwrap();
///
/// // Returns true if /var/www/html/some/route doesn't exist
/// let should_rewrite = condition.matches(&request);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct NonExistenceCondition;

impl NonExistenceCondition {
    /// Create a new non-existence condition
    ///
    /// The document root must be provided via the request extensions
    /// using the `DocumentRoot` type.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::NonExistenceCondition;
    ///
    /// let condition = NonExistenceCondition::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Condition for NonExistenceCondition {
    fn matches<B>(&self, request: &Request<B>) -> bool {
        if let Some(doc_root) = request.document_root() {
            let path = request.uri().path();
            let stripped = path.strip_prefix('/').unwrap_or(path);
            !doc_root.path.join(stripped).exists()
        } else {
            // No document root set, cannot check existence
            false
        }
    }
}

/// Condition that groups multiple conditions with AND or OR logic
///
/// This condition allows combining multiple conditions using boolean logic.
/// It can operate in AND mode (all conditions must match) or OR mode
/// (at least one condition must match).
///
/// GroupCondition is typically created using the [`ConditionExt`] trait's
/// `and()` and `or()` methods rather than directly.
///
/// # Examples
///
/// ```
/// use http_rewriter::{GroupCondition, PathCondition, MethodCondition, Condition};
/// use http::{Request, Method};
///
/// // Create an OR group manually
/// let path1 = PathCondition::new("^/api/.*").unwrap();
/// let path2 = PathCondition::new("^/admin/.*").unwrap();
/// let or_group = GroupCondition::or(Box::new(path1), Box::new(path2));
///
/// let request = Request::builder()
///     .uri("/api/users")
///     .body(())
///     .unwrap();
/// assert!(or_group.matches(&request));
///
/// let request = Request::builder()
///     .uri("/admin/panel")
///     .body(())
///     .unwrap();
/// assert!(or_group.matches(&request));
///
/// let request = Request::builder()
///     .uri("/home")
///     .body(())
///     .unwrap();
/// assert!(!or_group.matches(&request));
/// ```
pub enum GroupCondition<A, B>
where
    A: Condition + ?Sized,
    B: Condition + ?Sized,
{
    /// Combines two conditions using logical AND
    And(Box<A>, Box<B>),
    /// Combines two conditions using logical OR
    Or(Box<A>, Box<B>),
}

impl<A, B> GroupCondition<A, B>
where
    A: Condition + ?Sized,
    B: Condition + ?Sized,
{
    /// Create a new AND group condition from two conditions
    ///
    /// Both conditions must match for the group to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{GroupCondition, PathCondition, MethodCondition};
    /// use http::Method;
    ///
    /// let path_cond = PathCondition::new("^/api/.*").unwrap();
    /// let method_cond = MethodCondition::new(Method::POST)
    ///     .expect("Method::POST is always valid");
    /// let and_group = GroupCondition::and(Box::new(path_cond), Box::new(method_cond));
    /// ```
    pub fn and(a: Box<A>, b: Box<B>) -> Box<Self> {
        Box::new(GroupCondition::And(a, b))
    }

    /// Create a new OR group condition from two conditions
    ///
    /// At least one condition must match for the group to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{GroupCondition, MethodCondition};
    /// use http::Method;
    ///
    /// let post_cond = MethodCondition::new(Method::POST)
    ///     .expect("Method::POST is always valid");
    /// let put_cond = MethodCondition::new(Method::PUT)
    ///     .expect("Method::PUT is always valid");
    /// let or_group = GroupCondition::or(Box::new(post_cond), Box::new(put_cond));
    /// ```
    pub fn or(a: Box<A>, b: Box<B>) -> Box<Self> {
        Box::new(GroupCondition::Or(a, b))
    }
}

impl<A, B> Condition for GroupCondition<A, B>
where
    A: Condition + ?Sized,
    B: Condition + ?Sized,
{
    fn matches<Body>(&self, request: &Request<Body>) -> bool {
        match self {
            GroupCondition::And(a, b) => a.matches(request) && b.matches(request),
            GroupCondition::Or(a, b) => a.matches(request) || b.matches(request),
        }
    }
}

/// Extension trait for combining conditions with boolean logic
///
/// This trait provides convenient methods for combining conditions using
/// AND and OR logic. It's automatically implemented for all types that
/// implement [`Condition`].
///
/// # Examples
///
/// ```
/// use http_rewriter::{Condition, ConditionExt, PathCondition, MethodCondition};
/// use http::{Request, Method};
///
/// // Combine conditions with AND
/// let api_post = PathCondition::new("^/api/.*").unwrap()
///     .and(MethodCondition::new(Method::POST)
///         .expect("Method::POST is always valid"));
///
/// let request = Request::builder()
///     .method(Method::POST)
///     .uri("/api/users")
///     .body(())
///     .unwrap();
/// assert!(api_post.matches(&request));
///
/// // Combine conditions with OR
/// let post_or_put = MethodCondition::new(Method::POST)
///         .expect("Method::POST is always valid")
///     .or(MethodCondition::new(Method::PUT)
///         .expect("Method::PUT is always valid"));
///
/// let request = Request::builder()
///     .method(Method::PUT)
///     .uri("/resource")
///     .body(())
///     .unwrap();
/// assert!(post_or_put.matches(&request));
/// ```
///
/// ```
/// use http_rewriter::{Condition, ConditionExt, PathCondition, HeaderCondition};
/// use http::Request;
///
/// // Complex condition: API requests that accept JSON
/// let api_json = PathCondition::new("^/api/.*").unwrap()
///     .and(HeaderCondition::new("Accept", ".*json.*").unwrap());
///
/// let request = Request::builder()
///     .uri("/api/data")
///     .header("Accept", "application/json")
///     .body(())
///     .unwrap();
/// assert!(api_json.matches(&request));
/// ```
pub trait ConditionExt: Condition + Sized + 'static {
    /// Create a new condition that matches when both conditions match
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{ConditionExt, PathCondition, MethodCondition};
    /// use http::Method;
    ///
    /// // Match POST requests to /api/*
    /// let condition = PathCondition::new("^/api/.*").unwrap()
    ///     .and(MethodCondition::new(Method::POST)
    ///         .expect("Method::POST is always valid"));
    /// ```
    fn and<C: Condition + 'static>(self, other: C) -> GroupCondition<Self, C> {
        GroupCondition::And(Box::new(self), Box::new(other))
    }

    /// Create a new condition that matches when either condition matches
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{ConditionExt, PathCondition};
    ///
    /// // Match requests to either /api/* or /admin/*
    /// let condition = PathCondition::new("^/api/.*").unwrap()
    ///     .or(PathCondition::new("^/admin/.*").unwrap());
    /// ```
    fn or<C: Condition + 'static>(self, other: C) -> GroupCondition<Self, C> {
        GroupCondition::Or(Box::new(self), Box::new(other))
    }
}

// Implement ConditionExt for all types that implement Condition
impl<T: Condition + 'static> ConditionExt for T {}

/// Implementation of Condition for closures
///
/// Any closure that takes a `&Request<()>` and returns a `bool` can be used
/// as a condition. The request body is ignored in conditions - only the
/// metadata (method, URI, headers, extensions) is considered.
///
/// This preserves the request body throughout the rewrite process while
/// allowing ergonomic closure-based conditions.
///
/// # Examples
///
/// ```
/// use http_rewriter::Condition;
/// use http::Request;
///
/// // Simple closure condition that checks path length
/// let long_path = |request: &Request<()>| -> bool {
///     request.uri().path().len() > 20
/// };
///
/// let request = Request::builder()
///     .uri("/very/long/path/to/resource")
///     .body("some body")
///     .unwrap();
/// assert!(long_path.matches(&request));
/// ```
impl<F> Condition for F
where
    F: Fn(&Request<()>) -> bool + Send + Sync,
{
    fn matches<B>(&self, request: &Request<B>) -> bool {
        // SAFETY: We transmute the request to have a () body type.
        // This is safe because:
        // 1. We're only reading from the request (immutable borrow)
        // 2. The closure should only access metadata, not the body
        // 3. The request structure layout is the same regardless of body type
        // 4. We never actually access the body field through the closure
        unsafe {
            let request_ref: &Request<()> = std::mem::transmute(request);
            self(request_ref)
        }
    }
}

//! Rewriters for transforming HTTP requests
//!
//! This module provides various rewriter types that can transform HTTP requests
//! by modifying their paths, methods, headers, or other properties. Rewriters
//! can be chained together and applied conditionally.
//!
//! # Overview
//!
//! Rewriters implement the [`Rewriter`] trait and can transform any part of an
//! HTTP request while preserving the body. They work with any request body type,
//! making them compatible with streaming requests.
//!
//! # Examples
//!
//! ```
//! use http_rewriter::{Rewriter, PathRewriter, RewriterExt};
//! use http::Request;
//!
//! // Rewrite /old/* to /new/*
//! let rewriter = PathRewriter::new("^/old/(.*)", "/new/$1").unwrap();
//!
//! let request = Request::builder()
//!     .uri("/old/api/users")
//!     .body(())
//!     .unwrap();
//!
//! let result = rewriter.rewrite(request).unwrap();
//! assert_eq!(result.uri().path(), "/new/api/users");
//! ```
//!
//! ```
//! use http_rewriter::{Rewriter, PathRewriter, MethodRewriter, RewriterExt};
//! use http::{Request, Method};
//!
//! // Chain multiple rewriters
//! let rewriter = PathRewriter::new("^/api/(.*)", "/v2/$1").unwrap()
//!     .then(MethodRewriter::new(Method::POST).unwrap());
//!
//! let request = Request::builder()
//!     .method(Method::GET)
//!     .uri("/api/users")
//!     .body(())
//!     .unwrap();
//!
//! let result = rewriter.rewrite(request).unwrap();
//! assert_eq!(result.uri().path(), "/v2/users");
//! assert_eq!(result.method(), Method::POST);
//! ```

use super::{Condition, ConditionalRewriter};
use http::{Method, Request};
use regex::Regex;

/// Error type for rewrite operations
///
/// This error is returned when a rewrite operation fails, such as when
/// an invalid URI is produced or a header value is malformed.
#[derive(Debug, Clone, PartialEq)]
pub struct RewriteError(String);

impl RewriteError {
    /// Create a new rewrite error with the given message
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RewriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rewrite error: {}", self.0)
    }
}

impl std::error::Error for RewriteError {}

/// Trait for types that can transform HTTP requests
///
/// This trait is implemented by all rewriter types and allows them to
/// transform requests. The trait is generic over the request body type,
/// allowing rewriters to work with streaming requests.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, RewriteError};
/// use http::Request;
///
/// // Custom rewriter that adds a header
/// struct ApiVersionRewriter;
///
/// impl Rewriter for ApiVersionRewriter {
///     fn rewrite<B>(&self, mut request: Request<B>) -> Result<Request<B>, RewriteError> {
///         request.headers_mut().insert(
///             "X-API-Version",
///             "2.0".parse().unwrap()
///         );
///         Ok(request)
///     }
/// }
///
/// let rewriter = ApiVersionRewriter;
/// let request = Request::builder()
///     .uri("/api/users")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.headers().get("x-api-version").unwrap(), "2.0");
/// ```
pub trait Rewriter: Send + Sync {
    /// Apply the rewrite transformation to the request
    ///
    /// Returns the transformed request or an error if the transformation fails.
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError>;
}

/// Rewriter that transforms request paths using regex pattern and replacement
///
/// This rewriter uses regular expressions to match and replace parts of the
/// request URI path. It preserves query parameters and other URI components.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, PathRewriter};
/// use http::Request;
///
/// // Simple path prefix change
/// let rewriter = PathRewriter::new("^/old/", "/new/").unwrap();
///
/// let request = Request::builder()
///     .uri("/old/api/users?page=1")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/new/api/users");
/// assert_eq!(result.uri().query(), Some("page=1"));
/// ```
///
/// ```
/// use http_rewriter::{Rewriter, PathRewriter};
/// use http::Request;
///
/// // Capture groups for dynamic rewrites
/// let rewriter = PathRewriter::new(
///     r"^/products/(\d+)/reviews/(\d+)$",
///     "/api/reviews/$2?product_id=$1"
/// ).unwrap();
///
/// let request = Request::builder()
///     .uri("/products/123/reviews/456")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/api/reviews/456");
/// assert_eq!(result.uri().query(), Some("product_id=123"));
/// ```
#[derive(Debug, Clone)]
pub struct PathRewriter {
    pattern: Regex,
    replacement: String,
}

impl PathRewriter {
    /// Create a new path rewriter with regex pattern and replacement
    ///
    /// # Arguments
    ///
    /// * `pattern` - Regular expression pattern to match against the path
    /// * `replacement` - Replacement string, can include capture group references like $1, $2
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern is not a valid regular expression
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::PathRewriter;
    ///
    /// // Simple replacement
    /// let rewriter = PathRewriter::new("/old/", "/new/").unwrap();
    ///
    /// // With capture groups
    /// let rewriter = PathRewriter::new(r"^/user/(\d+)$", "/users/$1/profile").unwrap();
    ///
    /// // Remove path prefix
    /// let rewriter = PathRewriter::new("^/api/v1/", "/").unwrap();
    /// ```
    pub fn new(
        pattern: impl AsRef<str>,
        replacement: impl Into<String>,
    ) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: Regex::new(pattern.as_ref())?,
            replacement: replacement.into(),
        })
    }
}

impl Rewriter for PathRewriter {
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        let (mut parts, body) = request.into_parts();

        let path = parts.uri.path().to_string();
        let new_path = self.pattern.replace(&path, &self.replacement);

        if new_path != path {
            // Build new URI with the new path
            let uri_str = if let Some(query) = parts.uri.query() {
                format!("{}?{}", new_path, query)
            } else {
                new_path.to_string()
            };

            parts.uri = uri_str
                .parse()
                .map_err(|_| RewriteError("Invalid URI after path rewrite".to_string()))?;
        }

        Ok(Request::from_parts(parts, body))
    }
}

/// Rewriter that changes the HTTP method of requests
///
/// This rewriter changes the HTTP method to a fixed value, useful for
/// converting between different request types or enforcing specific methods.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, MethodRewriter};
/// use http::{Request, Method};
///
/// // Convert all requests to POST
/// let rewriter = MethodRewriter::new(Method::POST).unwrap();
///
/// let request = Request::builder()
///     .method(Method::GET)
///     .uri("/api/data")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.method(), Method::POST);
/// assert_eq!(result.uri().path(), "/api/data"); // URI unchanged
/// ```
///
/// ```
/// use http_rewriter::{Rewriter, MethodRewriter, PathCondition, RewriterExt};
/// use http::{Request, Method};
///
/// // Convert GET requests to /api/* to POST
/// let rewriter = MethodRewriter::new(Method::POST).unwrap()
///     .when(PathCondition::new("^/api/.*").unwrap());
///
/// let request = Request::builder()
///     .method(Method::GET)
///     .uri("/api/users")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.method(), Method::POST);
/// ```
#[derive(Debug, Clone)]
pub struct MethodRewriter {
    method: Method,
}

impl MethodRewriter {
    /// Create a new method rewriter for the specified HTTP method
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method to set on rewritten requests
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::MethodRewriter;
    /// use http::Method;
    ///
    /// // Force all requests to use POST
    /// let rewriter = MethodRewriter::new(Method::POST);
    ///
    /// // Convert to PUT for updates
    /// let rewriter = MethodRewriter::new(Method::PUT);
    /// ```
    pub fn new<M>(method: M) -> Result<Self, RewriteError>
    where
        M: TryInto<Method>,
    {
        Ok(Self {
            method: method.try_into().map_err(|_| {
                RewriteError("Invalid method specified for MethodRewriter".to_string())
            })?,
        })
    }
}

impl Rewriter for MethodRewriter {
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        let (mut parts, body) = request.into_parts();
        parts.method = self.method.clone();
        Ok(Request::from_parts(parts, body))
    }
}

/// Rewriter that transforms request headers using regex pattern and replacement
///
/// This rewriter modifies the value of a specific header using regular expression
/// matching and replacement. If the header doesn't exist or the pattern doesn't
/// match, the request is left unchanged.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, HeaderRewriter};
/// use http::Request;
///
/// // Change host header
/// let rewriter = HeaderRewriter::new("Host", "old.example.com", "new.example.com").unwrap();
///
/// let request = Request::builder()
///     .uri("/api/users")
///     .header("Host", "old.example.com")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.headers().get("host").unwrap(), "new.example.com");
/// ```
///
/// ```
/// use http_rewriter::{Rewriter, HeaderRewriter};
/// use http::Request;
///
/// // Add prefix to user agent
/// let rewriter = HeaderRewriter::new("User-Agent", "^(.*)$", "MyProxy/1.0 $1").unwrap();
///
/// let request = Request::builder()
///     .uri("/")
///     .header("User-Agent", "Mozilla/5.0")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.headers().get("user-agent").unwrap(), "MyProxy/1.0 Mozilla/5.0");
/// ```
#[derive(Debug, Clone)]
pub struct HeaderRewriter {
    name: String,
    pattern: Regex,
    replacement: String,
}

impl HeaderRewriter {
    /// Create a new header rewriter
    ///
    /// # Arguments
    ///
    /// * `name` - The header name to rewrite (case-insensitive)
    /// * `pattern` - Regular expression pattern to match against the header value
    /// * `replacement` - Replacement string, can include capture group references
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern is not a valid regular expression
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::HeaderRewriter;
    ///
    /// // Simple replacement
    /// let rewriter = HeaderRewriter::new("Host", "localhost", "127.0.0.1").unwrap();
    ///
    /// // Pattern with capture groups
    /// let rewriter = HeaderRewriter::new(
    ///     "Authorization",
    ///     r"Bearer (.+)",
    ///     "Token $1"
    /// ).unwrap();
    /// ```
    pub fn new(
        name: impl Into<String>,
        pattern: impl AsRef<str>,
        replacement: impl Into<String>,
    ) -> Result<Self, regex::Error> {
        Ok(Self {
            name: name.into(),
            pattern: Regex::new(pattern.as_ref())?,
            replacement: replacement.into(),
        })
    }
}

impl Rewriter for HeaderRewriter {
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        let (mut parts, body) = request.into_parts();

        if let Some(value) = parts.headers.get(&self.name) {
            if let Ok(value_str) = value.to_str() {
                let new_value = self.pattern.replace(value_str, &self.replacement);
                if new_value != value_str {
                    let header_name = http::HeaderName::from_bytes(self.name.as_bytes())
                        .map_err(|_| RewriteError("Invalid header name".to_string()))?;
                    let header_value = http::HeaderValue::from_str(&new_value)
                        .map_err(|_| RewriteError("Invalid header value".to_string()))?;
                    parts.headers.insert(header_name, header_value);
                }
            }
        }

        Ok(Request::from_parts(parts, body))
    }
}

/// Rewriter that transforms the entire URI (href) using regex pattern and replacement
///
/// Unlike [`PathRewriter`] which only modifies the path component, this rewriter
/// can transform the entire URI including the scheme, authority, path, and query.
/// This is useful for redirecting between domains or changing protocols.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, HrefRewriter};
/// use http::Request;
///
/// // Redirect from HTTP to HTTPS
/// let rewriter = HrefRewriter::new("^http://", "https://").unwrap();
///
/// let request = Request::builder()
///     .uri("http://example.com/api/users")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().to_string(), "https://example.com/api/users");
/// ```
///
/// ```
/// use http_rewriter::{Rewriter, HrefRewriter};
/// use http::Request;
///
/// // Redirect to a different domain
/// let rewriter = HrefRewriter::new(
///     r"^https://old\.example\.com/(.*)$",
///     "https://new.example.com/$1"
/// ).unwrap();
///
/// let request = Request::builder()
///     .uri("https://old.example.com/api/v1/users?page=2")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().to_string(), "https://new.example.com/api/v1/users?page=2");
/// ```
#[derive(Debug, Clone)]
pub struct HrefRewriter {
    pattern: Regex,
    replacement: String,
}

impl HrefRewriter {
    /// Create a new href rewriter with regex pattern and replacement
    ///
    /// # Arguments
    ///
    /// * `pattern` - Regular expression pattern to match against the full URI
    /// * `replacement` - Replacement string, can include capture group references like $1, $2
    ///
    /// # Errors
    ///
    /// Returns an error if the pattern is not a valid regular expression
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::HrefRewriter;
    ///
    /// // Change protocol
    /// let rewriter = HrefRewriter::new("^http://", "https://").unwrap();
    ///
    /// // Redirect between domains with path preservation
    /// let rewriter = HrefRewriter::new(
    ///     r"^https://api\.old\.com/(.*)$",
    ///     "https://api.new.com/$1"
    /// ).unwrap();
    ///
    /// // Add subdomain
    /// let rewriter = HrefRewriter::new(
    ///     r"^https://example\.com/",
    ///     "https://www.example.com/"
    /// ).unwrap();
    /// ```
    pub fn new(
        pattern: impl AsRef<str>,
        replacement: impl Into<String>,
    ) -> Result<Self, regex::Error> {
        Ok(Self {
            pattern: Regex::new(pattern.as_ref())?,
            replacement: replacement.into(),
        })
    }
}

impl Rewriter for HrefRewriter {
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        let (mut parts, body) = request.into_parts();

        let uri_str = parts.uri.to_string();
        let new_uri_str = self.pattern.replace(&uri_str, &self.replacement);

        if new_uri_str != uri_str {
            parts.uri = new_uri_str
                .parse()
                .map_err(|_| RewriteError("Invalid URI after rewrite".to_string()))?;
        }

        Ok(Request::from_parts(parts, body))
    }
}

/// Rewriter that applies multiple rewriters in sequence
///
/// This rewriter chains two rewriters together, applying the first rewriter
/// and then passing its output to the second rewriter. This allows building
/// complex transformation pipelines.
///
/// SequenceRewriter is typically created using the [`RewriterExt::then`] method
/// rather than directly.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, SequenceRewriter, PathRewriter, MethodRewriter};
/// use http::{Request, Method};
///
/// // Create a sequence manually
/// let path_rewriter = PathRewriter::new("^/old/", "/new/").unwrap();
/// let method_rewriter = MethodRewriter::new(Method::POST).unwrap();
/// let sequence = SequenceRewriter::new(path_rewriter, method_rewriter);
///
/// let request = Request::builder()
///     .method(Method::GET)
///     .uri("/old/api/users")
///     .body(())
///     .unwrap();
///
/// let result = sequence.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/new/api/users");
/// assert_eq!(result.method(), Method::POST);
/// ```
///
/// ```
/// use http_rewriter::{Rewriter, RewriterExt, PathRewriter, HeaderRewriter};
/// use http::Request;
///
/// // Using the then() method for cleaner syntax
/// let rewriter = PathRewriter::new("^/api/v1/", "/api/v2/").unwrap()
///     .then(HeaderRewriter::new("X-API-Version", ".*", "2.0").unwrap());
///
/// let request = Request::builder()
///     .uri("/api/v1/users")
///     .header("X-API-Version", "1.0")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/api/v2/users");
/// assert_eq!(result.headers().get("x-api-version").unwrap(), "2.0");
/// ```
pub struct SequenceRewriter<R1, R2> {
    first: R1,
    second: R2,
}

impl<R1: Rewriter, R2: Rewriter> SequenceRewriter<R1, R2> {
    /// Create a new sequence rewriter that applies two rewriters in order
    ///
    /// # Arguments
    ///
    /// * `first` - The first rewriter to apply
    /// * `second` - The second rewriter to apply to the result of the first
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{SequenceRewriter, PathRewriter, MethodRewriter};
    /// use http::Method;
    ///
    /// let sequence = SequenceRewriter::new(
    ///     PathRewriter::new("/old/", "/new/").unwrap(),
    ///     MethodRewriter::new(Method::POST).unwrap()
    /// );
    /// ```
    pub fn new(first: R1, second: R2) -> Self {
        Self { first, second }
    }
}

impl<R1: Rewriter, R2: Rewriter> Rewriter for SequenceRewriter<R1, R2> {
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        let request = self.first.rewrite(request)?;
        self.second.rewrite(request)
    }
}

/// Implementation of Rewriter for closures that transform requests
///
/// Any closure that takes a `Request<()>` and returns
/// `Result<Request<()>, RewriteError>` can be used as a rewriter.
/// This allows for custom transformation logic without creating a new type.
///
/// The closure receives the full request (with empty body) and can modify any
/// component. The original request body is preserved and reattached after transformation.
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, RewriteError};
/// use http::Request;
///
/// // Add a custom header to all requests
/// let add_header = |mut request: Request<()>| -> Result<Request<()>, RewriteError> {
///     request.headers_mut().insert(
///         "X-Processed-By",
///         "custom-rewriter".parse().unwrap()
///     );
///     Ok(request)
/// };
///
/// let request = Request::builder()
///     .uri("/api/data")
///     .body("request body")
///     .unwrap();
///
/// let result = add_header.rewrite(request).unwrap();
/// assert_eq!(result.headers().get("x-processed-by").unwrap(), "custom-rewriter");
/// assert_eq!(result.body(), &"request body");
/// ```
///
/// ```
/// use http_rewriter::{Rewriter, RewriteError, RewriterExt};
/// use http::{Request, Method};
///
/// // Complex transformation with error handling
/// let api_transformer = |mut request: Request<()>| -> Result<Request<()>, RewriteError> {
///     // Add API key header
///     request.headers_mut().insert("X-API-Key", "secret-key".parse().unwrap());
///
///     // Force JSON content type for POST/PUT
///     if request.method() == Method::POST || request.method() == Method::PUT {
///         request.headers_mut().insert(
///             "Content-Type",
///             "application/json".parse().unwrap()
///         );
///     }
///
///     // Validate path
///     if request.uri().path().contains("..") {
///         return Err(RewriteError::new("Invalid path: contains .."));
///     }
///
///     Ok(request)
/// };
///
/// // Chain with other rewriters
/// use http_rewriter::PathRewriter;
/// let full_rewriter = PathRewriter::new("^/v1/", "/v2/").unwrap()
///     .then(api_transformer);
///
/// let request = Request::builder()
///     .method(Method::POST)
///     .uri("/v1/users")
///     .body("request body")
///     .unwrap();
///
/// let result = full_rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/v2/users");
/// assert_eq!(result.headers().get("x-api-key").unwrap(), "secret-key");
/// assert_eq!(result.headers().get("content-type").unwrap(), "application/json");
/// assert_eq!(result.body(), &"request body");
/// ```
impl<F> Rewriter for F
where
    F: Fn(Request<()>) -> Result<Request<()>, RewriteError> + Send + Sync,
{
    fn rewrite<B>(&self, request: Request<B>) -> Result<Request<B>, RewriteError> {
        let (parts, body) = request.into_parts();

        // Create a Request<()> for the closure
        let empty_request = Request::from_parts(parts, ());

        // Apply the transformation
        let transformed_request = self(empty_request)?;

        // Extract the transformed parts and reattach the original body
        let (new_parts, _) = transformed_request.into_parts();
        Ok(Request::from_parts(new_parts, body))
    }
}

/// Extension trait for chaining rewriters
///
/// This trait provides convenient methods for composing rewriters.
/// It's automatically implemented for all types that implement [`Rewriter`].
///
/// # Examples
///
/// ```
/// use http_rewriter::{Rewriter, RewriterExt, PathRewriter, MethodRewriter, HeaderRewriter};
/// use http::{Request, Method};
///
/// // Chain multiple rewriters together
/// let rewriter = PathRewriter::new("^/api/v1/", "/api/v2/").unwrap()
///     .then(MethodRewriter::new(Method::POST).unwrap())
///     .then(HeaderRewriter::new("X-API-Version", ".*", "2.0").unwrap());
///
/// let request = Request::builder()
///     .method(Method::GET)
///     .uri("/api/v1/users")
///     .header("X-API-Version", "1.0")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/api/v2/users");
/// assert_eq!(result.method(), Method::POST);
/// assert_eq!(result.headers().get("x-api-version").unwrap(), "2.0");
/// ```
///
/// ```
/// use http_rewriter::{Rewriter, RewriterExt, PathRewriter, RewriteError};
/// use http::Request;
///
/// // Mix standard rewriters with closures
/// let add_auth = |mut request: Request<()>| -> Result<Request<()>, RewriteError> {
///     request.headers_mut().insert("Authorization", "Bearer token123".parse().unwrap());
///     Ok(request)
/// };
///
/// let rewriter = PathRewriter::new("^/public/", "/api/").unwrap()
///     .then(add_auth);
///
/// let request = Request::builder()
///     .uri("/public/data")
///     .body(())
///     .unwrap();
///
/// let result = rewriter.rewrite(request).unwrap();
/// assert_eq!(result.uri().path(), "/api/data");
/// assert_eq!(result.headers().get("authorization").unwrap(), "Bearer token123");
/// ```
pub trait RewriterExt: Rewriter + Sized {
    /// Chain this rewriter with another, creating a sequence that applies both in order
    ///
    /// The second rewriter receives the output of the first rewriter.
    /// If either rewriter returns an error, the error is propagated.
    ///
    /// # Arguments
    ///
    /// * `other` - The rewriter to apply after this one
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{RewriterExt, PathRewriter, MethodRewriter};
    /// use http::Method;
    ///
    /// // Create a rewriter pipeline
    /// let pipeline = PathRewriter::new("/old/", "/new/").unwrap()
    ///     .then(MethodRewriter::new(Method::POST).unwrap());
    /// ```
    fn then<R: Rewriter>(self, other: R) -> SequenceRewriter<Self, R> {
        SequenceRewriter::new(self, other)
    }

    /// Apply this rewriter conditionally based on a condition
    ///
    /// Creates a [`ConditionalRewriter`] that only applies this rewriter's
    /// transformations when the given condition matches the request.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition that determines when to apply this rewriter
    ///
    /// # Examples
    ///
    /// ```
    /// use http_rewriter::{
    ///     RewriterExt, PathRewriter, MethodCondition
    /// };
    /// use http::Method;
    ///
    /// // Only rewrite POST requests
    /// let rewriter = PathRewriter::new("/old/", "/new/").unwrap()
    ///     .when(MethodCondition::new(Method::POST)
    ///         .expect("Method::POST is always valid"));
    /// ```
    fn when<C: Condition>(self, condition: C) -> ConditionalRewriter<Self, C>;
}

impl<T: Rewriter> RewriterExt for T {
    fn when<C: Condition>(self, condition: C) -> ConditionalRewriter<Self, C> {
        ConditionalRewriter::new(self, condition)
    }
}

use std::ops::Deref;

use ::napi::{Error, Result, Status};
use napi_derive::napi;

pub use http_handler::napi::NapiRequest;

//
// Conditions
//

use crate::{
    Condition, ExistenceCondition, HeaderCondition, MethodCondition, NonExistenceCondition,
    PathCondition,
};

/// A N-API wrapper for the `PathCondition` type.
#[napi(js_name = "PathCondition")]
pub struct NapiPathCondition(PathCondition);

#[napi]
impl NapiPathCondition {
    /// Create a new path condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new PathCondition('/path/to/resource');
    /// ```
    #[napi(constructor)]
    pub fn new(pattern: String) -> Result<Self> {
        let condition = PathCondition::new(pattern)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(NapiPathCondition(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: NapiRequest) -> Result<bool> {
        Ok(self.0.matches(request.deref()))
    }
}

/// A N-API wrapper for the `HeaderCondition` type.
#[napi(js_name = "HeaderCondition")]
pub struct NapiHeaderCondition(HeaderCondition);

#[napi]
impl NapiHeaderCondition {
    /// Create a new header condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new HeaderCondition('Content-Type', 'application/json');
    /// ```
    #[napi(constructor)]
    pub fn new(header: String, value: String) -> Result<Self> {
        let condition = HeaderCondition::new(header, value)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(NapiHeaderCondition(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: NapiRequest) -> Result<bool> {
        Ok(self.0.matches(request.deref()))
    }
}

/// A N-API wrapper for the `MethodCondition` type.
#[napi(js_name = "MethodCondition")]
pub struct NapiMethodCondition(MethodCondition);

#[napi]
impl NapiMethodCondition {
    /// Create a new method condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new MethodCondition('GET');
    /// ```
    #[napi(constructor)]
    pub fn new(method: String) -> Result<Self> {
        let condition = MethodCondition::new(method.as_str())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(NapiMethodCondition(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: NapiRequest) -> Result<bool> {
        Ok(self.0.matches(request.deref()))
    }
}

/// A N-API wrapper for the `ExistenceCondition` type.
#[napi(js_name = "ExistenceCondition")]
pub struct NapiExistenceCondition(ExistenceCondition);

#[napi]
impl NapiExistenceCondition {
    /// Create a new existence condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new ExistenceCondition();
    /// ```
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        let condition = ExistenceCondition::new();
        Ok(NapiExistenceCondition(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: &NapiRequest) -> Result<bool> {
        Ok(self.0.matches(request))
    }
}

/// A N-API wrapper for the `NonExistenceCondition` type.
#[napi(js_name = "NonExistenceCondition")]
pub struct NapiNonExistenceCondition(NonExistenceCondition);

#[napi]
impl NapiNonExistenceCondition {
    /// Create a new non-existence condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new NonExistenceCondition();
    /// ```
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        let condition = NonExistenceCondition::new();
        Ok(NapiNonExistenceCondition(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: &NapiRequest) -> Result<bool> {
        Ok(self.0.matches(request))
    }
}

//
// Rewriters
//

use crate::{HeaderRewriter, MethodRewriter, PathRewriter, Rewriter};

/// A N-API wrapper for the `PathRewriter` type.
#[napi(js_name = "PathRewriter")]
pub struct NapiPathRewriter(PathRewriter);

#[napi]
impl NapiPathRewriter {
    /// Create a new path rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new PathRewriter();
    /// ```
    #[napi(constructor)]
    pub fn new(pattern: String, replacement: String) -> Result<Self> {
        let rewriter = PathRewriter::new(pattern, replacement)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(NapiPathRewriter(rewriter))
    }

    /// Rewrite the given path.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite('/path/to/resource');
    /// ```
    #[napi]
    pub fn rewrite(&self, request: NapiRequest) -> Result<NapiRequest> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

/// A N-API wrapper for the `HeaderRewriter` type.
#[napi(js_name = "HeaderRewriter")]
pub struct NapiHeaderRewriter(HeaderRewriter);

#[napi]
impl NapiHeaderRewriter {
    /// Create a new header rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new HeaderRewriter();
    /// ```
    #[napi(constructor)]
    pub fn new(header: String, pattern: String, replacement: String) -> Result<Self> {
        let rewriter = HeaderRewriter::new(header, pattern, replacement)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(NapiHeaderRewriter(rewriter))
    }

    /// Rewrite the given request headers.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite(request);
    /// ```
    #[napi]
    pub fn rewrite(&self, request: NapiRequest) -> Result<NapiRequest> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

/// A N-API wrapper for the `MethodRewriter` type.
#[napi(js_name = "MethodRewriter")]
pub struct NapiMethodRewriter(MethodRewriter);

#[napi]
impl NapiMethodRewriter {
    /// Create a new method rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new MethodRewriter();
    /// ```
    #[napi(constructor)]
    pub fn new(method: String) -> Result<Self> {
        let rewriter = MethodRewriter::new(method.as_str())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(NapiMethodRewriter(rewriter))
    }

    /// Rewrite the given request method.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite(request);
    /// ```
    #[napi]
    pub fn rewrite(&self, request: NapiRequest) -> Result<NapiRequest> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

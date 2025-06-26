use std::ops::Deref;

use ::napi::bindgen_prelude::{Either5, Either6};
use ::napi::{Error, Result, Status};
use napi_derive::napi;

use http_handler::napi::Request;

//
// Conditions
//

use crate::{Condition, ConditionExt};

/// A N-API wrapper for the `PathCondition` type.
#[napi]
#[derive(Clone)]
pub struct PathCondition(crate::PathCondition);

#[napi]
impl PathCondition {
    /// Create a new path condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new PathCondition('/path/to/resource');
    /// ```
    #[napi(constructor)]
    pub fn new(pattern: String) -> Result<Self> {
        let condition = crate::PathCondition::new(pattern)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: Request) -> Result<bool> {
        Ok(self.0.matches(request.deref()))
    }
}

/// A N-API wrapper for the `HeaderCondition` type.
#[napi]
#[derive(Clone)]
pub struct HeaderCondition(crate::HeaderCondition);

#[napi]
impl HeaderCondition {
    /// Create a new header condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new HeaderCondition('Content-Type', 'application/json');
    /// ```
    #[napi(constructor)]
    pub fn new(header: String, value: String) -> Result<Self> {
        let condition = crate::HeaderCondition::new(header, value)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: Request) -> Result<bool> {
        Ok(self.0.matches(request.deref()))
    }
}

/// A N-API wrapper for the `MethodCondition` type.
#[napi]
#[derive(Clone)]
pub struct MethodCondition(crate::MethodCondition);

#[napi]
impl MethodCondition {
    /// Create a new method condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new MethodCondition('GET');
    /// ```
    #[napi(constructor)]
    pub fn new(method: String) -> Result<Self> {
        let condition = crate::MethodCondition::new(method.as_str())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self(condition))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: Request) -> Result<bool> {
        Ok(self.0.matches(request.deref()))
    }
}

/// A N-API wrapper for the `ExistenceCondition` type.
#[napi]
#[derive(Clone, Copy)]
pub struct ExistenceCondition(crate::ExistenceCondition);

#[napi]
impl ExistenceCondition {
    /// Create a new existence condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new ExistenceCondition();
    /// ```
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        Ok(Self(crate::ExistenceCondition::new()))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: &Request) -> Result<bool> {
        Ok(self.0.matches(request))
    }
}

/// A N-API wrapper for the `NonExistenceCondition` type.
#[napi]
#[derive(Clone, Copy)]
pub struct NonExistenceCondition(crate::NonExistenceCondition);

#[napi]
impl NonExistenceCondition {
    /// Create a new non-existence condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const condition = new NonExistenceCondition();
    /// ```
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        Ok(Self(crate::NonExistenceCondition::new()))
    }

    /// Check if the given request matches the condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: &Request) -> Result<bool> {
        Ok(self.0.matches(request))
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
enum GroupConditionType {
    Path_Path(crate::GroupCondition<crate::PathCondition, crate::PathCondition>),
    Path_Header(crate::GroupCondition<crate::PathCondition, crate::HeaderCondition>),
    Path_Method(crate::GroupCondition<crate::PathCondition, crate::MethodCondition>),
    Path_Existence(crate::GroupCondition<crate::PathCondition, crate::ExistenceCondition>),
    Path_NonExistence(crate::GroupCondition<crate::PathCondition, crate::NonExistenceCondition>),
    Path_Group(crate::GroupCondition<crate::PathCondition, GroupConditionType>),
    Header_Path(crate::GroupCondition<crate::HeaderCondition, crate::PathCondition>),
    Header_Header(crate::GroupCondition<crate::HeaderCondition, crate::HeaderCondition>),
    Header_Method(crate::GroupCondition<crate::HeaderCondition, crate::MethodCondition>),
    Header_Existence(crate::GroupCondition<crate::HeaderCondition, crate::ExistenceCondition>),
    Header_NonExistence(
        crate::GroupCondition<crate::HeaderCondition, crate::NonExistenceCondition>,
    ),
    Header_Group(crate::GroupCondition<crate::HeaderCondition, GroupConditionType>),
    Method_Path(crate::GroupCondition<crate::MethodCondition, crate::PathCondition>),
    Method_Header(crate::GroupCondition<crate::MethodCondition, crate::HeaderCondition>),
    Method_Method(crate::GroupCondition<crate::MethodCondition, crate::MethodCondition>),
    Method_Existence(crate::GroupCondition<crate::MethodCondition, crate::ExistenceCondition>),
    Method_NonExistence(
        crate::GroupCondition<crate::MethodCondition, crate::NonExistenceCondition>,
    ),
    Method_Group(crate::GroupCondition<crate::MethodCondition, GroupConditionType>),
    Existence_Path(crate::GroupCondition<crate::ExistenceCondition, crate::PathCondition>),
    Existence_Header(crate::GroupCondition<crate::ExistenceCondition, crate::HeaderCondition>),
    Existence_Method(crate::GroupCondition<crate::ExistenceCondition, crate::MethodCondition>),
    Existence_Existence(
        crate::GroupCondition<crate::ExistenceCondition, crate::ExistenceCondition>,
    ),
    Existence_NonExistence(
        crate::GroupCondition<crate::ExistenceCondition, crate::NonExistenceCondition>,
    ),
    Existence_Group(crate::GroupCondition<crate::ExistenceCondition, GroupConditionType>),
    NonExistence_Path(crate::GroupCondition<crate::NonExistenceCondition, crate::PathCondition>),
    NonExistence_Header(
        crate::GroupCondition<crate::NonExistenceCondition, crate::HeaderCondition>,
    ),
    NonExistence_Method(
        crate::GroupCondition<crate::NonExistenceCondition, crate::MethodCondition>,
    ),
    NonExistence_Existence(
        crate::GroupCondition<crate::NonExistenceCondition, crate::ExistenceCondition>,
    ),
    NonExistence_NonExistence(
        crate::GroupCondition<crate::NonExistenceCondition, crate::NonExistenceCondition>,
    ),
    NonExistence_Group(crate::GroupCondition<crate::NonExistenceCondition, GroupConditionType>),
    Group_Path(crate::GroupCondition<GroupConditionType, crate::PathCondition>),
    Group_Header(crate::GroupCondition<GroupConditionType, crate::HeaderCondition>),
    Group_Method(crate::GroupCondition<GroupConditionType, crate::MethodCondition>),
    Group_Existence(crate::GroupCondition<GroupConditionType, crate::ExistenceCondition>),
    Group_NonExistence(crate::GroupCondition<GroupConditionType, crate::NonExistenceCondition>),
    Group_Group(crate::GroupCondition<GroupConditionType, GroupConditionType>),
}

impl Condition for GroupConditionType {
    fn matches<B>(&self, request: &http::Request<B>) -> bool {
        match self {
            GroupConditionType::Path_Path(c) => c.matches(request),
            GroupConditionType::Path_Header(c) => c.matches(request),
            GroupConditionType::Path_Method(c) => c.matches(request),
            GroupConditionType::Path_Existence(c) => c.matches(request),
            GroupConditionType::Path_NonExistence(c) => c.matches(request),
            GroupConditionType::Path_Group(c) => c.matches(request),
            GroupConditionType::Header_Path(c) => c.matches(request),
            GroupConditionType::Header_Header(c) => c.matches(request),
            GroupConditionType::Header_Method(c) => c.matches(request),
            GroupConditionType::Header_Existence(c) => c.matches(request),
            GroupConditionType::Header_NonExistence(c) => c.matches(request),
            GroupConditionType::Header_Group(c) => c.matches(request),
            GroupConditionType::Method_Path(c) => c.matches(request),
            GroupConditionType::Method_Header(c) => c.matches(request),
            GroupConditionType::Method_Method(c) => c.matches(request),
            GroupConditionType::Method_Existence(c) => c.matches(request),
            GroupConditionType::Method_NonExistence(c) => c.matches(request),
            GroupConditionType::Method_Group(c) => c.matches(request),
            GroupConditionType::Existence_Path(c) => c.matches(request),
            GroupConditionType::Existence_Header(c) => c.matches(request),
            GroupConditionType::Existence_Method(c) => c.matches(request),
            GroupConditionType::Existence_Existence(c) => c.matches(request),
            GroupConditionType::Existence_NonExistence(c) => c.matches(request),
            GroupConditionType::Existence_Group(c) => c.matches(request),
            GroupConditionType::NonExistence_Path(c) => c.matches(request),
            GroupConditionType::NonExistence_Header(c) => c.matches(request),
            GroupConditionType::NonExistence_Method(c) => c.matches(request),
            GroupConditionType::NonExistence_Existence(c) => c.matches(request),
            GroupConditionType::NonExistence_NonExistence(c) => c.matches(request),
            GroupConditionType::NonExistence_Group(c) => c.matches(request),
            GroupConditionType::Group_Path(c) => c.matches(request),
            GroupConditionType::Group_Header(c) => c.matches(request),
            GroupConditionType::Group_Method(c) => c.matches(request),
            GroupConditionType::Group_Existence(c) => c.matches(request),
            GroupConditionType::Group_NonExistence(c) => c.matches(request),
            GroupConditionType::Group_Group(c) => c.matches(request),
        }
    }
}

macro_rules! impl_from_group_condition {
    ($a:ty, $b:ty, $name:ident) => {
        impl From<crate::GroupCondition<$a, $b>> for GroupConditionType {
            fn from(condition: crate::GroupCondition<$a, $b>) -> Self {
                GroupConditionType::$name(condition)
            }
        }

        impl From<Box<crate::GroupCondition<$a, $b>>> for GroupConditionType {
            fn from(condition: Box<crate::GroupCondition<$a, $b>>) -> Self {
                GroupConditionType::$name(*condition)
            }
        }
    };
}

impl_from_group_condition!(crate::PathCondition, crate::PathCondition, Path_Path);
impl_from_group_condition!(crate::HeaderCondition, crate::PathCondition, Header_Path);
impl_from_group_condition!(crate::MethodCondition, crate::PathCondition, Method_Path);
impl_from_group_condition!(
    crate::ExistenceCondition,
    crate::PathCondition,
    Existence_Path
);
impl_from_group_condition!(
    crate::NonExistenceCondition,
    crate::PathCondition,
    NonExistence_Path
);
impl_from_group_condition!(GroupConditionType, crate::PathCondition, Group_Path);

impl_from_group_condition!(crate::PathCondition, crate::HeaderCondition, Path_Header);
impl_from_group_condition!(
    crate::HeaderCondition,
    crate::HeaderCondition,
    Header_Header
);
impl_from_group_condition!(
    crate::MethodCondition,
    crate::HeaderCondition,
    Method_Header
);
impl_from_group_condition!(
    crate::ExistenceCondition,
    crate::HeaderCondition,
    Existence_Header
);
impl_from_group_condition!(
    crate::NonExistenceCondition,
    crate::HeaderCondition,
    NonExistence_Header
);
impl_from_group_condition!(GroupConditionType, crate::HeaderCondition, Group_Header);

impl_from_group_condition!(crate::PathCondition, crate::MethodCondition, Path_Method);
impl_from_group_condition!(
    crate::HeaderCondition,
    crate::MethodCondition,
    Header_Method
);
impl_from_group_condition!(
    crate::MethodCondition,
    crate::MethodCondition,
    Method_Method
);
impl_from_group_condition!(
    crate::ExistenceCondition,
    crate::MethodCondition,
    Existence_Method
);
impl_from_group_condition!(
    crate::NonExistenceCondition,
    crate::MethodCondition,
    NonExistence_Method
);
impl_from_group_condition!(GroupConditionType, crate::MethodCondition, Group_Method);

impl_from_group_condition!(
    crate::PathCondition,
    crate::ExistenceCondition,
    Path_Existence
);
impl_from_group_condition!(
    crate::HeaderCondition,
    crate::ExistenceCondition,
    Header_Existence
);
impl_from_group_condition!(
    crate::MethodCondition,
    crate::ExistenceCondition,
    Method_Existence
);
impl_from_group_condition!(
    crate::ExistenceCondition,
    crate::ExistenceCondition,
    Existence_Existence
);
impl_from_group_condition!(
    crate::NonExistenceCondition,
    crate::ExistenceCondition,
    NonExistence_Existence
);
impl_from_group_condition!(
    GroupConditionType,
    crate::ExistenceCondition,
    Group_Existence
);

impl_from_group_condition!(
    crate::PathCondition,
    crate::NonExistenceCondition,
    Path_NonExistence
);
impl_from_group_condition!(
    crate::HeaderCondition,
    crate::NonExistenceCondition,
    Header_NonExistence
);
impl_from_group_condition!(
    crate::MethodCondition,
    crate::NonExistenceCondition,
    Method_NonExistence
);
impl_from_group_condition!(
    crate::ExistenceCondition,
    crate::NonExistenceCondition,
    Existence_NonExistence
);
impl_from_group_condition!(
    crate::NonExistenceCondition,
    crate::NonExistenceCondition,
    NonExistence_NonExistence
);
impl_from_group_condition!(
    GroupConditionType,
    crate::NonExistenceCondition,
    Group_NonExistence
);

impl_from_group_condition!(crate::PathCondition, GroupConditionType, Path_Group);
impl_from_group_condition!(crate::HeaderCondition, GroupConditionType, Header_Group);
impl_from_group_condition!(crate::MethodCondition, GroupConditionType, Method_Group);
impl_from_group_condition!(
    crate::ExistenceCondition,
    GroupConditionType,
    Existence_Group
);
impl_from_group_condition!(
    crate::NonExistenceCondition,
    GroupConditionType,
    NonExistence_Group
);
impl_from_group_condition!(GroupConditionType, GroupConditionType, Group_Group);

/// A N-API wrapper for the `GroupCondition` type.
#[napi]
pub struct GroupCondition(GroupConditionType);

#[napi]
impl GroupCondition {
    /// Check if the given request matches the group condition.
    ///
    /// # Examples
    ///
    /// ```js
    /// const matches = condition.matches(request);
    /// ```
    #[napi]
    pub fn matches(&self, request: Request) -> Result<bool> {
        Ok(self.0.matches(request.deref()))
    }
}

/// Type alias for any condition type that can be passed to and/or methods
type AnyCondition<'a> = Either6<
    &'a PathCondition,
    &'a HeaderCondition,
    &'a MethodCondition,
    &'a ExistenceCondition,
    &'a NonExistenceCondition,
    &'a GroupCondition,
>;

macro_rules! impl_condition_combinators {
    ($type:ty) => {
        #[napi]
        impl $type {
            /// Create a new condition that matches when both conditions match
            ///
            /// # Examples
            ///
            /// ```js
            /// const combined = condition1.and(condition2);
            /// ```
            #[napi]
            pub fn and(&self, other: AnyCondition) -> Result<GroupCondition> {
                let this = self.0.clone();
                Ok(GroupCondition(match other {
                    Either6::A(path) => this.and(path.0.clone()).into(),
                    Either6::B(header) => this.and(header.0.clone()).into(),
                    Either6::C(method) => this.and(method.0.clone()).into(),
                    Either6::D(existence) => this.and(existence.0).into(),
                    Either6::E(nonexistence) => this.and(nonexistence.0).into(),
                    Either6::F(group) => this.and(group.0.clone()).into(),
                }))
            }

            /// Create a new condition that matches when either condition matches
            ///
            /// # Examples
            ///
            /// ```js
            /// const combined = condition1.or(condition2);
            /// ```
            #[napi]
            pub fn or(&self, other: AnyCondition) -> Result<GroupCondition> {
                let this = self.0.clone();
                Ok(GroupCondition(match other {
                    Either6::A(path) => this.or(path.0.clone()).into(),
                    Either6::B(header) => this.or(header.0.clone()).into(),
                    Either6::C(method) => this.or(method.0.clone()).into(),
                    Either6::D(existence) => this.or(existence.0).into(),
                    Either6::E(nonexistence) => this.or(nonexistence.0).into(),
                    Either6::F(group) => this.or(group.0.clone()).into(),
                }))
            }
        }
    };
}

impl_condition_combinators!(PathCondition);
impl_condition_combinators!(HeaderCondition);
impl_condition_combinators!(MethodCondition);
impl_condition_combinators!(ExistenceCondition);
impl_condition_combinators!(NonExistenceCondition);
impl_condition_combinators!(GroupCondition);

//
// Rewriters
//

use crate::{Rewriter, RewriterExt};

/// A N-API wrapper for the `PathRewriter` type.
#[napi]
#[derive(Clone)]
pub struct PathRewriter(crate::PathRewriter);

#[napi]
impl PathRewriter {
    /// Create a new path rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new PathRewriter();
    /// ```
    #[napi(constructor)]
    pub fn new(pattern: String, replacement: String) -> Result<Self> {
        let rewriter = crate::PathRewriter::new(pattern, replacement)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self(rewriter))
    }

    /// Rewrite the given path.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite('/path/to/resource');
    /// ```
    #[napi]
    pub fn rewrite(&self, request: Request) -> Result<Request> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

/// A N-API wrapper for the `HeaderRewriter` type.
#[napi]
#[derive(Clone)]
pub struct HeaderRewriter(crate::HeaderRewriter);

#[napi]
impl HeaderRewriter {
    /// Create a new header rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new HeaderRewriter();
    /// ```
    #[napi(constructor)]
    pub fn new(header: String, pattern: String, replacement: String) -> Result<Self> {
        let rewriter = crate::HeaderRewriter::new(header, pattern, replacement)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self(rewriter))
    }

    /// Rewrite the given request headers.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite(request);
    /// ```
    #[napi]
    pub fn rewrite(&self, request: Request) -> Result<Request> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

/// A N-API wrapper for the `MethodRewriter` type.
#[napi]
#[derive(Clone)]
pub struct MethodRewriter(crate::MethodRewriter);

#[napi]
impl MethodRewriter {
    /// Create a new method rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new MethodRewriter();
    /// ```
    #[napi(constructor)]
    pub fn new(method: String) -> Result<Self> {
        let rewriter = crate::MethodRewriter::new(method.as_str())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self(rewriter))
    }

    /// Rewrite the given request method.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite(request);
    /// ```
    #[napi]
    pub fn rewrite(&self, request: Request) -> Result<Request> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
enum SequenceRewriterType {
    Path_Path(crate::SequenceRewriter<crate::PathRewriter, crate::PathRewriter>),
    Path_Header(crate::SequenceRewriter<crate::PathRewriter, crate::HeaderRewriter>),
    Path_Method(crate::SequenceRewriter<crate::PathRewriter, crate::MethodRewriter>),
    Path_Sequence(crate::SequenceRewriter<crate::PathRewriter, SequenceRewriterType>),
    Path_Conditional(crate::SequenceRewriter<crate::PathRewriter, ConditionalRewriterType>),

    Header_Path(crate::SequenceRewriter<crate::HeaderRewriter, crate::PathRewriter>),
    Header_Header(crate::SequenceRewriter<crate::HeaderRewriter, crate::HeaderRewriter>),
    Header_Method(crate::SequenceRewriter<crate::HeaderRewriter, crate::MethodRewriter>),
    Header_Sequence(crate::SequenceRewriter<crate::HeaderRewriter, SequenceRewriterType>),
    Header_Conditional(crate::SequenceRewriter<crate::HeaderRewriter, ConditionalRewriterType>),

    Method_Path(crate::SequenceRewriter<crate::MethodRewriter, crate::PathRewriter>),
    Method_Header(crate::SequenceRewriter<crate::MethodRewriter, crate::HeaderRewriter>),
    Method_Method(crate::SequenceRewriter<crate::MethodRewriter, crate::MethodRewriter>),
    Method_Sequence(crate::SequenceRewriter<crate::MethodRewriter, SequenceRewriterType>),
    Method_Conditional(crate::SequenceRewriter<crate::MethodRewriter, ConditionalRewriterType>),

    // Sequences with sequences (for nested sequences)
    Sequence_Path(crate::SequenceRewriter<SequenceRewriterType, crate::PathRewriter>),
    Sequence_Header(crate::SequenceRewriter<SequenceRewriterType, crate::HeaderRewriter>),
    Sequence_Method(crate::SequenceRewriter<SequenceRewriterType, crate::MethodRewriter>),
    Sequence_Sequence(crate::SequenceRewriter<SequenceRewriterType, SequenceRewriterType>),
    Sequence_Conditional(crate::SequenceRewriter<SequenceRewriterType, ConditionalRewriterType>),

    // Conditional rewriters
    Conditional_Path(crate::SequenceRewriter<ConditionalRewriterType, crate::PathRewriter>),
    Conditional_Header(crate::SequenceRewriter<ConditionalRewriterType, crate::HeaderRewriter>),
    Conditional_Method(crate::SequenceRewriter<ConditionalRewriterType, crate::MethodRewriter>),
    Conditional_Sequence(crate::SequenceRewriter<ConditionalRewriterType, SequenceRewriterType>),
    Conditional_Conditional(
        crate::SequenceRewriter<ConditionalRewriterType, ConditionalRewriterType>,
    ),
}

impl Rewriter for SequenceRewriterType {
    fn rewrite<B>(
        &self,
        request: http::Request<B>,
    ) -> std::result::Result<http::Request<B>, crate::RewriteError> {
        match self {
            SequenceRewriterType::Path_Path(r) => r.rewrite(request),
            SequenceRewriterType::Path_Header(r) => r.rewrite(request),
            SequenceRewriterType::Path_Method(r) => r.rewrite(request),
            SequenceRewriterType::Path_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Path_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Header_Path(r) => r.rewrite(request),
            SequenceRewriterType::Header_Header(r) => r.rewrite(request),
            SequenceRewriterType::Header_Method(r) => r.rewrite(request),
            SequenceRewriterType::Header_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Header_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Method_Path(r) => r.rewrite(request),
            SequenceRewriterType::Method_Header(r) => r.rewrite(request),
            SequenceRewriterType::Method_Method(r) => r.rewrite(request),
            SequenceRewriterType::Method_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Method_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Sequence_Path(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Header(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Method(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Conditional_Path(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Header(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Method(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Conditional(r) => r.rewrite(request),
        }
    }
}

macro_rules! impl_from_sequence_rewriter {
    ($a:ty, $b:ty, $name:ident) => {
        impl From<crate::SequenceRewriter<$a, $b>> for SequenceRewriterType {
            fn from(rewriter: crate::SequenceRewriter<$a, $b>) -> Self {
                SequenceRewriterType::$name(rewriter)
            }
        }

        impl From<Box<crate::SequenceRewriter<$a, $b>>> for SequenceRewriterType {
            fn from(rewriter: Box<crate::SequenceRewriter<$a, $b>>) -> Self {
                SequenceRewriterType::$name(*rewriter)
            }
        }
    };
}

impl_from_sequence_rewriter!(crate::PathRewriter, crate::PathRewriter, Path_Path);
impl_from_sequence_rewriter!(crate::PathRewriter, crate::HeaderRewriter, Path_Header);
impl_from_sequence_rewriter!(crate::PathRewriter, crate::MethodRewriter, Path_Method);
impl_from_sequence_rewriter!(crate::PathRewriter, SequenceRewriterType, Path_Sequence);
impl_from_sequence_rewriter!(
    crate::PathRewriter,
    ConditionalRewriterType,
    Path_Conditional
);

impl_from_sequence_rewriter!(crate::HeaderRewriter, crate::PathRewriter, Header_Path);
impl_from_sequence_rewriter!(crate::HeaderRewriter, crate::HeaderRewriter, Header_Header);
impl_from_sequence_rewriter!(crate::HeaderRewriter, crate::MethodRewriter, Header_Method);
impl_from_sequence_rewriter!(crate::HeaderRewriter, SequenceRewriterType, Header_Sequence);
impl_from_sequence_rewriter!(
    crate::HeaderRewriter,
    ConditionalRewriterType,
    Header_Conditional
);

impl_from_sequence_rewriter!(crate::MethodRewriter, crate::PathRewriter, Method_Path);
impl_from_sequence_rewriter!(crate::MethodRewriter, crate::HeaderRewriter, Method_Header);
impl_from_sequence_rewriter!(crate::MethodRewriter, crate::MethodRewriter, Method_Method);
impl_from_sequence_rewriter!(crate::MethodRewriter, SequenceRewriterType, Method_Sequence);
impl_from_sequence_rewriter!(
    crate::MethodRewriter,
    ConditionalRewriterType,
    Method_Conditional
);

impl_from_sequence_rewriter!(SequenceRewriterType, crate::PathRewriter, Sequence_Path);
impl_from_sequence_rewriter!(SequenceRewriterType, crate::HeaderRewriter, Sequence_Header);
impl_from_sequence_rewriter!(SequenceRewriterType, crate::MethodRewriter, Sequence_Method);
impl_from_sequence_rewriter!(
    SequenceRewriterType,
    SequenceRewriterType,
    Sequence_Sequence
);
impl_from_sequence_rewriter!(
    SequenceRewriterType,
    ConditionalRewriterType,
    Sequence_Conditional
);

impl_from_sequence_rewriter!(
    ConditionalRewriterType,
    crate::PathRewriter,
    Conditional_Path
);
impl_from_sequence_rewriter!(
    ConditionalRewriterType,
    crate::HeaderRewriter,
    Conditional_Header
);
impl_from_sequence_rewriter!(
    ConditionalRewriterType,
    crate::MethodRewriter,
    Conditional_Method
);
impl_from_sequence_rewriter!(
    ConditionalRewriterType,
    SequenceRewriterType,
    Conditional_Sequence
);
impl_from_sequence_rewriter!(
    ConditionalRewriterType,
    ConditionalRewriterType,
    Conditional_Conditional
);

// Implementation of Rewriter for Box<SequenceRewriterType>
// TODO: Should SequenceRewriter just contain boxed values so we don't need this?
impl Rewriter for Box<SequenceRewriterType> {
    fn rewrite<B>(
        &self,
        request: http::Request<B>,
    ) -> std::result::Result<http::Request<B>, crate::RewriteError> {
        (**self).rewrite(request)
    }
}

/// A N-API wrapper for the `SequenceRewriter` type.
#[napi]
pub struct SequenceRewriter(SequenceRewriterType);

#[napi]
impl SequenceRewriter {
    /// Rewrite the given request using the sequence of rewriters.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite(request);
    /// ```
    #[napi]
    pub fn rewrite(&self, request: Request) -> Result<Request> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

/// Type alias for any rewriter type that can be passed to then/when methods
type AnyRewriter<'a> = ::napi::bindgen_prelude::Either5<
    &'a PathRewriter,
    &'a HeaderRewriter,
    &'a MethodRewriter,
    &'a SequenceRewriter,
    &'a ConditionalRewriter,
>;

// Since Rewriter and Condition traits have generic methods, we need to create
// a type-erased wrapper that can be used with ConditionalRewriter
#[allow(non_camel_case_types)]
#[derive(Clone)]
enum ConditionalRewriterType {
    Path_Path(crate::ConditionalRewriter<crate::PathRewriter, crate::PathCondition>),
    Path_Header(crate::ConditionalRewriter<crate::PathRewriter, crate::HeaderCondition>),
    Path_Method(crate::ConditionalRewriter<crate::PathRewriter, crate::MethodCondition>),
    Path_Existence(crate::ConditionalRewriter<crate::PathRewriter, crate::ExistenceCondition>),
    Path_NonExistence(
        crate::ConditionalRewriter<crate::PathRewriter, crate::NonExistenceCondition>,
    ),
    Path_Group(crate::ConditionalRewriter<crate::PathRewriter, GroupConditionType>),
    Header_Path(crate::ConditionalRewriter<crate::HeaderRewriter, crate::PathCondition>),
    Header_Header(crate::ConditionalRewriter<crate::HeaderRewriter, crate::HeaderCondition>),
    Header_Method(crate::ConditionalRewriter<crate::HeaderRewriter, crate::MethodCondition>),
    Header_Existence(crate::ConditionalRewriter<crate::HeaderRewriter, crate::ExistenceCondition>),
    Header_NonExistence(
        crate::ConditionalRewriter<crate::HeaderRewriter, crate::NonExistenceCondition>,
    ),
    Header_Group(crate::ConditionalRewriter<crate::HeaderRewriter, GroupConditionType>),
    Method_Path(crate::ConditionalRewriter<crate::MethodRewriter, crate::PathCondition>),
    Method_Header(crate::ConditionalRewriter<crate::MethodRewriter, crate::HeaderCondition>),
    Method_Method(crate::ConditionalRewriter<crate::MethodRewriter, crate::MethodCondition>),
    Method_Existence(crate::ConditionalRewriter<crate::MethodRewriter, crate::ExistenceCondition>),
    Method_NonExistence(
        crate::ConditionalRewriter<crate::MethodRewriter, crate::NonExistenceCondition>,
    ),
    Method_Group(crate::ConditionalRewriter<crate::MethodRewriter, GroupConditionType>),
    Sequence_Path(crate::ConditionalRewriter<SequenceRewriterType, crate::PathCondition>),
    Sequence_Header(crate::ConditionalRewriter<SequenceRewriterType, crate::HeaderCondition>),
    Sequence_Method(crate::ConditionalRewriter<SequenceRewriterType, crate::MethodCondition>),
    Sequence_Existence(crate::ConditionalRewriter<SequenceRewriterType, crate::ExistenceCondition>),
    Sequence_NonExistence(
        crate::ConditionalRewriter<SequenceRewriterType, crate::NonExistenceCondition>,
    ),
    Sequence_Group(crate::ConditionalRewriter<SequenceRewriterType, GroupConditionType>),
    Conditional_Path(crate::ConditionalRewriter<ConditionalRewriterType, crate::PathCondition>),
    Conditional_Header(crate::ConditionalRewriter<ConditionalRewriterType, crate::HeaderCondition>),
    Conditional_Method(crate::ConditionalRewriter<ConditionalRewriterType, crate::MethodCondition>),
    Conditional_Existence(
        crate::ConditionalRewriter<ConditionalRewriterType, crate::ExistenceCondition>,
    ),
    Conditional_NonExistence(
        crate::ConditionalRewriter<ConditionalRewriterType, crate::NonExistenceCondition>,
    ),
    Conditional_Group(crate::ConditionalRewriter<ConditionalRewriterType, GroupConditionType>),
}

impl Rewriter for ConditionalRewriterType {
    fn rewrite<B>(
        &self,
        request: http::Request<B>,
    ) -> std::result::Result<http::Request<B>, crate::RewriteError> {
        match self {
            ConditionalRewriterType::Path_Path(r) => r.rewrite(request),
            ConditionalRewriterType::Path_Header(r) => r.rewrite(request),
            ConditionalRewriterType::Path_Method(r) => r.rewrite(request),
            ConditionalRewriterType::Path_Existence(r) => r.rewrite(request),
            ConditionalRewriterType::Path_NonExistence(r) => r.rewrite(request),
            ConditionalRewriterType::Path_Group(r) => r.rewrite(request),
            ConditionalRewriterType::Header_Path(r) => r.rewrite(request),
            ConditionalRewriterType::Header_Header(r) => r.rewrite(request),
            ConditionalRewriterType::Header_Method(r) => r.rewrite(request),
            ConditionalRewriterType::Header_Existence(r) => r.rewrite(request),
            ConditionalRewriterType::Header_NonExistence(r) => r.rewrite(request),
            ConditionalRewriterType::Header_Group(r) => r.rewrite(request),
            ConditionalRewriterType::Method_Path(r) => r.rewrite(request),
            ConditionalRewriterType::Method_Header(r) => r.rewrite(request),
            ConditionalRewriterType::Method_Method(r) => r.rewrite(request),
            ConditionalRewriterType::Method_Existence(r) => r.rewrite(request),
            ConditionalRewriterType::Method_NonExistence(r) => r.rewrite(request),
            ConditionalRewriterType::Method_Group(r) => r.rewrite(request),
            ConditionalRewriterType::Sequence_Path(r) => r.rewrite(request),
            ConditionalRewriterType::Sequence_Header(r) => r.rewrite(request),
            ConditionalRewriterType::Sequence_Method(r) => r.rewrite(request),
            ConditionalRewriterType::Sequence_Existence(r) => r.rewrite(request),
            ConditionalRewriterType::Sequence_NonExistence(r) => r.rewrite(request),
            ConditionalRewriterType::Sequence_Group(r) => r.rewrite(request),
            ConditionalRewriterType::Conditional_Path(r) => r.rewrite(request),
            ConditionalRewriterType::Conditional_Header(r) => r.rewrite(request),
            ConditionalRewriterType::Conditional_Method(r) => r.rewrite(request),
            ConditionalRewriterType::Conditional_Existence(r) => r.rewrite(request),
            ConditionalRewriterType::Conditional_NonExistence(r) => r.rewrite(request),
            ConditionalRewriterType::Conditional_Group(r) => r.rewrite(request),
        }
    }
}

macro_rules! impl_from_conditional_rewriter {
    ($a:ty, $b:ty, $name:ident) => {
        impl From<crate::ConditionalRewriter<$a, $b>> for ConditionalRewriterType {
            fn from(rewriter: crate::ConditionalRewriter<$a, $b>) -> Self {
                ConditionalRewriterType::$name(rewriter)
            }
        }

        impl From<Box<crate::ConditionalRewriter<$a, $b>>> for ConditionalRewriterType {
            fn from(rewriter: Box<crate::ConditionalRewriter<$a, $b>>) -> Self {
                ConditionalRewriterType::$name(*rewriter)
            }
        }
    };
}

impl_from_conditional_rewriter!(crate::PathRewriter, crate::PathCondition, Path_Path);
impl_from_conditional_rewriter!(crate::PathRewriter, crate::HeaderCondition, Path_Header);
impl_from_conditional_rewriter!(crate::PathRewriter, crate::MethodCondition, Path_Method);
impl_from_conditional_rewriter!(
    crate::PathRewriter,
    crate::ExistenceCondition,
    Path_Existence
);
impl_from_conditional_rewriter!(
    crate::PathRewriter,
    crate::NonExistenceCondition,
    Path_NonExistence
);
impl_from_conditional_rewriter!(crate::PathRewriter, GroupConditionType, Path_Group);

impl_from_conditional_rewriter!(crate::HeaderRewriter, crate::PathCondition, Header_Path);
impl_from_conditional_rewriter!(crate::HeaderRewriter, crate::HeaderCondition, Header_Header);
impl_from_conditional_rewriter!(crate::HeaderRewriter, crate::MethodCondition, Header_Method);
impl_from_conditional_rewriter!(
    crate::HeaderRewriter,
    crate::ExistenceCondition,
    Header_Existence
);
impl_from_conditional_rewriter!(
    crate::HeaderRewriter,
    crate::NonExistenceCondition,
    Header_NonExistence
);
impl_from_conditional_rewriter!(crate::HeaderRewriter, GroupConditionType, Header_Group);

impl_from_conditional_rewriter!(crate::MethodRewriter, crate::PathCondition, Method_Path);
impl_from_conditional_rewriter!(crate::MethodRewriter, crate::HeaderCondition, Method_Header);
impl_from_conditional_rewriter!(crate::MethodRewriter, crate::MethodCondition, Method_Method);
impl_from_conditional_rewriter!(
    crate::MethodRewriter,
    crate::ExistenceCondition,
    Method_Existence
);
impl_from_conditional_rewriter!(
    crate::MethodRewriter,
    crate::NonExistenceCondition,
    Method_NonExistence
);
impl_from_conditional_rewriter!(crate::MethodRewriter, GroupConditionType, Method_Group);

impl_from_conditional_rewriter!(SequenceRewriterType, crate::PathCondition, Sequence_Path);
impl_from_conditional_rewriter!(
    SequenceRewriterType,
    crate::HeaderCondition,
    Sequence_Header
);
impl_from_conditional_rewriter!(
    SequenceRewriterType,
    crate::MethodCondition,
    Sequence_Method
);
impl_from_conditional_rewriter!(
    SequenceRewriterType,
    crate::ExistenceCondition,
    Sequence_Existence
);
impl_from_conditional_rewriter!(
    SequenceRewriterType,
    crate::NonExistenceCondition,
    Sequence_NonExistence
);
impl_from_conditional_rewriter!(SequenceRewriterType, GroupConditionType, Sequence_Group);

impl_from_conditional_rewriter!(
    ConditionalRewriterType,
    crate::PathCondition,
    Conditional_Path
);
impl_from_conditional_rewriter!(
    ConditionalRewriterType,
    crate::HeaderCondition,
    Conditional_Header
);
impl_from_conditional_rewriter!(
    ConditionalRewriterType,
    crate::MethodCondition,
    Conditional_Method
);
impl_from_conditional_rewriter!(
    ConditionalRewriterType,
    crate::ExistenceCondition,
    Conditional_Existence
);
impl_from_conditional_rewriter!(
    ConditionalRewriterType,
    crate::NonExistenceCondition,
    Conditional_NonExistence
);
impl_from_conditional_rewriter!(
    ConditionalRewriterType,
    GroupConditionType,
    Conditional_Group
);

// Implementation of Rewriter for Box<SequenceRewriterType>
// TODO: Should SequenceRewriter just contain boxed values so we don't need this?
impl Rewriter for Box<ConditionalRewriterType> {
    fn rewrite<B>(
        &self,
        request: http::Request<B>,
    ) -> std::result::Result<http::Request<B>, crate::RewriteError> {
        (**self).rewrite(request)
    }
}

/// A N-API wrapper for the `ConditionalRewriter` type.
#[napi]
pub struct ConditionalRewriter(ConditionalRewriterType);

#[napi]
impl ConditionalRewriter {
    /// Rewrite the given request if the condition matches.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite(request);
    /// ```
    #[napi]
    pub fn rewrite(&self, request: Request) -> Result<Request> {
        let rewritten = self
            .0
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

macro_rules! impl_rewriter_combinators {
    ($type:ty) => {
        #[napi]
        impl $type {
            /// Chain this rewriter with another, creating a sequence that applies both in order
            ///
            /// # Examples
            ///
            /// ```js
            /// const sequence = rewriter1.then(rewriter2);
            /// ```
            #[napi]
            pub fn then(&self, other: AnyRewriter) -> Result<SequenceRewriter> {
                let this = self.0.clone();
                Ok(SequenceRewriter(match other {
                    Either5::A(path) => this.then(path.0.clone()).into(),
                    Either5::B(header) => this.then(header.0.clone()).into(),
                    Either5::C(method) => this.then(method.0.clone()).into(),
                    Either5::D(sequence) => this.then(sequence.0.clone()).into(),
                    Either5::E(conditional) => this.then(conditional.0.clone()).into(),
                }))
            }

            /// Apply this rewriter conditionally based on a condition
            ///
            /// # Examples
            ///
            /// ```js
            /// const conditional = rewriter.when(condition);
            /// ```
            #[napi]
            pub fn when(&self, condition: AnyCondition) -> Result<ConditionalRewriter> {
                let this = self.0.clone();
                Ok(ConditionalRewriter(match condition {
                    Either6::A(path) => this.clone().when(path.0.clone()).into(),
                    Either6::B(header) => this.clone().when(header.0.clone()).into(),
                    Either6::C(method) => this.clone().when(method.0.clone()).into(),
                    Either6::D(existence) => this.clone().when(existence.0).into(),
                    Either6::E(nonexistence) => this.clone().when(nonexistence.0).into(),
                    Either6::F(group) => this.when(group.0.clone()).into(),
                }))
            }
        }
    };
}

impl_rewriter_combinators!(PathRewriter);
impl_rewriter_combinators!(HeaderRewriter);
impl_rewriter_combinators!(MethodRewriter);
impl_rewriter_combinators!(SequenceRewriter);
impl_rewriter_combinators!(ConditionalRewriter);

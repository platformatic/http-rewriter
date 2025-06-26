use std::ops::Deref;

use ::napi::bindgen_prelude::Either6;
use ::napi::{Error, Result, Status};
use napi_derive::napi;

use http_handler::napi::NapiRequest;

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
    pub fn matches(&self, request: NapiRequest) -> Result<bool> {
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
    pub fn matches(&self, request: NapiRequest) -> Result<bool> {
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
    pub fn matches(&self, request: NapiRequest) -> Result<bool> {
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
    pub fn matches(&self, request: &NapiRequest) -> Result<bool> {
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
    pub fn matches(&self, request: &NapiRequest) -> Result<bool> {
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
    pub fn matches(&self, request: NapiRequest) -> Result<bool> {
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

use std::ops::Deref;

use ::napi::bindgen_prelude::Either6;
use ::napi::{Error, Result, Status};
use napi_derive::napi;

use http_handler::napi::Request;

//
// Basic Conditions
//

use crate::{Condition as ConditionTrait, ConditionExt};

/// A N-API wrapper for the `PathCondition` type.
#[napi]
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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

//
// Complex Conditions
//

// Since Condition traits have generic methods, we need to create a type-erased
// wrapper that can be used with GroupCondition
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

impl crate::Condition for GroupConditionType {
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

// Implement `From` for each combination of GroupCondition
macro_rules! impl_from_group_condition {
    ($a:ty, $b:ty, $name:ident) => {
        impl From<crate::GroupCondition<$a, $b>> for GroupConditionType {
            fn from(condition: crate::GroupCondition<$a, $b>) -> Self {
                GroupConditionType::$name(condition)
            }
        }

        impl TryFrom<GroupConditionType> for crate::GroupCondition<$a, $b> {
            type Error = Error;

            fn try_from(value: GroupConditionType) -> Result<Self> {
                match value {
                    GroupConditionType::$name(c) => Ok(c),
                    _ => Err(Error::new(
                        Status::InvalidArg,
                        format!(
                            "Expected GroupConditionType::{}, found {:?}",
                            stringify!($name),
                            value
                        ),
                    )),
                }
            }
        }

        impl From<crate::GroupCondition<$a, $b>> for Condition {
            fn from(condition: crate::GroupCondition<$a, $b>) -> Self {
                Condition(Either6::F(condition.into()))
            }
        }

        impl TryFrom<Condition> for crate::GroupCondition<$a, $b> {
            type Error = Error;

            fn try_from(value: Condition) -> Result<Self> {
                match value.0 {
                    Either6::F(c) => c.try_into(),
                    _ => Err(Error::new(
                        Status::InvalidArg,
                        format!(
                            "Expected crate::GroupCondition<{}, {}>, found {:?}",
                            stringify!($a),
                            stringify!($b),
                            value
                        ),
                    )),
                }
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
#[derive(Clone, Debug)]
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

// Type alias for any condition which can be passed to `and`/`or` methods in JS
type AnyCondition<'a> = Either6<
    &'a PathCondition,
    &'a HeaderCondition,
    &'a MethodCondition,
    &'a ExistenceCondition,
    &'a NonExistenceCondition,
    &'a GroupCondition,
>;

// Type alias for any condition which can be passed to `and`/`or` methods in Rust
type AnyConditionOwned = Either6<
    crate::PathCondition,
    crate::HeaderCondition,
    crate::MethodCondition,
    crate::ExistenceCondition,
    crate::NonExistenceCondition,
    GroupConditionType,
>;

macro_rules! impl_from_condition {
    ($type:ty, $name:ident) => {
        impl From<$type> for Condition {
            fn from(condition: $type) -> Self {
                Condition(Either6::$name(condition))
            }
        }

        impl TryFrom<Condition> for $type {
            type Error = Error;

            fn try_from(value: Condition) -> Result<Self> {
                match value.0 {
                    Either6::$name(c) => Ok(c),
                    _ => Err(Error::new(
                        Status::InvalidArg,
                        format!("Expected Either6::{}, found {:?}", stringify!($name), value),
                    )),
                }
            }
        }
    };
}

impl_from_condition!(crate::PathCondition, A);
impl_from_condition!(crate::HeaderCondition, B);
impl_from_condition!(crate::MethodCondition, C);
impl_from_condition!(crate::ExistenceCondition, D);
impl_from_condition!(crate::NonExistenceCondition, E);
impl_from_condition!(GroupConditionType, F);

// Implement combinators for all condition types
//
// Provides:
// - `and` method to combine conditions with logical AND
// - `or` method to combine conditions with logical OR
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

/// Allows constructing rewriter and condition configurations from JSON.
#[derive(Clone, Debug)]
pub struct Condition(AnyConditionOwned);

impl crate::Condition for Condition {
    fn matches<B>(&self, request: &http::Request<B>) -> bool {
        match &self.0 {
            Either6::A(c) => c.matches(request),
            Either6::B(c) => c.matches(request),
            Either6::C(c) => c.matches(request),
            Either6::D(c) => c.matches(request),
            Either6::E(c) => c.matches(request),
            Either6::F(c) => c.matches(request),
        }
    }
}

impl TryFrom<ConditionConfig> for Condition {
    type Error = Error;

    fn try_from(config: ConditionConfig) -> Result<Self> {
        match config.condition {
            ConditionType::Path => {
                let path_condition = crate::PathCondition::try_from(config)
                    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
                Ok(path_condition.into())
            }
            ConditionType::Header => {
                let header_condition = crate::HeaderCondition::try_from(config)
                    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
                Ok(header_condition.into())
            }
            ConditionType::Method => {
                let method_condition = crate::MethodCondition::try_from(config)
                    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
                Ok(method_condition.into())
            }
            ConditionType::Exists => {
                let existence_condition = crate::ExistenceCondition::new();
                Ok(existence_condition.into())
            }
            ConditionType::NotExists => {
                let nonexistence_condition = crate::NonExistenceCondition::new();
                Ok(nonexistence_condition.into())
            }
        }
    }
}

fn and<A, B>(a: A, b: B) -> Condition
where
    A: Into<Condition>,
    B: Into<Condition>,
{
    match (a.into().0, b.into().0) {
        (Either6::A(a), Either6::A(b)) => a.and(b).into(),
        (Either6::A(a), Either6::B(b)) => a.and(b).into(),
        (Either6::A(a), Either6::C(b)) => a.and(b).into(),
        (Either6::A(a), Either6::D(b)) => a.and(b).into(),
        (Either6::A(a), Either6::E(b)) => a.and(b).into(),
        (Either6::A(a), Either6::F(b)) => a.and(b).into(),

        (Either6::B(a), Either6::A(b)) => a.and(b).into(),
        (Either6::B(a), Either6::B(b)) => a.and(b).into(),
        (Either6::B(a), Either6::C(b)) => a.and(b).into(),
        (Either6::B(a), Either6::D(b)) => a.and(b).into(),
        (Either6::B(a), Either6::E(b)) => a.and(b).into(),
        (Either6::B(a), Either6::F(b)) => a.and(b).into(),

        (Either6::C(a), Either6::A(b)) => a.and(b).into(),
        (Either6::C(a), Either6::B(b)) => a.and(b).into(),
        (Either6::C(a), Either6::C(b)) => a.and(b).into(),
        (Either6::C(a), Either6::D(b)) => a.and(b).into(),
        (Either6::C(a), Either6::E(b)) => a.and(b).into(),
        (Either6::C(a), Either6::F(b)) => a.and(b).into(),

        (Either6::D(a), Either6::A(b)) => a.and(b).into(),
        (Either6::D(a), Either6::B(b)) => a.and(b).into(),
        (Either6::D(a), Either6::C(b)) => a.and(b).into(),
        (Either6::D(a), Either6::D(b)) => a.and(b).into(),
        (Either6::D(a), Either6::E(b)) => a.and(b).into(),
        (Either6::D(a), Either6::F(b)) => a.and(b).into(),

        (Either6::E(a), Either6::A(b)) => a.and(b).into(),
        (Either6::E(a), Either6::B(b)) => a.and(b).into(),
        (Either6::E(a), Either6::C(b)) => a.and(b).into(),
        (Either6::E(a), Either6::D(b)) => a.and(b).into(),
        (Either6::E(a), Either6::E(b)) => a.and(b).into(),
        (Either6::E(a), Either6::F(b)) => a.and(b).into(),

        (Either6::F(a), Either6::A(b)) => a.and(b).into(),
        (Either6::F(a), Either6::B(b)) => a.and(b).into(),
        (Either6::F(a), Either6::C(b)) => a.and(b).into(),
        (Either6::F(a), Either6::D(b)) => a.and(b).into(),
        (Either6::F(a), Either6::E(b)) => a.and(b).into(),
        (Either6::F(a), Either6::F(b)) => a.and(b).into(),
    }
}

fn or<A, B>(a: A, b: B) -> Condition
where
    A: Into<Condition>,
    B: Into<Condition>,
{
    match (a.into().0, b.into().0) {
        (Either6::A(a), Either6::A(b)) => a.or(b).into(),
        (Either6::A(a), Either6::B(b)) => a.or(b).into(),
        (Either6::A(a), Either6::C(b)) => a.or(b).into(),
        (Either6::A(a), Either6::D(b)) => a.or(b).into(),
        (Either6::A(a), Either6::E(b)) => a.or(b).into(),
        (Either6::A(a), Either6::F(b)) => a.or(b).into(),

        (Either6::B(a), Either6::A(b)) => a.or(b).into(),
        (Either6::B(a), Either6::B(b)) => a.or(b).into(),
        (Either6::B(a), Either6::C(b)) => a.or(b).into(),
        (Either6::B(a), Either6::D(b)) => a.or(b).into(),
        (Either6::B(a), Either6::E(b)) => a.or(b).into(),
        (Either6::B(a), Either6::F(b)) => a.or(b).into(),

        (Either6::C(a), Either6::A(b)) => a.or(b).into(),
        (Either6::C(a), Either6::B(b)) => a.or(b).into(),
        (Either6::C(a), Either6::C(b)) => a.or(b).into(),
        (Either6::C(a), Either6::D(b)) => a.or(b).into(),
        (Either6::C(a), Either6::E(b)) => a.or(b).into(),
        (Either6::C(a), Either6::F(b)) => a.or(b).into(),

        (Either6::D(a), Either6::A(b)) => a.or(b).into(),
        (Either6::D(a), Either6::B(b)) => a.or(b).into(),
        (Either6::D(a), Either6::C(b)) => a.or(b).into(),
        (Either6::D(a), Either6::D(b)) => a.or(b).into(),
        (Either6::D(a), Either6::E(b)) => a.or(b).into(),
        (Either6::D(a), Either6::F(b)) => a.or(b).into(),

        (Either6::E(a), Either6::A(b)) => a.or(b).into(),
        (Either6::E(a), Either6::B(b)) => a.or(b).into(),
        (Either6::E(a), Either6::C(b)) => a.or(b).into(),
        (Either6::E(a), Either6::D(b)) => a.or(b).into(),
        (Either6::E(a), Either6::E(b)) => a.or(b).into(),
        (Either6::E(a), Either6::F(b)) => a.or(b).into(),

        (Either6::F(a), Either6::A(b)) => a.or(b).into(),
        (Either6::F(a), Either6::B(b)) => a.or(b).into(),
        (Either6::F(a), Either6::C(b)) => a.or(b).into(),
        (Either6::F(a), Either6::D(b)) => a.or(b).into(),
        (Either6::F(a), Either6::E(b)) => a.or(b).into(),
        (Either6::F(a), Either6::F(b)) => a.or(b).into(),
    }
}

//
// Rewriters
//

use crate::{Rewriter as RewriterTrait, RewriterExt};

/// A N-API wrapper for the `PathRewriter` type.
#[napi]
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

/// A N-API wrapper for the `HrefRewriter` type.
#[napi]
#[derive(Clone, Debug)]
pub struct HrefRewriter(crate::HrefRewriter);

#[napi]
impl HrefRewriter {
    /// Create a new href rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new HrefRewriter('^http://', 'https://');
    /// ```
    #[napi(constructor)]
    pub fn new(pattern: String, replacement: String) -> Result<Self> {
        let rewriter = crate::HrefRewriter::new(pattern, replacement)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(Self(rewriter))
    }

    /// Rewrite the given request href.
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

// Since Rewriter traits have generic methods, we need to create a type-erased
// wrapper that can be used with SequenceRewriter
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
enum SequenceRewriterType {
    Path_Path(crate::SequenceRewriter<crate::PathRewriter, crate::PathRewriter>),
    Path_Header(crate::SequenceRewriter<crate::PathRewriter, crate::HeaderRewriter>),
    Path_Method(crate::SequenceRewriter<crate::PathRewriter, crate::MethodRewriter>),
    Path_Href(crate::SequenceRewriter<crate::PathRewriter, crate::HrefRewriter>),
    Path_Sequence(crate::SequenceRewriter<crate::PathRewriter, SequenceRewriterType>),
    Path_Conditional(crate::SequenceRewriter<crate::PathRewriter, ConditionalRewriterType>),

    Header_Path(crate::SequenceRewriter<crate::HeaderRewriter, crate::PathRewriter>),
    Header_Header(crate::SequenceRewriter<crate::HeaderRewriter, crate::HeaderRewriter>),
    Header_Method(crate::SequenceRewriter<crate::HeaderRewriter, crate::MethodRewriter>),
    Header_Href(crate::SequenceRewriter<crate::HeaderRewriter, crate::HrefRewriter>),
    Header_Sequence(crate::SequenceRewriter<crate::HeaderRewriter, SequenceRewriterType>),
    Header_Conditional(crate::SequenceRewriter<crate::HeaderRewriter, ConditionalRewriterType>),

    Method_Path(crate::SequenceRewriter<crate::MethodRewriter, crate::PathRewriter>),
    Method_Header(crate::SequenceRewriter<crate::MethodRewriter, crate::HeaderRewriter>),
    Method_Method(crate::SequenceRewriter<crate::MethodRewriter, crate::MethodRewriter>),
    Method_Href(crate::SequenceRewriter<crate::MethodRewriter, crate::HrefRewriter>),
    Method_Sequence(crate::SequenceRewriter<crate::MethodRewriter, SequenceRewriterType>),
    Method_Conditional(crate::SequenceRewriter<crate::MethodRewriter, ConditionalRewriterType>),

    // Sequences with href
    Href_Path(crate::SequenceRewriter<crate::HrefRewriter, crate::PathRewriter>),
    Href_Header(crate::SequenceRewriter<crate::HrefRewriter, crate::HeaderRewriter>),
    Href_Method(crate::SequenceRewriter<crate::HrefRewriter, crate::MethodRewriter>),
    Href_Href(crate::SequenceRewriter<crate::HrefRewriter, crate::HrefRewriter>),
    Href_Sequence(crate::SequenceRewriter<crate::HrefRewriter, SequenceRewriterType>),
    Href_Conditional(crate::SequenceRewriter<crate::HrefRewriter, ConditionalRewriterType>),

    // Sequences with sequences (for nested sequences)
    Sequence_Path(crate::SequenceRewriter<SequenceRewriterType, crate::PathRewriter>),
    Sequence_Header(crate::SequenceRewriter<SequenceRewriterType, crate::HeaderRewriter>),
    Sequence_Method(crate::SequenceRewriter<SequenceRewriterType, crate::MethodRewriter>),
    Sequence_Href(crate::SequenceRewriter<SequenceRewriterType, crate::HrefRewriter>),
    Sequence_Sequence(crate::SequenceRewriter<SequenceRewriterType, SequenceRewriterType>),
    Sequence_Conditional(crate::SequenceRewriter<SequenceRewriterType, ConditionalRewriterType>),

    // Conditional rewriters
    Conditional_Path(crate::SequenceRewriter<ConditionalRewriterType, crate::PathRewriter>),
    Conditional_Header(crate::SequenceRewriter<ConditionalRewriterType, crate::HeaderRewriter>),
    Conditional_Method(crate::SequenceRewriter<ConditionalRewriterType, crate::MethodRewriter>),
    Conditional_Href(crate::SequenceRewriter<ConditionalRewriterType, crate::HrefRewriter>),
    Conditional_Sequence(crate::SequenceRewriter<ConditionalRewriterType, SequenceRewriterType>),
    Conditional_Conditional(
        crate::SequenceRewriter<ConditionalRewriterType, ConditionalRewriterType>,
    ),
}

impl crate::Rewriter for SequenceRewriterType {
    fn rewrite<B>(
        &self,
        request: http::Request<B>,
    ) -> std::result::Result<http::Request<B>, crate::RewriteError> {
        match self {
            SequenceRewriterType::Path_Path(r) => r.rewrite(request),
            SequenceRewriterType::Path_Header(r) => r.rewrite(request),
            SequenceRewriterType::Path_Method(r) => r.rewrite(request),
            SequenceRewriterType::Path_Href(r) => r.rewrite(request),
            SequenceRewriterType::Path_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Path_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Header_Path(r) => r.rewrite(request),
            SequenceRewriterType::Header_Header(r) => r.rewrite(request),
            SequenceRewriterType::Header_Method(r) => r.rewrite(request),
            SequenceRewriterType::Header_Href(r) => r.rewrite(request),
            SequenceRewriterType::Header_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Header_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Method_Path(r) => r.rewrite(request),
            SequenceRewriterType::Method_Header(r) => r.rewrite(request),
            SequenceRewriterType::Method_Method(r) => r.rewrite(request),
            SequenceRewriterType::Method_Href(r) => r.rewrite(request),
            SequenceRewriterType::Method_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Method_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Href_Path(r) => r.rewrite(request),
            SequenceRewriterType::Href_Header(r) => r.rewrite(request),
            SequenceRewriterType::Href_Method(r) => r.rewrite(request),
            SequenceRewriterType::Href_Href(r) => r.rewrite(request),
            SequenceRewriterType::Href_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Href_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Sequence_Path(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Header(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Method(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Href(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Sequence_Conditional(r) => r.rewrite(request),

            SequenceRewriterType::Conditional_Path(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Header(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Method(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Href(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Sequence(r) => r.rewrite(request),
            SequenceRewriterType::Conditional_Conditional(r) => {
                println!("yep: {:#?}", r);
                r.rewrite(request)
            }
        }
    }
}

// Implement `From` for each combination of SequenceRewriter
macro_rules! impl_from_sequence_rewriter {
    ($a:ty, $b:ty, $name:ident) => {
        impl From<crate::SequenceRewriter<$a, $b>> for SequenceRewriterType {
            fn from(rewriter: crate::SequenceRewriter<$a, $b>) -> Self {
                SequenceRewriterType::$name(rewriter)
            }
        }

        impl From<crate::SequenceRewriter<$a, $b>> for Rewriter {
            fn from(rewriter: crate::SequenceRewriter<$a, $b>) -> Self {
                Rewriter(Either6::E(rewriter.into()))
            }
        }
    };
}

impl_from_sequence_rewriter!(crate::PathRewriter, crate::PathRewriter, Path_Path);
impl_from_sequence_rewriter!(crate::PathRewriter, crate::HeaderRewriter, Path_Header);
impl_from_sequence_rewriter!(crate::PathRewriter, crate::MethodRewriter, Path_Method);
impl_from_sequence_rewriter!(crate::PathRewriter, crate::HrefRewriter, Path_Href);
impl_from_sequence_rewriter!(crate::PathRewriter, SequenceRewriterType, Path_Sequence);
impl_from_sequence_rewriter!(
    crate::PathRewriter,
    ConditionalRewriterType,
    Path_Conditional
);

impl_from_sequence_rewriter!(crate::HeaderRewriter, crate::PathRewriter, Header_Path);
impl_from_sequence_rewriter!(crate::HeaderRewriter, crate::HeaderRewriter, Header_Header);
impl_from_sequence_rewriter!(crate::HeaderRewriter, crate::MethodRewriter, Header_Method);
impl_from_sequence_rewriter!(crate::HeaderRewriter, crate::HrefRewriter, Header_Href);
impl_from_sequence_rewriter!(crate::HeaderRewriter, SequenceRewriterType, Header_Sequence);
impl_from_sequence_rewriter!(
    crate::HeaderRewriter,
    ConditionalRewriterType,
    Header_Conditional
);

impl_from_sequence_rewriter!(crate::MethodRewriter, crate::PathRewriter, Method_Path);
impl_from_sequence_rewriter!(crate::MethodRewriter, crate::HeaderRewriter, Method_Header);
impl_from_sequence_rewriter!(crate::MethodRewriter, crate::MethodRewriter, Method_Method);
impl_from_sequence_rewriter!(crate::MethodRewriter, crate::HrefRewriter, Method_Href);
impl_from_sequence_rewriter!(crate::MethodRewriter, SequenceRewriterType, Method_Sequence);
impl_from_sequence_rewriter!(
    crate::MethodRewriter,
    ConditionalRewriterType,
    Method_Conditional
);

impl_from_sequence_rewriter!(crate::HrefRewriter, crate::PathRewriter, Href_Path);
impl_from_sequence_rewriter!(crate::HrefRewriter, crate::HeaderRewriter, Href_Header);
impl_from_sequence_rewriter!(crate::HrefRewriter, crate::MethodRewriter, Href_Method);
impl_from_sequence_rewriter!(crate::HrefRewriter, crate::HrefRewriter, Href_Href);
impl_from_sequence_rewriter!(crate::HrefRewriter, SequenceRewriterType, Href_Sequence);
impl_from_sequence_rewriter!(
    crate::HrefRewriter,
    ConditionalRewriterType,
    Href_Conditional
);

impl_from_sequence_rewriter!(SequenceRewriterType, crate::PathRewriter, Sequence_Path);
impl_from_sequence_rewriter!(SequenceRewriterType, crate::HeaderRewriter, Sequence_Header);
impl_from_sequence_rewriter!(SequenceRewriterType, crate::MethodRewriter, Sequence_Method);
impl_from_sequence_rewriter!(SequenceRewriterType, crate::HrefRewriter, Sequence_Href);
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
    crate::HrefRewriter,
    Conditional_Href
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

/// A N-API wrapper for the `SequenceRewriter` type.
#[napi]
#[derive(Clone, Debug)]
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

// Since Rewriter and Condition traits have generic methods, we need to create
// a type-erased wrapper that can be used with ConditionalRewriter
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
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
    Href_Path(crate::ConditionalRewriter<crate::HrefRewriter, crate::PathCondition>),
    Href_Header(crate::ConditionalRewriter<crate::HrefRewriter, crate::HeaderCondition>),
    Href_Method(crate::ConditionalRewriter<crate::HrefRewriter, crate::MethodCondition>),
    Href_Existence(crate::ConditionalRewriter<crate::HrefRewriter, crate::ExistenceCondition>),
    Href_NonExistence(
        crate::ConditionalRewriter<crate::HrefRewriter, crate::NonExistenceCondition>,
    ),
    Href_Group(crate::ConditionalRewriter<crate::HrefRewriter, GroupConditionType>),
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

impl crate::Rewriter for ConditionalRewriterType {
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
            ConditionalRewriterType::Href_Path(r) => r.rewrite(request),
            ConditionalRewriterType::Href_Header(r) => r.rewrite(request),
            ConditionalRewriterType::Href_Method(r) => r.rewrite(request),
            ConditionalRewriterType::Href_Existence(r) => r.rewrite(request),
            ConditionalRewriterType::Href_NonExistence(r) => r.rewrite(request),
            ConditionalRewriterType::Href_Group(r) => r.rewrite(request),
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

// Implement `From` for each combination of ConditionalRewriter
macro_rules! impl_from_conditional_rewriter {
    ($a:ty, $b:ty, $name:ident) => {
        impl From<crate::ConditionalRewriter<$a, $b>> for ConditionalRewriterType {
            fn from(rewriter: crate::ConditionalRewriter<$a, $b>) -> Self {
                ConditionalRewriterType::$name(rewriter)
            }
        }

        impl From<crate::ConditionalRewriter<$a, $b>> for Rewriter {
            fn from(rewriter: crate::ConditionalRewriter<$a, $b>) -> Self {
                Rewriter(Either6::F(rewriter.into()))
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

impl_from_conditional_rewriter!(crate::HrefRewriter, crate::PathCondition, Href_Path);
impl_from_conditional_rewriter!(crate::HrefRewriter, crate::HeaderCondition, Href_Header);
impl_from_conditional_rewriter!(crate::HrefRewriter, crate::MethodCondition, Href_Method);
impl_from_conditional_rewriter!(
    crate::HrefRewriter,
    crate::ExistenceCondition,
    Href_Existence
);
impl_from_conditional_rewriter!(
    crate::HrefRewriter,
    crate::NonExistenceCondition,
    Href_NonExistence
);
impl_from_conditional_rewriter!(crate::HrefRewriter, GroupConditionType, Href_Group);

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

/// A N-API wrapper for the `ConditionalRewriter` type.
#[napi]
#[derive(Clone, Debug)]
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

/// Type alias for any rewriter which can be passed to `then`/`when` methods in JS
type AnyRewriter<'a> = Either6<
    &'a PathRewriter,
    &'a HeaderRewriter,
    &'a MethodRewriter,
    &'a HrefRewriter,
    &'a SequenceRewriter,
    &'a ConditionalRewriter,
>;

// Type alias for any rewriter which can be passed to `then`/`when` methods in Rust
type AnyRewriterOwned = Either6<
    crate::PathRewriter,
    crate::HeaderRewriter,
    crate::MethodRewriter,
    crate::HrefRewriter,
    SequenceRewriterType,
    ConditionalRewriterType,
>;

macro_rules! impl_from_rewriter {
    ($type:ty, $variant:ident) => {
        impl From<$type> for Rewriter {
            fn from(rewriter: $type) -> Self {
                Self(Either6::$variant(rewriter))
            }
        }
    };
}

impl_from_rewriter!(crate::PathRewriter, A);
impl_from_rewriter!(crate::HeaderRewriter, B);
impl_from_rewriter!(crate::MethodRewriter, C);
impl_from_rewriter!(crate::HrefRewriter, D);
impl_from_rewriter!(SequenceRewriterType, E);
impl_from_rewriter!(ConditionalRewriterType, F);

// Implement combinator functions for rewriter types
//
// This provides:
// - `then` to chain rewriters
// - `when` to apply rewriters conditionally
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
                    Either6::A(path) => this.then(path.0.clone()).into(),
                    Either6::B(header) => this.then(header.0.clone()).into(),
                    Either6::C(method) => this.then(method.0.clone()).into(),
                    Either6::D(href) => this.then(href.0.clone()).into(),
                    Either6::E(sequence) => this.then(sequence.0.clone()).into(),
                    Either6::F(conditional) => this.then(conditional.0.clone()).into(),
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
impl_rewriter_combinators!(HrefRewriter);
impl_rewriter_combinators!(SequenceRewriter);
impl_rewriter_combinators!(ConditionalRewriter);

//
// Config-based Rewriter
//

/// Describe if a conmdition set is combined with AND or OR logic
#[napi(string_enum = "lowercase")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum ConditionOperation {
    /// All conditions must match for the rewriters to be applied
    #[default]
    And,
    /// At least one condition must match for the rewriters to be applied
    Or,
}

/// The types of conditions which may be used in a `ConditionConfig`.
#[napi(string_enum = "snake_case")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ConditionType {
    /// Matches based on the request path
    Path,
    /// Matches based on the request header
    Header,
    /// Matches based on the request method
    Method,
    /// Matches if a file exists at the given path
    Exists,
    /// Matches if a file does not exist at the given path
    NotExists,
}

/// Configuration for a condition that can be used in a `ConditionalRewriterConfig`.
#[napi(object)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConditionConfig {
    /// The type of condition to apply
    #[napi(js_name = "type")]
    pub condition: ConditionType,
    /// The arguments for the condition, such as the path or header name
    pub args: Option<Vec<String>>,
}

impl TryFrom<ConditionConfig> for crate::PathCondition {
    type Error = Error;

    fn try_from(config: ConditionConfig) -> Result<Self> {
        if config.condition != ConditionType::Path {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Path condition type".to_string(),
            ));
        }
        let args = config.args.unwrap_or_default();
        if args.len() != 1 {
            return Err(Error::new(
                Status::InvalidArg,
                "Path condition requires exactly one argument".to_string(),
            ));
        }
        let pattern = args[0].clone();
        let condition = crate::PathCondition::new(pattern)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        Ok(condition)
    }
}

impl TryFrom<ConditionConfig> for crate::HeaderCondition {
    type Error = Error;

    fn try_from(config: ConditionConfig) -> Result<Self> {
        if config.condition != ConditionType::Header {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Header condition type".to_string(),
            ));
        }
        let args = config.args.unwrap_or_default();
        if args.len() != 2 {
            return Err(Error::new(
                Status::InvalidArg,
                "Header condition requires exactly two arguments".to_string(),
            ));
        }
        let header = args[0].clone();
        let value = args[1].clone();
        let condition = crate::HeaderCondition::new(header, value)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        Ok(condition)
    }
}

impl TryFrom<ConditionConfig> for crate::MethodCondition {
    type Error = Error;

    fn try_from(config: ConditionConfig) -> Result<Self> {
        if config.condition != ConditionType::Method {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Method condition type".to_string(),
            ));
        }
        let args = config.args.unwrap_or_default();
        if args.len() != 1 {
            return Err(Error::new(
                Status::InvalidArg,
                "Method condition requires exactly one argument".to_string(),
            ));
        }
        let method = args[0].clone();
        let condition = crate::MethodCondition::new(method)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        Ok(condition)
    }
}

impl TryFrom<ConditionConfig> for crate::ExistenceCondition {
    type Error = Error;

    fn try_from(config: ConditionConfig) -> Result<Self> {
        if config.condition != ConditionType::Exists {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Exists condition type".to_string(),
            ));
        }
        if !config.args.unwrap_or_default().is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "Exists condition requires no arguments".to_string(),
            ));
        }
        Ok(crate::ExistenceCondition::new())
    }
}

impl TryFrom<ConditionConfig> for crate::NonExistenceCondition {
    type Error = Error;

    fn try_from(config: ConditionConfig) -> Result<Self> {
        if config.condition != ConditionType::NotExists {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected NotExists condition type".to_string(),
            ));
        }
        if !config.args.unwrap_or_default().is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "NotExists condition requires no arguments".to_string(),
            ));
        }
        Ok(crate::NonExistenceCondition::new())
    }
}

impl TryFrom<(ConditionOperation, Vec<Condition>)> for Condition {
    type Error = Error;

    fn try_from((operation, conditions): (ConditionOperation, Vec<Condition>)) -> Result<Self> {
        if conditions.is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "At least one condition is required".to_string(),
            ));
        }

        Ok(conditions
            .into_iter()
            .reduce(|a, b| match operation {
                ConditionOperation::And => and(a, b),
                ConditionOperation::Or => or(a, b),
            })
            .unwrap())
    }
}

/// The types of rewriters which may be used in a `RewriterConfig`.
#[napi(string_enum = "lowercase")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RewriterType {
    /// Rewrites the request path
    Path,
    /// Rewrites a request header
    Header,
    /// Rewrites the request method
    Method,
    /// Rewrites the request href
    Href,
}

/// Configuration for a rewriter that can be used in a `ConditionalRewriterConfig`.
#[napi(object)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RewriterConfig {
    /// The type of rewriter to apply
    #[napi(js_name = "type")]
    pub rewriter_type: RewriterType,
    /// The arguments for the rewriter, such as the pattern and replacement
    pub args: Option<Vec<String>>,
}

//
// Convert `RewriterConfig` into specific rewriter types.
//

impl TryFrom<RewriterConfig> for crate::PathRewriter {
    type Error = Error;

    fn try_from(config: RewriterConfig) -> Result<Self> {
        if config.rewriter_type != RewriterType::Path {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Path rewriter type".to_string(),
            ));
        }
        let args = config.args.unwrap_or_default();
        if args.len() != 2 {
            return Err(Error::new(
                Status::InvalidArg,
                "Path rewriter requires exactly two arguments".to_string(),
            ));
        }
        let pattern = args[0].clone();
        let replacement = args[1].clone();
        let rewriter = crate::PathRewriter::new(pattern, replacement)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        Ok(rewriter)
    }
}

impl TryFrom<RewriterConfig> for crate::HeaderRewriter {
    type Error = Error;

    fn try_from(config: RewriterConfig) -> Result<Self> {
        if config.rewriter_type != RewriterType::Header {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Header rewriter type".to_string(),
            ));
        }
        let args = config.args.unwrap_or_default();
        if args.len() != 3 {
            return Err(Error::new(
                Status::InvalidArg,
                "Header rewriter requires exactly three arguments".to_string(),
            ));
        }
        let header = args[0].clone();
        let pattern = args[1].clone();
        let replacement = args[2].clone();
        let rewriter = crate::HeaderRewriter::new(header, pattern, replacement)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        Ok(rewriter)
    }
}

impl TryFrom<RewriterConfig> for crate::MethodRewriter {
    type Error = Error;

    fn try_from(config: RewriterConfig) -> Result<Self> {
        if config.rewriter_type != RewriterType::Method {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Method rewriter type".to_string(),
            ));
        }
        let args = config.args.unwrap_or_default();
        if args.len() != 1 {
            return Err(Error::new(
                Status::InvalidArg,
                "Method rewriter requires exactly one argument".to_string(),
            ));
        }
        let method = args[0].clone();
        let rewriter = crate::MethodRewriter::new(method.as_str())
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        Ok(rewriter)
    }
}

impl TryFrom<RewriterConfig> for crate::HrefRewriter {
    type Error = Error;

    fn try_from(config: RewriterConfig) -> Result<Self> {
        if config.rewriter_type != RewriterType::Href {
            return Err(Error::new(
                Status::InvalidArg,
                "Expected Href rewriter type".to_string(),
            ));
        }
        let args = config.args.unwrap_or_default();
        if args.len() != 2 {
            return Err(Error::new(
                Status::InvalidArg,
                "Href rewriter requires exactly two arguments".to_string(),
            ));
        }
        let pattern = args[0].clone();
        let replacement = args[1].clone();
        let rewriter = crate::HrefRewriter::new(pattern, replacement)
            .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?;
        Ok(rewriter)
    }
}

/// Configuration for a conditional rewriter that can be used in a `Rewriter`.
#[napi(object)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ConditionalRewriterConfig {
    /// The logical operation to use when applying the condition set
    pub operation: Option<ConditionOperation>,
    /// The conditions that must be met for the rewriters to be applied
    pub conditions: Option<Vec<ConditionConfig>>,
    /// The rewriters to apply if the conditions are met
    pub rewriters: Vec<RewriterConfig>,
}

/// Allows constructing rewriter and condition configurations from JSON.
#[napi]
#[derive(Clone, Debug)]
pub struct Rewriter(AnyRewriterOwned);

#[napi]
impl Rewriter {
    /// Create a new rewriter from a list of configurations.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewriter = new Rewriter([
    ///   {
    ///     operation: 'And',
    ///     conditions: [
    ///       { type: 'Path', args: ['/old-path'] },
    ///       { type: 'Method', args: ['GET'] }
    ///     ],
    ///     rewriters: [
    ///       { type: 'Path', args: ['/new-path'] }
    ///     ]
    ///   },
    ///   {
    ///     conditions: [
    ///       { type: 'Path', args: ['/api/*'] }
    ///     ],
    ///     rewriters: [
    ///       { type: 'Header', args: ['X-API-Version', '2'] }
    ///     ]
    ///   }
    /// ]);
    /// ```
    #[napi(constructor)]
    pub fn new(configs: Vec<ConditionalRewriterConfig>) -> Result<Self> {
        let rewriter = Rewriter::try_from(configs)
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewriter)
    }

    /// Rewrite the given request using the configured rewriter.
    ///
    /// # Examples
    ///
    /// ```js
    /// const rewritten = rewriter.rewrite(request);
    /// ```
    #[napi(js_name = "rewrite")]
    pub fn js_rewrite(&self, request: Request) -> Result<Request> {
        let rewritten = self
            .rewrite(request.deref().to_owned())
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(rewritten.into())
    }
}

impl crate::Rewriter for Rewriter {
    fn rewrite<B>(
        &self,
        request: http::Request<B>,
    ) -> std::result::Result<http::Request<B>, crate::RewriteError> {
        match &self.0 {
            Either6::A(path) => path.rewrite(request),
            Either6::B(header) => header.rewrite(request),
            Either6::C(method) => method.rewrite(request),
            Either6::D(href) => href.rewrite(request),
            Either6::E(sequence) => sequence.rewrite(request),
            Either6::F(conditional) => conditional.rewrite(request),
        }
    }
}

use ::napi::bindgen_prelude::{ClassInstance, FromNapiValue};
use ::napi::sys;

impl FromNapiValue for Rewriter {
    unsafe fn from_napi_value(env: sys::napi_env, value: sys::napi_value) -> Result<Self> {
        // Try to convert from ClassInstance<Rewriter>
        if let Ok(instance) = unsafe { ClassInstance::<Rewriter>::from_napi_value(env, value) } {
            return Ok(Rewriter(instance.0.clone()));
        }

        // If that fails, try to convert from AnyRewriter
        if let Ok(rewriter) = unsafe { AnyRewriter::from_napi_value(env, value) } {
            return Ok(match rewriter {
                Either6::A(PathRewriter(path)) => path.to_owned().into(),
                Either6::B(HeaderRewriter(header)) => header.to_owned().into(),
                Either6::C(MethodRewriter(method)) => method.to_owned().into(),
                Either6::D(HrefRewriter(href)) => href.to_owned().into(),
                Either6::E(SequenceRewriter(sequence)) => sequence.to_owned().into(),
                Either6::F(ConditionalRewriter(conditional)) => conditional.to_owned().into(),
            });
        }

        // If both conversions fail, return an error
        Err(Error::new(
            Status::InvalidArg,
            "Expected a Rewriter instance or any other type of rewriter",
        ))
    }
}

impl TryFrom<ConditionalRewriterConfig> for Rewriter {
    type Error = Error;

    fn try_from(config: ConditionalRewriterConfig) -> Result<Self> {
        // Extract fields before consuming config
        let ConditionalRewriterConfig {
            operation,
            conditions,
            rewriters,
        } = config;

        // Validate that we have at least one rewriter
        if rewriters.is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "At least one rewriter is required".to_string(),
            ));
        }

        let rewriter: Rewriter = rewriters
            .into_iter()
            .map(Rewriter::try_from)
            .collect::<Result<Vec<_>>>()?
            .try_into()?;

        if conditions.is_none() {
            return Ok(rewriter);
        }

        let conditions = conditions.unwrap_or_default();
        if conditions.is_empty() {
            return Ok(rewriter);
        }

        let conditions: Vec<Condition> = conditions
            .into_iter()
            .map(Condition::try_from)
            .collect::<Result<Vec<_>>>()?;

        let operation = operation.unwrap_or_default();

        let condition: Condition = (operation, conditions).try_into()?;

        Ok(when(rewriter, condition))
    }
}

impl TryFrom<RewriterConfig> for Rewriter {
    type Error = Error;

    fn try_from(config: RewriterConfig) -> Result<Self> {
        Ok(Rewriter(match config.rewriter_type {
            RewriterType::Path => Either6::A(config.try_into()?),
            RewriterType::Header => Either6::B(config.try_into()?),
            RewriterType::Method => Either6::C(config.try_into()?),
            RewriterType::Href => Either6::D(config.try_into()?),
        }))
    }
}

impl TryFrom<Vec<RewriterConfig>> for Rewriter {
    type Error = Error;

    fn try_from(configs: Vec<RewriterConfig>) -> Result<Self> {
        if configs.is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "At least one rewriter configuration is required".to_string(),
            ));
        }

        // Convert each config to a rewriter
        configs
            .into_iter()
            .map(Rewriter::try_from)
            .collect::<Result<Vec<_>>>()?
            .try_into()
    }
}

impl TryFrom<Vec<Rewriter>> for Rewriter {
    type Error = Error;

    fn try_from(rewriters: Vec<Rewriter>) -> Result<Self> {
        // Ensure we have at least one rewriter
        if rewriters.is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "At least one rewriter is required".to_string(),
            ));
        }

        // Reduce the rewriters into a single Rewriter sequence
        Ok(rewriters.into_iter().reduce(then).unwrap())
    }
}

impl TryFrom<Vec<ConditionalRewriterConfig>> for Rewriter {
    type Error = Error;

    fn try_from(configs: Vec<ConditionalRewriterConfig>) -> Result<Self> {
        if configs.is_empty() {
            return Err(Error::new(
                Status::InvalidArg,
                "At least one configuration is required".to_string(),
            ));
        }

        // Convert each config to a ConditionalRewriterType
        configs
            .into_iter()
            .map(Rewriter::try_from)
            .collect::<Result<Vec<_>>>()?
            .try_into()
    }
}

//
// Generic combinators for rewriters
//

fn then<A, B>(a: A, b: B) -> Rewriter
where
    A: Into<Rewriter>,
    B: Into<Rewriter>,
{
    match (a.into().0, b.into().0) {
        (Either6::A(a), Either6::A(b)) => a.then(b).into(),
        (Either6::A(a), Either6::B(b)) => a.then(b).into(),
        (Either6::A(a), Either6::C(b)) => a.then(b).into(),
        (Either6::A(a), Either6::D(b)) => a.then(b).into(),
        (Either6::A(a), Either6::E(b)) => a.then(b).into(),
        (Either6::A(a), Either6::F(b)) => a.then(b).into(),

        (Either6::B(a), Either6::A(b)) => a.then(b).into(),
        (Either6::B(a), Either6::B(b)) => a.then(b).into(),
        (Either6::B(a), Either6::C(b)) => a.then(b).into(),
        (Either6::B(a), Either6::D(b)) => a.then(b).into(),
        (Either6::B(a), Either6::E(b)) => a.then(b).into(),
        (Either6::B(a), Either6::F(b)) => a.then(b).into(),

        (Either6::C(a), Either6::A(b)) => a.then(b).into(),
        (Either6::C(a), Either6::B(b)) => a.then(b).into(),
        (Either6::C(a), Either6::C(b)) => a.then(b).into(),
        (Either6::C(a), Either6::D(b)) => a.then(b).into(),
        (Either6::C(a), Either6::E(b)) => a.then(b).into(),
        (Either6::C(a), Either6::F(b)) => a.then(b).into(),

        (Either6::D(a), Either6::A(b)) => a.then(b).into(),
        (Either6::D(a), Either6::B(b)) => a.then(b).into(),
        (Either6::D(a), Either6::C(b)) => a.then(b).into(),
        (Either6::D(a), Either6::D(b)) => a.then(b).into(),
        (Either6::D(a), Either6::E(b)) => a.then(b).into(),
        (Either6::D(a), Either6::F(b)) => a.then(b).into(),

        (Either6::E(a), Either6::A(b)) => a.then(b).into(),
        (Either6::E(a), Either6::B(b)) => a.then(b).into(),
        (Either6::E(a), Either6::C(b)) => a.then(b).into(),
        (Either6::E(a), Either6::D(b)) => a.then(b).into(),
        (Either6::E(a), Either6::E(b)) => a.then(b).into(),
        (Either6::E(a), Either6::F(b)) => a.then(b).into(),

        (Either6::F(a), Either6::A(b)) => a.then(b).into(),
        (Either6::F(a), Either6::B(b)) => a.then(b).into(),
        (Either6::F(a), Either6::C(b)) => a.then(b).into(),
        (Either6::F(a), Either6::D(b)) => a.then(b).into(),
        (Either6::F(a), Either6::E(b)) => a.then(b).into(),
        (Either6::F(a), Either6::F(b)) => a.then(b).into(),
    }
}

fn when<A, B>(a: A, b: B) -> Rewriter
where
    A: Into<Rewriter>,
    B: Into<Condition>,
{
    match (a.into().0, b.into().0) {
        (Either6::A(path), Either6::A(condition)) => path.when(condition).into(),
        (Either6::A(path), Either6::B(condition)) => path.when(condition).into(),
        (Either6::A(path), Either6::C(condition)) => path.when(condition).into(),
        (Either6::A(path), Either6::D(condition)) => path.when(condition).into(),
        (Either6::A(path), Either6::E(condition)) => path.when(condition).into(),
        (Either6::A(path), Either6::F(condition)) => path.when(condition).into(),

        (Either6::B(header), Either6::A(condition)) => header.when(condition).into(),
        (Either6::B(header), Either6::B(condition)) => header.when(condition).into(),
        (Either6::B(header), Either6::C(condition)) => header.when(condition).into(),
        (Either6::B(header), Either6::D(condition)) => header.when(condition).into(),
        (Either6::B(header), Either6::E(condition)) => header.when(condition).into(),
        (Either6::B(header), Either6::F(condition)) => header.when(condition).into(),

        (Either6::C(method), Either6::A(condition)) => method.when(condition).into(),
        (Either6::C(method), Either6::B(condition)) => method.when(condition).into(),
        (Either6::C(method), Either6::C(condition)) => method.when(condition).into(),
        (Either6::C(method), Either6::D(condition)) => method.when(condition).into(),
        (Either6::C(method), Either6::E(condition)) => method.when(condition).into(),
        (Either6::C(method), Either6::F(condition)) => method.when(condition).into(),

        (Either6::D(href), Either6::A(condition)) => href.when(condition).into(),
        (Either6::D(href), Either6::B(condition)) => href.when(condition).into(),
        (Either6::D(href), Either6::C(condition)) => href.when(condition).into(),
        (Either6::D(href), Either6::D(condition)) => href.when(condition).into(),
        (Either6::D(href), Either6::E(condition)) => href.when(condition).into(),
        (Either6::D(href), Either6::F(condition)) => href.when(condition).into(),

        (Either6::E(sequence), Either6::A(condition)) => sequence.when(condition).into(),
        (Either6::E(sequence), Either6::B(condition)) => sequence.when(condition).into(),
        (Either6::E(sequence), Either6::C(condition)) => sequence.when(condition).into(),
        (Either6::E(sequence), Either6::D(condition)) => sequence.when(condition).into(),
        (Either6::E(sequence), Either6::E(condition)) => sequence.when(condition).into(),
        (Either6::E(sequence), Either6::F(condition)) => sequence.when(condition).into(),

        (Either6::F(conditional), Either6::A(condition)) => conditional.when(condition).into(),
        (Either6::F(conditional), Either6::B(condition)) => conditional.when(condition).into(),
        (Either6::F(conditional), Either6::C(condition)) => conditional.when(condition).into(),
        (Either6::F(conditional), Either6::D(condition)) => conditional.when(condition).into(),
        (Either6::F(conditional), Either6::E(condition)) => conditional.when(condition).into(),
        (Either6::F(conditional), Either6::F(condition)) => conditional.when(condition).into(),
    }
}

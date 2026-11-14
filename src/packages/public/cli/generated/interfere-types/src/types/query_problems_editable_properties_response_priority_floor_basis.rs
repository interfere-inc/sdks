pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsEditablePropertiesResponsePriorityFloorBasis {
    #[serde(rename = "activeUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<i64>,
    #[serde(rename = "affectedUsers")]
    #[serde(default)]
    pub affected_users: i64,
    #[serde(rename = "evaluatedAt")]
    #[serde(default)]
    pub evaluated_at: i64,
    #[serde(default)]
    pub raised: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub share: Option<f64>,
    #[serde(default)]
    pub stale: bool,
}

impl QueryProblemsEditablePropertiesResponsePriorityFloorBasis {
    pub fn builder() -> QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder {
        <QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder {
    active_users: Option<i64>,
    affected_users: Option<i64>,
    evaluated_at: Option<i64>,
    raised: Option<bool>,
    share: Option<f64>,
    stale: Option<bool>,
}

impl QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder {
    pub fn active_users(mut self, value: i64) -> Self {
        self.active_users = Some(value);
        self
    }

    pub fn affected_users(mut self, value: i64) -> Self {
        self.affected_users = Some(value);
        self
    }

    pub fn evaluated_at(mut self, value: i64) -> Self {
        self.evaluated_at = Some(value);
        self
    }

    pub fn raised(mut self, value: bool) -> Self {
        self.raised = Some(value);
        self
    }

    pub fn share(mut self, value: f64) -> Self {
        self.share = Some(value);
        self
    }

    pub fn stale(mut self, value: bool) -> Self {
        self.stale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsEditablePropertiesResponsePriorityFloorBasis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`affected_users`](QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder::affected_users)
    /// - [`evaluated_at`](QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder::evaluated_at)
    /// - [`raised`](QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder::raised)
    /// - [`stale`](QueryProblemsEditablePropertiesResponsePriorityFloorBasisBuilder::stale)
    pub fn build(self) -> Result<QueryProblemsEditablePropertiesResponsePriorityFloorBasis, BuildError> {
        Ok(QueryProblemsEditablePropertiesResponsePriorityFloorBasis {
            active_users: self.active_users,
            affected_users: self.affected_users.ok_or_else(|| BuildError::missing_field("affected_users"))?,
            evaluated_at: self.evaluated_at.ok_or_else(|| BuildError::missing_field("evaluated_at"))?,
            raised: self.raised.ok_or_else(|| BuildError::missing_field("raised"))?,
            share: self.share,
            stale: self.stale.ok_or_else(|| BuildError::missing_field("stale"))?,
        })
    }
}

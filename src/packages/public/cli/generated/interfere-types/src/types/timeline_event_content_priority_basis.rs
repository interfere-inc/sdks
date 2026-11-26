pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentPriorityBasis {
    #[serde(rename = "activeUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<i64>,
    #[serde(rename = "affectedUsers")]
    #[serde(default)]
    pub affected_users: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub share: Option<f64>,
}

impl TimelineEventContentPriorityBasis {
    pub fn builder() -> TimelineEventContentPriorityBasisBuilder {
        <TimelineEventContentPriorityBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentPriorityBasisBuilder {
    active_users: Option<i64>,
    affected_users: Option<i64>,
    share: Option<f64>,
}

impl TimelineEventContentPriorityBasisBuilder {
    pub fn active_users(mut self, value: i64) -> Self {
        self.active_users = Some(value);
        self
    }

    pub fn affected_users(mut self, value: i64) -> Self {
        self.affected_users = Some(value);
        self
    }

    pub fn share(mut self, value: f64) -> Self {
        self.share = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentPriorityBasis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`affected_users`](TimelineEventContentPriorityBasisBuilder::affected_users)
    pub fn build(self) -> Result<TimelineEventContentPriorityBasis, BuildError> {
        Ok(TimelineEventContentPriorityBasis {
            active_users: self.active_users,
            affected_users: self.affected_users.ok_or_else(|| BuildError::missing_field("affected_users"))?,
            share: self.share,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis {
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

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasisBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasisBuilder {
    active_users: Option<i64>,
    affected_users: Option<i64>,
    share: Option<f64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasisBuilder {
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`affected_users`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasisBuilder::affected_users)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentBasisBasis {
            active_users: self.active_users,
            affected_users: self.affected_users.ok_or_else(|| BuildError::missing_field("affected_users"))?,
            share: self.share,
        })
    }
}

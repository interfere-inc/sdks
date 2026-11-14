pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo {
    #[serde(default)]
    pub at: i64,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub status: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoStatus,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoBuilder {
    at: Option<i64>,
    user_id: Option<String>,
    status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoStatus>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoBuilder {
    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoBuilder::at)
    /// - [`status`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwoBuilder::status)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionTwo {
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            user_id: self.user_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt {
    pub status: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtStatus,
    #[serde(default)]
    pub at: i64,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtBuilder {
    status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtStatus>,
    at: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtBuilder {
    pub fn status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtBuilder::status)
    /// - [`at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAtBuilder::at)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionAt {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
        })
    }
}

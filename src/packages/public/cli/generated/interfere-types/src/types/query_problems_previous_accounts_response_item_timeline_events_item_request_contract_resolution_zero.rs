pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero {
    pub status: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroStatus,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroBuilder {
    status: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroStatus>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroBuilder {
    pub fn status(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZeroBuilder::status)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolutionZero {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

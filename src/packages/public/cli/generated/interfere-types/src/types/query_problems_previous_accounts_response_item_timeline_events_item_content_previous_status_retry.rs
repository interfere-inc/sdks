pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryKind,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryKind>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetryBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentPreviousStatusRetry {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

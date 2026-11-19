pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryKind,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryKind>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetryBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalRetry {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

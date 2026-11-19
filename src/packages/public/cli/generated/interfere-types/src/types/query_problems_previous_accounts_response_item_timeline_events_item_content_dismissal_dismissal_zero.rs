pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroKind,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroKind>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZero {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

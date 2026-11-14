pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindKind,
    #[serde(rename = "gapKind")]
    pub gap_kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindGapKind,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindKind>,
    gap_kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindGapKind>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn gap_kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindGapKind) -> Self {
        self.gap_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindBuilder::kind)
    /// - [`gap_kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKindBuilder::gap_kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionGapKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            gap_kind: self.gap_kind.ok_or_else(|| BuildError::missing_field("gap_kind"))?,
        })
    }
}

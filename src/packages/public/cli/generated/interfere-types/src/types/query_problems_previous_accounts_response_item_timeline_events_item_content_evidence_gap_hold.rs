pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldKind,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldBuilder {
    category: Option<String>,
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldKind>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHoldBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapHold {
            category: self.category,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindKind,
    #[serde(rename = "gapKind")]
    pub gap_kind: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindGapKind,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindKind>,
    gap_kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindGapKind>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn gap_kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindGapKind) -> Self {
        self.gap_kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindBuilder::kind)
    /// - [`gap_kind`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindBuilder::gap_kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            gap_kind: self.gap_kind.ok_or_else(|| BuildError::missing_field("gap_kind"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldKind,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldBuilder {
    category: Option<String>,
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldKind>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHoldBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapHold {
            category: self.category,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

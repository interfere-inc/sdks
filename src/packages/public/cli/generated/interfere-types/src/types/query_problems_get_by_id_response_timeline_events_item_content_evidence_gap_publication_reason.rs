pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason {
    #[serde(default)]
    pub attempts: i64,
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonKind,
    pub reason: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonReason,
    pub rejection: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonRejection,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder {
    attempts: Option<i64>,
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonKind>,
    reason: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonReason>,
    rejection: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonRejection>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn reason(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn rejection(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::attempts)
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::kind)
    /// - [`reason`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::reason)
    /// - [`rejection`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::rejection)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReason {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason {
    #[serde(default)]
    pub attempts: i64,
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonKind,
    pub reason: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonReason,
    pub rejection: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonRejection,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder {
    attempts: Option<i64>,
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonKind>,
    reason: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonReason>,
    rejection: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonRejection>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn reason(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn rejection(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonRejection) -> Self {
        self.rejection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::attempts)
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::kind)
    /// - [`reason`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::reason)
    /// - [`rejection`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReasonBuilder::rejection)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationReason {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            rejection: self.rejection.ok_or_else(|| BuildError::missing_field("rejection"))?,
        })
    }
}

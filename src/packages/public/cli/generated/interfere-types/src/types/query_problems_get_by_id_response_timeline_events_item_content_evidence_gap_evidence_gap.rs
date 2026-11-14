pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapKind,
    #[serde(rename = "replayDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_disabled: Option<bool>,
    #[serde(rename = "sharedAsk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_ask: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapKind>,
    replay_disabled: Option<bool>,
    shared_ask: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk>,
    surface: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn replay_disabled(mut self, value: bool) -> Self {
        self.replay_disabled = Some(value);
        self
    }

    pub fn shared_ask(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk) -> Self {
        self.shared_ask = Some(value);
        self
    }

    pub fn surface(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface) -> Self {
        self.surface = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGap {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            replay_disabled: self.replay_disabled,
            shared_ask: self.shared_ask,
            surface: self.surface,
        })
    }
}

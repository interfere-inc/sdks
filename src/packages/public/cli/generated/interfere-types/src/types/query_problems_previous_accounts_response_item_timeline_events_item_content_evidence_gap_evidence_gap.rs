pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap {
    pub kind: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapKind,
    #[serde(rename = "replayDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_disabled: Option<bool>,
    #[serde(rename = "sharedAsk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_ask: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSurface>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapBuilder {
    kind: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapKind>,
    replay_disabled: Option<bool>,
    shared_ask: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk>,
    service: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapService>,
    surface: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSurface>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapBuilder {
    pub fn kind(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn replay_disabled(mut self, value: bool) -> Self {
        self.replay_disabled = Some(value);
        self
    }

    pub fn shared_ask(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSharedAsk) -> Self {
        self.shared_ask = Some(value);
        self
    }

    pub fn service(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapService) -> Self {
        self.service = Some(value);
        self
    }

    pub fn surface(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapSurface) -> Self {
        self.surface = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGapBuilder::kind)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapEvidenceGap {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            replay_disabled: self.replay_disabled,
            shared_ask: self.shared_ask,
            service: self.service,
            surface: self.surface,
        })
    }
}

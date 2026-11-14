pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapEvidenceGap {
    pub kind: TimelineEventContentEvidenceGapEvidenceGapKind,
    #[serde(rename = "replayDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_disabled: Option<bool>,
    #[serde(rename = "sharedAsk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_ask: Option<TimelineEventContentEvidenceGapEvidenceGapSharedAsk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<TimelineEventContentEvidenceGapEvidenceGapSurface>,
}

impl TimelineEventContentEvidenceGapEvidenceGap {
    pub fn builder() -> TimelineEventContentEvidenceGapEvidenceGapBuilder {
        <TimelineEventContentEvidenceGapEvidenceGapBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapEvidenceGapBuilder {
    kind: Option<TimelineEventContentEvidenceGapEvidenceGapKind>,
    replay_disabled: Option<bool>,
    shared_ask: Option<TimelineEventContentEvidenceGapEvidenceGapSharedAsk>,
    surface: Option<TimelineEventContentEvidenceGapEvidenceGapSurface>,
}

impl TimelineEventContentEvidenceGapEvidenceGapBuilder {
    pub fn kind(mut self, value: TimelineEventContentEvidenceGapEvidenceGapKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn replay_disabled(mut self, value: bool) -> Self {
        self.replay_disabled = Some(value);
        self
    }

    pub fn shared_ask(mut self, value: TimelineEventContentEvidenceGapEvidenceGapSharedAsk) -> Self {
        self.shared_ask = Some(value);
        self
    }

    pub fn surface(mut self, value: TimelineEventContentEvidenceGapEvidenceGapSurface) -> Self {
        self.surface = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapEvidenceGap`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentEvidenceGapEvidenceGapBuilder::kind)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapEvidenceGap, BuildError> {
        Ok(TimelineEventContentEvidenceGapEvidenceGap {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            replay_disabled: self.replay_disabled,
            shared_ask: self.shared_ask,
            surface: self.surface,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptKind {
    pub kind: TimelineEventRequestContractResolutionReceiptReceiptKindKind,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "observedAt")]
    #[serde(default)]
    pub observed_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidence>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptKind {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptKindBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptKindBuilder {
    kind: Option<TimelineEventRequestContractResolutionReceiptReceiptKindKind>,
    observed_at: Option<i64>,
    evidence: Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidence>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptKindBuilder {
    pub fn kind(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn observed_at(mut self, value: i64) -> Self {
        self.observed_at = Some(value);
        self
    }

    pub fn evidence(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptKindEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventRequestContractResolutionReceiptReceiptKindBuilder::kind)
    /// - [`observed_at`](TimelineEventRequestContractResolutionReceiptReceiptKindBuilder::observed_at)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptKind, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            observed_at: self.observed_at.ok_or_else(|| BuildError::missing_field("observed_at"))?,
            evidence: self.evidence,
        })
    }
}

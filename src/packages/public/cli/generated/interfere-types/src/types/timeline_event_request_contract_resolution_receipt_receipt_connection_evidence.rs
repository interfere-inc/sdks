pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence {
    #[serde(rename = "requiredEvidence")]
    pub required_evidence: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "observedAt")]
    #[serde(default)]
    pub observed_at: i64,
    #[serde(rename = "recordId")]
    #[serde(default)]
    pub record_id: String,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceBuilder {
    required_evidence: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence>,
    observed_at: Option<i64>,
    record_id: Option<String>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceBuilder {
    pub fn required_evidence(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidence) -> Self {
        self.required_evidence = Some(value);
        self
    }

    pub fn observed_at(mut self, value: i64) -> Self {
        self.observed_at = Some(value);
        self
    }

    pub fn record_id(mut self, value: impl Into<String>) -> Self {
        self.record_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required_evidence`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceBuilder::required_evidence)
    /// - [`observed_at`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceBuilder::observed_at)
    /// - [`record_id`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceBuilder::record_id)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidence {
            required_evidence: self.required_evidence.ok_or_else(|| BuildError::missing_field("required_evidence"))?,
            observed_at: self.observed_at.ok_or_else(|| BuildError::missing_field("observed_at"))?,
            record_id: self.record_id.ok_or_else(|| BuildError::missing_field("record_id"))?,
        })
    }
}

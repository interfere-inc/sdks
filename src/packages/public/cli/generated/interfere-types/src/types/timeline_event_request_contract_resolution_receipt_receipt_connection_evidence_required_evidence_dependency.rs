pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyMode,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyMode>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptConnectionEvidenceRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

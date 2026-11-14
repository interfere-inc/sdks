pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyMode,
}

impl TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency {
    pub fn builder() -> TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder {
        <TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyMode>,
}

impl TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency, BuildError> {
        Ok(TimelineEventRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

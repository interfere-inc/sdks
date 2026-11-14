pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode,
}

impl TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
    pub fn builder() -> TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
        <TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode>,
}

impl TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency, BuildError> {
        Ok(TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventRequestContractRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: TimelineEventRequestContractRequiredEvidenceDependencyMode,
}

impl TimelineEventRequestContractRequiredEvidenceDependency {
    pub fn builder() -> TimelineEventRequestContractRequiredEvidenceDependencyBuilder {
        <TimelineEventRequestContractRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventRequestContractRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<TimelineEventRequestContractRequiredEvidenceDependencyMode>,
}

impl TimelineEventRequestContractRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: TimelineEventRequestContractRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventRequestContractRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](TimelineEventRequestContractRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](TimelineEventRequestContractRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<TimelineEventRequestContractRequiredEvidenceDependency, BuildError> {
        Ok(TimelineEventRequestContractRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: TimelineEventContentActionKindRequiredEvidenceDependencyMode,
}

impl TimelineEventContentActionKindRequiredEvidenceDependency {
    pub fn builder() -> TimelineEventContentActionKindRequiredEvidenceDependencyBuilder {
        <TimelineEventContentActionKindRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<TimelineEventContentActionKindRequiredEvidenceDependencyMode>,
}

impl TimelineEventContentActionKindRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: TimelineEventContentActionKindRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](TimelineEventContentActionKindRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](TimelineEventContentActionKindRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<TimelineEventContentActionKindRequiredEvidenceDependency, BuildError> {
        Ok(TimelineEventContentActionKindRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

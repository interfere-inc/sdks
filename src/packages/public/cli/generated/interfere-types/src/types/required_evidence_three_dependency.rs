pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceThreeDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: RequiredEvidenceThreeDependencyMode,
}

impl RequiredEvidenceThreeDependency {
    pub fn builder() -> RequiredEvidenceThreeDependencyBuilder {
        <RequiredEvidenceThreeDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceThreeDependencyBuilder {
    group_id: Option<String>,
    mode: Option<RequiredEvidenceThreeDependencyMode>,
}

impl RequiredEvidenceThreeDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: RequiredEvidenceThreeDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceThreeDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](RequiredEvidenceThreeDependencyBuilder::group_id)
    /// - [`mode`](RequiredEvidenceThreeDependencyBuilder::mode)
    pub fn build(self) -> Result<RequiredEvidenceThreeDependency, BuildError> {
        Ok(RequiredEvidenceThreeDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

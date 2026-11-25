pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceTwoDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: RequiredEvidenceTwoDependencyMode,
}

impl RequiredEvidenceTwoDependency {
    pub fn builder() -> RequiredEvidenceTwoDependencyBuilder {
        <RequiredEvidenceTwoDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceTwoDependencyBuilder {
    group_id: Option<String>,
    mode: Option<RequiredEvidenceTwoDependencyMode>,
}

impl RequiredEvidenceTwoDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: RequiredEvidenceTwoDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceTwoDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](RequiredEvidenceTwoDependencyBuilder::group_id)
    /// - [`mode`](RequiredEvidenceTwoDependencyBuilder::mode)
    pub fn build(self) -> Result<RequiredEvidenceTwoDependency, BuildError> {
        Ok(RequiredEvidenceTwoDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

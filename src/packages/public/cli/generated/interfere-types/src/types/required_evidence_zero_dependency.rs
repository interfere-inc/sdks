pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceZeroDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: RequiredEvidenceZeroDependencyMode,
}

impl RequiredEvidenceZeroDependency {
    pub fn builder() -> RequiredEvidenceZeroDependencyBuilder {
        <RequiredEvidenceZeroDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceZeroDependencyBuilder {
    group_id: Option<String>,
    mode: Option<RequiredEvidenceZeroDependencyMode>,
}

impl RequiredEvidenceZeroDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: RequiredEvidenceZeroDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceZeroDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](RequiredEvidenceZeroDependencyBuilder::group_id)
    /// - [`mode`](RequiredEvidenceZeroDependencyBuilder::mode)
    pub fn build(self) -> Result<RequiredEvidenceZeroDependency, BuildError> {
        Ok(RequiredEvidenceZeroDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

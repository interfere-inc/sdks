pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceOneDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: RequiredEvidenceOneDependencyMode,
}

impl RequiredEvidenceOneDependency {
    pub fn builder() -> RequiredEvidenceOneDependencyBuilder {
        <RequiredEvidenceOneDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceOneDependencyBuilder {
    group_id: Option<String>,
    mode: Option<RequiredEvidenceOneDependencyMode>,
}

impl RequiredEvidenceOneDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: RequiredEvidenceOneDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceOneDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](RequiredEvidenceOneDependencyBuilder::group_id)
    /// - [`mode`](RequiredEvidenceOneDependencyBuilder::mode)
    pub fn build(self) -> Result<RequiredEvidenceOneDependency, BuildError> {
        Ok(RequiredEvidenceOneDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

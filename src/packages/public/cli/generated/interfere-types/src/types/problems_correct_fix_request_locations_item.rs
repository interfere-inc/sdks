pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorrectFixRequestLocationsItem {
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<CorrectFixRequestLocationsItemLines>,
}

impl CorrectFixRequestLocationsItem {
    pub fn builder() -> CorrectFixRequestLocationsItemBuilder {
        <CorrectFixRequestLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectFixRequestLocationsItemBuilder {
    repository: Option<String>,
    path: Option<String>,
    lines: Option<CorrectFixRequestLocationsItemLines>,
}

impl CorrectFixRequestLocationsItemBuilder {
    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn lines(mut self, value: CorrectFixRequestLocationsItemLines) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectFixRequestLocationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`repository`](CorrectFixRequestLocationsItemBuilder::repository)
    /// - [`path`](CorrectFixRequestLocationsItemBuilder::path)
    pub fn build(self) -> Result<CorrectFixRequestLocationsItem, BuildError> {
        Ok(CorrectFixRequestLocationsItem {
            repository: self.repository.ok_or_else(|| BuildError::missing_field("repository"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            lines: self.lines,
        })
    }
}

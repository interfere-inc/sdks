pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorrectRootCauseRequestCauseLocationsItem {
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<CorrectRootCauseRequestCauseLocationsItemLines>,
}

impl CorrectRootCauseRequestCauseLocationsItem {
    pub fn builder() -> CorrectRootCauseRequestCauseLocationsItemBuilder {
        <CorrectRootCauseRequestCauseLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectRootCauseRequestCauseLocationsItemBuilder {
    repository: Option<String>,
    path: Option<String>,
    lines: Option<CorrectRootCauseRequestCauseLocationsItemLines>,
}

impl CorrectRootCauseRequestCauseLocationsItemBuilder {
    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn lines(mut self, value: CorrectRootCauseRequestCauseLocationsItemLines) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectRootCauseRequestCauseLocationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`repository`](CorrectRootCauseRequestCauseLocationsItemBuilder::repository)
    /// - [`path`](CorrectRootCauseRequestCauseLocationsItemBuilder::path)
    pub fn build(self) -> Result<CorrectRootCauseRequestCauseLocationsItem, BuildError> {
        Ok(CorrectRootCauseRequestCauseLocationsItem {
            repository: self.repository.ok_or_else(|| BuildError::missing_field("repository"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            lines: self.lines,
        })
    }
}

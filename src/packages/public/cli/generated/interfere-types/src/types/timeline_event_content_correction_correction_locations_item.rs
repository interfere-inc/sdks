pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentCorrectionCorrectionLocationsItem {
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<TimelineEventContentCorrectionCorrectionLocationsItemLines>,
}

impl TimelineEventContentCorrectionCorrectionLocationsItem {
    pub fn builder() -> TimelineEventContentCorrectionCorrectionLocationsItemBuilder {
        <TimelineEventContentCorrectionCorrectionLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionCorrectionLocationsItemBuilder {
    repository: Option<String>,
    path: Option<String>,
    lines: Option<TimelineEventContentCorrectionCorrectionLocationsItemLines>,
}

impl TimelineEventContentCorrectionCorrectionLocationsItemBuilder {
    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn lines(mut self, value: TimelineEventContentCorrectionCorrectionLocationsItemLines) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionCorrectionLocationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`repository`](TimelineEventContentCorrectionCorrectionLocationsItemBuilder::repository)
    /// - [`path`](TimelineEventContentCorrectionCorrectionLocationsItemBuilder::path)
    pub fn build(self) -> Result<TimelineEventContentCorrectionCorrectionLocationsItem, BuildError> {
        Ok(TimelineEventContentCorrectionCorrectionLocationsItem {
            repository: self.repository.ok_or_else(|| BuildError::missing_field("repository"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            lines: self.lines,
        })
    }
}

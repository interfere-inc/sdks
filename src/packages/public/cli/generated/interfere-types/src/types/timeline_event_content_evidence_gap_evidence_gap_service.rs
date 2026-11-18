pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentEvidenceGapEvidenceGapService {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl TimelineEventContentEvidenceGapEvidenceGapService {
    pub fn builder() -> TimelineEventContentEvidenceGapEvidenceGapServiceBuilder {
        <TimelineEventContentEvidenceGapEvidenceGapServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentEvidenceGapEvidenceGapServiceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl TimelineEventContentEvidenceGapEvidenceGapServiceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentEvidenceGapEvidenceGapService`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TimelineEventContentEvidenceGapEvidenceGapServiceBuilder::id)
    /// - [`name`](TimelineEventContentEvidenceGapEvidenceGapServiceBuilder::name)
    pub fn build(self) -> Result<TimelineEventContentEvidenceGapEvidenceGapService, BuildError> {
        Ok(TimelineEventContentEvidenceGapEvidenceGapService {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

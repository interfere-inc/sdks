pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapService {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapService {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapServiceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapServiceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapServiceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapService`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapServiceBuilder::id)
    /// - [`name`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapServiceBuilder::name)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapService, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapService {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface {
    #[serde(default)]
    pub name: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(default)]
    pub slug: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurfaceBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurfaceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurfaceBuilder {
    name: Option<String>,
    slug: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurfaceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurfaceBuilder::name)
    /// - [`slug`](QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurfaceBuilder::slug)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapEvidenceGapSurface {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}

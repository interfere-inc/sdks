pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyMode,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyMode>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentActionKindRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

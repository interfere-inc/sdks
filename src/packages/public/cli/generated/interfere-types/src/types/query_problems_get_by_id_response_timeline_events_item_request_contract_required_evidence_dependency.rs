pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    pub mode: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyMode,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder {
    group_id: Option<String>,
    mode: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyMode>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder::group_id)
    /// - [`mode`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependencyBuilder::mode)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidenceDependency {
            group_id: self.group_id.ok_or_else(|| BuildError::missing_field("group_id"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

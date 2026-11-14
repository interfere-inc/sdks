pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope {
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScopeBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScopeBuilder {
    organization_id: Option<String>,
    problem_id: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScopeBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScopeBuilder::organization_id)
    /// - [`problem_id`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScopeBuilder::problem_id)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}

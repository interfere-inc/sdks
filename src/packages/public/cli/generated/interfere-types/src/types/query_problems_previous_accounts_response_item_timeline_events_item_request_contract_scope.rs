pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope {
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScopeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScopeBuilder {
    organization_id: Option<String>,
    problem_id: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScopeBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScopeBuilder::organization_id)
    /// - [`problem_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScopeBuilder::problem_id)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}

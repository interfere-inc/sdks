pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "requiredEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_evidence: Option<RequiredEvidence>,
    #[serde(default)]
    pub scope: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope,
    pub completion: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletion,
    pub resolution: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolution,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder {
    version: Option<String>,
    required_evidence: Option<RequiredEvidence>,
    scope: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope>,
    completion: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletion>,
    resolution: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolution>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn required_evidence(mut self, value: RequiredEvidence) -> Self {
        self.required_evidence = Some(value);
        self
    }

    pub fn scope(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn completion(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletion) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn resolution(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder::version)
    /// - [`scope`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder::scope)
    /// - [`completion`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder::completion)
    /// - [`resolution`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractBuilder::resolution)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            required_evidence: self.required_evidence,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            completion: self.completion.ok_or_else(|| BuildError::missing_field("completion"))?,
            resolution: self.resolution.ok_or_else(|| BuildError::missing_field("resolution"))?,
        })
    }
}

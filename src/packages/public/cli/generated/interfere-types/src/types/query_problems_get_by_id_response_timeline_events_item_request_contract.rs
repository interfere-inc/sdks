pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContract {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "requiredEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence>,
    #[serde(default)]
    pub scope: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope,
    pub completion: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletion,
    pub resolution: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolution,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContract {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder {
    version: Option<String>,
    required_evidence: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence>,
    scope: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope>,
    completion: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletion>,
    resolution: Option<QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolution>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn required_evidence(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractRequiredEvidence) -> Self {
        self.required_evidence = Some(value);
        self
    }

    pub fn scope(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn completion(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletion) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn resolution(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemRequestContract`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder::version)
    /// - [`scope`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder::scope)
    /// - [`completion`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder::completion)
    /// - [`resolution`](QueryProblemsGetByIdResponseTimelineEventsItemRequestContractBuilder::resolution)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemRequestContract, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemRequestContract {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            required_evidence: self.required_evidence,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            completion: self.completion.ok_or_else(|| BuildError::missing_field("completion"))?,
            resolution: self.resolution.ok_or_else(|| BuildError::missing_field("resolution"))?,
        })
    }
}

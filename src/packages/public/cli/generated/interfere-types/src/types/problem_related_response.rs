pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemRelatedResponse {
    /// The Problem this one was merged into, if any
    #[serde(rename = "duplicateOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_of: Option<String>,
    /// Problems merged into this one
    #[serde(default)]
    pub duplicates: Vec<ProblemRelatedResponseDuplicatesItem>,
    /// Server Problems whose errors ran inside this Problem's failing requests in the last 7 days; a link shows both errors ran in one request, not that one caused the other
    #[serde(rename = "sameRequest")]
    #[serde(default)]
    pub same_request: Vec<ProblemRelatedResponseSameRequestItem>,
    /// Earlier Problems with a recorded root cause in the same code area, as leads to test
    #[serde(default)]
    pub prior: Vec<ProblemRelatedResponsePriorItem>,
}

impl ProblemRelatedResponse {
    pub fn builder() -> ProblemRelatedResponseBuilder {
        <ProblemRelatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemRelatedResponseBuilder {
    duplicate_of: Option<String>,
    duplicates: Option<Vec<ProblemRelatedResponseDuplicatesItem>>,
    same_request: Option<Vec<ProblemRelatedResponseSameRequestItem>>,
    prior: Option<Vec<ProblemRelatedResponsePriorItem>>,
}

impl ProblemRelatedResponseBuilder {
    pub fn duplicate_of(mut self, value: impl Into<String>) -> Self {
        self.duplicate_of = Some(value.into());
        self
    }

    pub fn duplicates(mut self, value: Vec<ProblemRelatedResponseDuplicatesItem>) -> Self {
        self.duplicates = Some(value);
        self
    }

    pub fn same_request(mut self, value: Vec<ProblemRelatedResponseSameRequestItem>) -> Self {
        self.same_request = Some(value);
        self
    }

    pub fn prior(mut self, value: Vec<ProblemRelatedResponsePriorItem>) -> Self {
        self.prior = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemRelatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`duplicates`](ProblemRelatedResponseBuilder::duplicates)
    /// - [`same_request`](ProblemRelatedResponseBuilder::same_request)
    /// - [`prior`](ProblemRelatedResponseBuilder::prior)
    pub fn build(self) -> Result<ProblemRelatedResponse, BuildError> {
        Ok(ProblemRelatedResponse {
            duplicate_of: self.duplicate_of,
            duplicates: self.duplicates.ok_or_else(|| BuildError::missing_field("duplicates"))?,
            same_request: self.same_request.ok_or_else(|| BuildError::missing_field("same_request"))?,
            prior: self.prior.ok_or_else(|| BuildError::missing_field("prior"))?,
        })
    }
}

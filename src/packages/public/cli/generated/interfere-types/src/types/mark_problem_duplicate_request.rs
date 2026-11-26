pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkProblemDuplicateRequest {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "keeperProblemId")]
    #[serde(default)]
    pub keeper_problem_id: String,
}

impl MarkProblemDuplicateRequest {
    pub fn builder() -> MarkProblemDuplicateRequestBuilder {
        <MarkProblemDuplicateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkProblemDuplicateRequestBuilder {
    request_id: Option<String>,
    keeper_problem_id: Option<String>,
}

impl MarkProblemDuplicateRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn keeper_problem_id(mut self, value: impl Into<String>) -> Self {
        self.keeper_problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MarkProblemDuplicateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](MarkProblemDuplicateRequestBuilder::request_id)
    /// - [`keeper_problem_id`](MarkProblemDuplicateRequestBuilder::keeper_problem_id)
    pub fn build(self) -> Result<MarkProblemDuplicateRequest, BuildError> {
        Ok(MarkProblemDuplicateRequest {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            keeper_problem_id: self.keeper_problem_id.ok_or_else(|| BuildError::missing_field("keeper_problem_id"))?,
        })
    }
}


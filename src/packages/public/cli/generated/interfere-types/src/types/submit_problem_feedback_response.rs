pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitProblemFeedbackResponse {
    #[serde(default)]
    pub success: String,
}

impl SubmitProblemFeedbackResponse {
    pub fn builder() -> SubmitProblemFeedbackResponseBuilder {
        <SubmitProblemFeedbackResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitProblemFeedbackResponseBuilder {
    success: Option<String>,
}

impl SubmitProblemFeedbackResponseBuilder {
    pub fn success(mut self, value: impl Into<String>) -> Self {
        self.success = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitProblemFeedbackResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](SubmitProblemFeedbackResponseBuilder::success)
    pub fn build(self) -> Result<SubmitProblemFeedbackResponse, BuildError> {
        Ok(SubmitProblemFeedbackResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitWorkspaceFeedbackResponse {
    #[serde(default)]
    pub success: String,
}

impl SubmitWorkspaceFeedbackResponse {
    pub fn builder() -> SubmitWorkspaceFeedbackResponseBuilder {
        <SubmitWorkspaceFeedbackResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitWorkspaceFeedbackResponseBuilder {
    success: Option<String>,
}

impl SubmitWorkspaceFeedbackResponseBuilder {
    pub fn success(mut self, value: impl Into<String>) -> Self {
        self.success = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitWorkspaceFeedbackResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](SubmitWorkspaceFeedbackResponseBuilder::success)
    pub fn build(self) -> Result<SubmitWorkspaceFeedbackResponse, BuildError> {
        Ok(SubmitWorkspaceFeedbackResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

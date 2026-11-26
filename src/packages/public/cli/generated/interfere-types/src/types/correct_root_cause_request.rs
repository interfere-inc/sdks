pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CorrectRootCauseRequest {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    pub cause: CorrectRootCauseRequestCause,
    pub reason: CorrectRootCauseRequestReason,
    #[serde(rename = "completeEnding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_ending: Option<bool>,
}

impl CorrectRootCauseRequest {
    pub fn builder() -> CorrectRootCauseRequestBuilder {
        <CorrectRootCauseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorrectRootCauseRequestBuilder {
    request_id: Option<String>,
    cause: Option<CorrectRootCauseRequestCause>,
    reason: Option<CorrectRootCauseRequestReason>,
    complete_ending: Option<bool>,
}

impl CorrectRootCauseRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn cause(mut self, value: CorrectRootCauseRequestCause) -> Self {
        self.cause = Some(value);
        self
    }

    pub fn reason(mut self, value: CorrectRootCauseRequestReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn complete_ending(mut self, value: bool) -> Self {
        self.complete_ending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorrectRootCauseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](CorrectRootCauseRequestBuilder::request_id)
    /// - [`cause`](CorrectRootCauseRequestBuilder::cause)
    /// - [`reason`](CorrectRootCauseRequestBuilder::reason)
    pub fn build(self) -> Result<CorrectRootCauseRequest, BuildError> {
        Ok(CorrectRootCauseRequest {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            cause: self.cause.ok_or_else(|| BuildError::missing_field("cause"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            complete_ending: self.complete_ending,
        })
    }
}


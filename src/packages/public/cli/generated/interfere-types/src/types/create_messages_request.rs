pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMessagesRequest {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub message: String,
}

impl CreateMessagesRequest {
    pub fn builder() -> CreateMessagesRequestBuilder {
        <CreateMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMessagesRequestBuilder {
    request_id: Option<String>,
    message: Option<String>,
}

impl CreateMessagesRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](CreateMessagesRequestBuilder::request_id)
    /// - [`message`](CreateMessagesRequestBuilder::message)
    pub fn build(self) -> Result<CreateMessagesRequest, BuildError> {
        Ok(CreateMessagesRequest {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateThreadsRequest {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<CreateThreadsRequestPriority>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub message: String,
}

impl CreateThreadsRequest {
    pub fn builder() -> CreateThreadsRequestBuilder {
        <CreateThreadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateThreadsRequestBuilder {
    request_id: Option<String>,
    priority: Option<CreateThreadsRequestPriority>,
    title: Option<String>,
    message: Option<String>,
}

impl CreateThreadsRequestBuilder {
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn priority(mut self, value: CreateThreadsRequestPriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateThreadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_id`](CreateThreadsRequestBuilder::request_id)
    /// - [`title`](CreateThreadsRequestBuilder::title)
    /// - [`message`](CreateThreadsRequestBuilder::message)
    pub fn build(self) -> Result<CreateThreadsRequest, BuildError> {
        Ok(CreateThreadsRequest {
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            priority: self.priority,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubmitWorkspaceFeedbackRequest {
    #[serde(rename = "issueId")]
    #[serde(default)]
    pub issue_id: String,
    /// What the feedback is about: `feature` requests a product change, `bug` reports a defect in Interfere, `integration` requests an integration, and `missed` reports a problem in the workspace's own product that Interfere failed to detect.
    pub kind: SubmitWorkspaceFeedbackRequestKind,
    #[serde(default)]
    pub message: String,
    #[serde(rename = "needId")]
    #[serde(default)]
    pub need_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SubmitWorkspaceFeedbackRequestAttachmentsItem>>,
}

impl SubmitWorkspaceFeedbackRequest {
    pub fn builder() -> SubmitWorkspaceFeedbackRequestBuilder {
        <SubmitWorkspaceFeedbackRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitWorkspaceFeedbackRequestBuilder {
    issue_id: Option<String>,
    kind: Option<SubmitWorkspaceFeedbackRequestKind>,
    message: Option<String>,
    need_id: Option<String>,
    title: Option<String>,
    attachments: Option<Vec<SubmitWorkspaceFeedbackRequestAttachmentsItem>>,
}

impl SubmitWorkspaceFeedbackRequestBuilder {
    pub fn issue_id(mut self, value: impl Into<String>) -> Self {
        self.issue_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: SubmitWorkspaceFeedbackRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn need_id(mut self, value: impl Into<String>) -> Self {
        self.need_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn attachments(mut self, value: Vec<SubmitWorkspaceFeedbackRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitWorkspaceFeedbackRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`issue_id`](SubmitWorkspaceFeedbackRequestBuilder::issue_id)
    /// - [`kind`](SubmitWorkspaceFeedbackRequestBuilder::kind)
    /// - [`message`](SubmitWorkspaceFeedbackRequestBuilder::message)
    /// - [`need_id`](SubmitWorkspaceFeedbackRequestBuilder::need_id)
    pub fn build(self) -> Result<SubmitWorkspaceFeedbackRequest, BuildError> {
        Ok(SubmitWorkspaceFeedbackRequest {
            issue_id: self.issue_id.ok_or_else(|| BuildError::missing_field("issue_id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            need_id: self.need_id.ok_or_else(|| BuildError::missing_field("need_id"))?,
            title: self.title,
            attachments: self.attachments,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitProblemFeedbackRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<SubmitProblemFeedbackRequestAccuracy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actionability: Option<SubmitProblemFeedbackRequestActionability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "submissionId")]
    #[serde(default)]
    pub submission_id: String,
    #[serde(rename = "submittedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub submitted_at: DateTime<FixedOffset>,
}

impl SubmitProblemFeedbackRequest {
    pub fn builder() -> SubmitProblemFeedbackRequestBuilder {
        <SubmitProblemFeedbackRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitProblemFeedbackRequestBuilder {
    accuracy: Option<SubmitProblemFeedbackRequestAccuracy>,
    actionability: Option<SubmitProblemFeedbackRequestActionability>,
    comment: Option<String>,
    submission_id: Option<String>,
    submitted_at: Option<DateTime<FixedOffset>>,
}

impl SubmitProblemFeedbackRequestBuilder {
    pub fn accuracy(mut self, value: SubmitProblemFeedbackRequestAccuracy) -> Self {
        self.accuracy = Some(value);
        self
    }

    pub fn actionability(mut self, value: SubmitProblemFeedbackRequestActionability) -> Self {
        self.actionability = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    pub fn submission_id(mut self, value: impl Into<String>) -> Self {
        self.submission_id = Some(value.into());
        self
    }

    pub fn submitted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.submitted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitProblemFeedbackRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`submission_id`](SubmitProblemFeedbackRequestBuilder::submission_id)
    /// - [`submitted_at`](SubmitProblemFeedbackRequestBuilder::submitted_at)
    pub fn build(self) -> Result<SubmitProblemFeedbackRequest, BuildError> {
        Ok(SubmitProblemFeedbackRequest {
            accuracy: self.accuracy,
            actionability: self.actionability,
            comment: self.comment,
            submission_id: self.submission_id.ok_or_else(|| BuildError::missing_field("submission_id"))?,
            submitted_at: self.submitted_at.ok_or_else(|| BuildError::missing_field("submitted_at"))?,
        })
    }
}


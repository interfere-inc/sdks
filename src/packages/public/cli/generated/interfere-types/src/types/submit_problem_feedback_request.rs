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
    /// Retry key for one submission. Omit it to keep a single feedback per user and problem, which a later submission replaces.
    #[serde(rename = "submissionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_id: Option<String>,
    /// Ignored. The server records the submission time.
    #[serde(rename = "submittedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub submitted_at: Option<DateTime<FixedOffset>>,
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
    pub fn build(self) -> Result<SubmitProblemFeedbackRequest, BuildError> {
        Ok(SubmitProblemFeedbackRequest {
            accuracy: self.accuracy,
            actionability: self.actionability,
            comment: self.comment,
            submission_id: self.submission_id,
            submitted_at: self.submitted_at,
        })
    }
}


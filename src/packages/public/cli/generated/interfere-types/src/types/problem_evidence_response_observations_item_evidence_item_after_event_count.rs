pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount {
    #[serde(rename = "afterEventCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_event_count: Option<i64>,
    #[serde(rename = "afterLimit")]
    #[serde(default)]
    pub after_limit: i64,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountKind,
    #[serde(default)]
    pub limit: i64,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder {
    after_event_count: Option<i64>,
    after_limit: Option<i64>,
    content_digest: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountKind>,
    limit: Option<i64>,
    session_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder {
    pub fn after_event_count(mut self, value: i64) -> Self {
        self.after_event_count = Some(value);
        self
    }

    pub fn after_limit(mut self, value: i64) -> Self {
        self.after_limit = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`after_limit`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder::after_limit)
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder::content_digest)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder::kind)
    /// - [`limit`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder::limit)
    /// - [`session_id`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCountBuilder::session_id)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemAfterEventCount {
            after_event_count: self.after_event_count,
            after_limit: self.after_limit.ok_or_else(|| BuildError::missing_field("after_limit"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit {
    #[serde(rename = "afterLimit")]
    #[serde(default)]
    pub after_limit: i64,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitKind,
    #[serde(default)]
    pub limit: i64,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder {
    after_limit: Option<i64>,
    content_digest: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitKind>,
    limit: Option<i64>,
    session_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder {
    pub fn after_limit(mut self, value: i64) -> Self {
        self.after_limit = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitKind) -> Self {
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

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`after_limit`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder::after_limit)
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder::content_digest)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder::kind)
    /// - [`limit`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder::limit)
    /// - [`session_id`](ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimitBuilder::session_id)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemAfterLimit {
            after_limit: self.after_limit.ok_or_else(|| BuildError::missing_field("after_limit"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}

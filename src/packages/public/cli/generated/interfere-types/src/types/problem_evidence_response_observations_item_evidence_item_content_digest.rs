pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest {
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestKind,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub since: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub until: i64,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder {
    content_digest: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestKind>,
    since: Option<i64>,
    until: Option<i64>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder {
    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn since(mut self, value: i64) -> Self {
        self.since = Some(value);
        self
    }

    pub fn until(mut self, value: i64) -> Self {
        self.until = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder::content_digest)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder::kind)
    /// - [`since`](ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder::since)
    /// - [`until`](ProblemEvidenceResponseObservationsItemEvidenceItemContentDigestBuilder::until)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemContentDigest {
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            since: self.since.ok_or_else(|| BuildError::missing_field("since"))?,
            until: self.until.ok_or_else(|| BuildError::missing_field("until"))?,
        })
    }
}

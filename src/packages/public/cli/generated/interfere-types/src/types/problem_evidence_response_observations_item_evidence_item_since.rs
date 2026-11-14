pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSince {
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemSinceKind,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub since: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub until: i64,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSince {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder {
    content_digest: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemSinceKind>,
    since: Option<i64>,
    until: Option<i64>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder {
    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemSinceKind) -> Self {
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

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemSince`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder::content_digest)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder::kind)
    /// - [`since`](ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder::since)
    /// - [`until`](ProblemEvidenceResponseObservationsItemEvidenceItemSinceBuilder::until)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemSince, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemSince {
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            since: self.since.ok_or_else(|| BuildError::missing_field("since"))?,
            until: self.until.ok_or_else(|| BuildError::missing_field("until"))?,
        })
    }
}

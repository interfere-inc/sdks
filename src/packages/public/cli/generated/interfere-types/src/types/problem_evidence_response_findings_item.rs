pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseFindingsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(rename = "errorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstSeenAt")]
    #[serde(default)]
    pub first_seen_at: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    pub last_seen_at: i64,
}

impl ProblemEvidenceResponseFindingsItem {
    pub fn builder() -> ProblemEvidenceResponseFindingsItemBuilder {
        <ProblemEvidenceResponseFindingsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseFindingsItemBuilder {
    id: Option<String>,
    fingerprint: Option<String>,
    error_name: Option<String>,
    environment: Option<String>,
    first_seen_at: Option<i64>,
    last_seen_at: Option<i64>,
}

impl ProblemEvidenceResponseFindingsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn error_name(mut self, value: impl Into<String>) -> Self {
        self.error_name = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn first_seen_at(mut self, value: i64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn last_seen_at(mut self, value: i64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseFindingsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProblemEvidenceResponseFindingsItemBuilder::id)
    /// - [`fingerprint`](ProblemEvidenceResponseFindingsItemBuilder::fingerprint)
    /// - [`first_seen_at`](ProblemEvidenceResponseFindingsItemBuilder::first_seen_at)
    /// - [`last_seen_at`](ProblemEvidenceResponseFindingsItemBuilder::last_seen_at)
    pub fn build(self) -> Result<ProblemEvidenceResponseFindingsItem, BuildError> {
        Ok(ProblemEvidenceResponseFindingsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            error_name: self.error_name,
            environment: self.environment,
            first_seen_at: self.first_seen_at.ok_or_else(|| BuildError::missing_field("first_seen_at"))?,
            last_seen_at: self.last_seen_at.ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
        })
    }
}

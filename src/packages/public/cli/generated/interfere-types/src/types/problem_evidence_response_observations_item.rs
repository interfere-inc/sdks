pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProblemEvidenceResponseObservationsItem {
    #[serde(default)]
    pub sequence: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub evidence: Vec<ProblemEvidenceResponseObservationsItemEvidenceItem>,
}

impl ProblemEvidenceResponseObservationsItem {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemBuilder {
        <ProblemEvidenceResponseObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemBuilder {
    sequence: Option<i64>,
    text: Option<String>,
    created_at: Option<i64>,
    evidence: Option<Vec<ProblemEvidenceResponseObservationsItemEvidenceItem>>,
}

impl ProblemEvidenceResponseObservationsItemBuilder {
    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn evidence(mut self, value: Vec<ProblemEvidenceResponseObservationsItemEvidenceItem>) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sequence`](ProblemEvidenceResponseObservationsItemBuilder::sequence)
    /// - [`created_at`](ProblemEvidenceResponseObservationsItemBuilder::created_at)
    /// - [`evidence`](ProblemEvidenceResponseObservationsItemBuilder::evidence)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItem, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItem {
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
            text: self.text,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}

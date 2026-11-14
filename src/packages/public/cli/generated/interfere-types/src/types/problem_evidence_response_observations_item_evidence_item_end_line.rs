pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemEndLine {
    #[serde(rename = "endLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemEndLineKind,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub r#ref: String,
    #[serde(default)]
    pub repository: String,
    #[serde(rename = "snapshotKey")]
    #[serde(default)]
    pub snapshot_key: String,
    #[serde(rename = "startLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEndLineProvenance>,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemEndLine {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder {
    end_line: Option<i64>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEndLineKind>,
    path: Option<String>,
    r#ref: Option<String>,
    repository: Option<String>,
    snapshot_key: Option<String>,
    start_line: Option<i64>,
    provenance: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEndLineProvenance>,
    surface_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder {
    pub fn end_line(mut self, value: i64) -> Self {
        self.end_line = Some(value);
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemEndLineKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    pub fn repository(mut self, value: impl Into<String>) -> Self {
        self.repository = Some(value.into());
        self
    }

    pub fn snapshot_key(mut self, value: impl Into<String>) -> Self {
        self.snapshot_key = Some(value.into());
        self
    }

    pub fn start_line(mut self, value: i64) -> Self {
        self.start_line = Some(value);
        self
    }

    pub fn provenance(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemEndLineProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemEndLine`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder::kind)
    /// - [`path`](ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder::path)
    /// - [`r#ref`](ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder::r#ref)
    /// - [`repository`](ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder::repository)
    /// - [`snapshot_key`](ProblemEvidenceResponseObservationsItemEvidenceItemEndLineBuilder::snapshot_key)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemEndLine, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemEndLine {
            end_line: self.end_line,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            r#ref: self.r#ref.ok_or_else(|| BuildError::missing_field("r#ref"))?,
            repository: self.repository.ok_or_else(|| BuildError::missing_field("repository"))?,
            snapshot_key: self.snapshot_key.ok_or_else(|| BuildError::missing_field("snapshot_key"))?,
            start_line: self.start_line,
            provenance: self.provenance,
            surface_id: self.surface_id,
        })
    }
}

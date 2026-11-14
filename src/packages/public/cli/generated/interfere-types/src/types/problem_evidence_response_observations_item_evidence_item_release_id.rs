pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdKind,
    #[serde(rename = "releaseId")]
    #[serde(default)]
    pub release_id: String,
    #[serde(rename = "surfaceChangeIds")]
    #[serde(default)]
    pub surface_change_ids: Vec<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdKind>,
    release_id: Option<String>,
    surface_change_ids: Option<Vec<String>>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn release_id(mut self, value: impl Into<String>) -> Self {
        self.release_id = Some(value.into());
        self
    }

    pub fn surface_change_ids(mut self, value: Vec<String>) -> Self {
        self.surface_change_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdBuilder::kind)
    /// - [`release_id`](ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdBuilder::release_id)
    /// - [`surface_change_ids`](ProblemEvidenceResponseObservationsItemEvidenceItemReleaseIdBuilder::surface_change_ids)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemReleaseId {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            release_id: self.release_id.ok_or_else(|| BuildError::missing_field("release_id"))?,
            surface_change_ids: self.surface_change_ids.ok_or_else(|| BuildError::missing_field("surface_change_ids"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame {
    #[serde(rename = "frameIndex")]
    #[serde(default)]
    pub frame_index: i64,
    #[serde(rename = "mapHash")]
    #[serde(default)]
    pub map_hash: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrameBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrameBuilder {
    frame_index: Option<i64>,
    map_hash: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrameBuilder {
    pub fn frame_index(mut self, value: i64) -> Self {
        self.frame_index = Some(value);
        self
    }

    pub fn map_hash(mut self, value: impl Into<String>) -> Self {
        self.map_hash = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame`].
    /// This method will fail if any of the following fields are not set:
    /// - [`frame_index`](ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrameBuilder::frame_index)
    /// - [`map_hash`](ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrameBuilder::map_hash)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame {
            frame_index: self.frame_index.ok_or_else(|| BuildError::missing_field("frame_index"))?,
            map_hash: self.map_hash.ok_or_else(|| BuildError::missing_field("map_hash"))?,
        })
    }
}

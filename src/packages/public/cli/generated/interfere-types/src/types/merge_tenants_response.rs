pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeTenantsResponse {
    #[serde(default)]
    pub keep: MergeTenantsResponseKeep,
    #[serde(default)]
    pub absorb: MergeTenantsResponseAbsorb,
    #[serde(rename = "alreadyMerged")]
    #[serde(default)]
    pub already_merged: bool,
    #[serde(default)]
    pub moved: MergeTenantsResponseMoved,
}

impl MergeTenantsResponse {
    pub fn builder() -> MergeTenantsResponseBuilder {
        <MergeTenantsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeTenantsResponseBuilder {
    keep: Option<MergeTenantsResponseKeep>,
    absorb: Option<MergeTenantsResponseAbsorb>,
    already_merged: Option<bool>,
    moved: Option<MergeTenantsResponseMoved>,
}

impl MergeTenantsResponseBuilder {
    pub fn keep(mut self, value: MergeTenantsResponseKeep) -> Self {
        self.keep = Some(value);
        self
    }

    pub fn absorb(mut self, value: MergeTenantsResponseAbsorb) -> Self {
        self.absorb = Some(value);
        self
    }

    pub fn already_merged(mut self, value: bool) -> Self {
        self.already_merged = Some(value);
        self
    }

    pub fn moved(mut self, value: MergeTenantsResponseMoved) -> Self {
        self.moved = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeTenantsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`keep`](MergeTenantsResponseBuilder::keep)
    /// - [`absorb`](MergeTenantsResponseBuilder::absorb)
    /// - [`already_merged`](MergeTenantsResponseBuilder::already_merged)
    /// - [`moved`](MergeTenantsResponseBuilder::moved)
    pub fn build(self) -> Result<MergeTenantsResponse, BuildError> {
        Ok(MergeTenantsResponse {
            keep: self.keep.ok_or_else(|| BuildError::missing_field("keep"))?,
            absorb: self.absorb.ok_or_else(|| BuildError::missing_field("absorb"))?,
            already_merged: self.already_merged.ok_or_else(|| BuildError::missing_field("already_merged"))?,
            moved: self.moved.ok_or_else(|| BuildError::missing_field("moved"))?,
        })
    }
}

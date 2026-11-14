pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeTenantsRequest {
    #[serde(rename = "keepId")]
    #[serde(default)]
    pub keep_id: String,
    #[serde(rename = "absorbId")]
    #[serde(default)]
    pub absorb_id: String,
}

impl MergeTenantsRequest {
    pub fn builder() -> MergeTenantsRequestBuilder {
        <MergeTenantsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeTenantsRequestBuilder {
    keep_id: Option<String>,
    absorb_id: Option<String>,
}

impl MergeTenantsRequestBuilder {
    pub fn keep_id(mut self, value: impl Into<String>) -> Self {
        self.keep_id = Some(value.into());
        self
    }

    pub fn absorb_id(mut self, value: impl Into<String>) -> Self {
        self.absorb_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MergeTenantsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`keep_id`](MergeTenantsRequestBuilder::keep_id)
    /// - [`absorb_id`](MergeTenantsRequestBuilder::absorb_id)
    pub fn build(self) -> Result<MergeTenantsRequest, BuildError> {
        Ok(MergeTenantsRequest {
            keep_id: self.keep_id.ok_or_else(|| BuildError::missing_field("keep_id"))?,
            absorb_id: self.absorb_id.ok_or_else(|| BuildError::missing_field("absorb_id"))?,
        })
    }
}


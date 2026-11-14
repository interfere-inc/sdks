pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTiersRequest {
    #[serde(default)]
    pub patch: UpdateTiersRequestPatch,
}

impl UpdateTiersRequest {
    pub fn builder() -> UpdateTiersRequestBuilder {
        <UpdateTiersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTiersRequestBuilder {
    patch: Option<UpdateTiersRequestPatch>,
}

impl UpdateTiersRequestBuilder {
    pub fn patch(mut self, value: UpdateTiersRequestPatch) -> Self {
        self.patch = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateTiersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`patch`](UpdateTiersRequestBuilder::patch)
    pub fn build(self) -> Result<UpdateTiersRequest, BuildError> {
        Ok(UpdateTiersRequest {
            patch: self.patch.ok_or_else(|| BuildError::missing_field("patch"))?,
        })
    }
}


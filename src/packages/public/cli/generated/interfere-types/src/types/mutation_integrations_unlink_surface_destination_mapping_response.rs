pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationIntegrationsUnlinkSurfaceDestinationMappingResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationIntegrationsUnlinkSurfaceDestinationMappingResponse {
    pub fn builder() -> MutationIntegrationsUnlinkSurfaceDestinationMappingResponseBuilder {
        <MutationIntegrationsUnlinkSurfaceDestinationMappingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationIntegrationsUnlinkSurfaceDestinationMappingResponseBuilder {
    success: Option<bool>,
}

impl MutationIntegrationsUnlinkSurfaceDestinationMappingResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationIntegrationsUnlinkSurfaceDestinationMappingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationIntegrationsUnlinkSurfaceDestinationMappingResponseBuilder::success)
    pub fn build(self) -> Result<MutationIntegrationsUnlinkSurfaceDestinationMappingResponse, BuildError> {
        Ok(MutationIntegrationsUnlinkSurfaceDestinationMappingResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

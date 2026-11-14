pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationIntegrationsSetSurfaceDestinationMappingResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationIntegrationsSetSurfaceDestinationMappingResponse {
    pub fn builder() -> MutationIntegrationsSetSurfaceDestinationMappingResponseBuilder {
        <MutationIntegrationsSetSurfaceDestinationMappingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationIntegrationsSetSurfaceDestinationMappingResponseBuilder {
    success: Option<bool>,
}

impl MutationIntegrationsSetSurfaceDestinationMappingResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationIntegrationsSetSurfaceDestinationMappingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationIntegrationsSetSurfaceDestinationMappingResponseBuilder::success)
    pub fn build(self) -> Result<MutationIntegrationsSetSurfaceDestinationMappingResponse, BuildError> {
        Ok(MutationIntegrationsSetSurfaceDestinationMappingResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

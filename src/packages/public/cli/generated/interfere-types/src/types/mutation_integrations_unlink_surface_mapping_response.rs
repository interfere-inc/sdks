pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationIntegrationsUnlinkSurfaceMappingResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationIntegrationsUnlinkSurfaceMappingResponse {
    pub fn builder() -> MutationIntegrationsUnlinkSurfaceMappingResponseBuilder {
        <MutationIntegrationsUnlinkSurfaceMappingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationIntegrationsUnlinkSurfaceMappingResponseBuilder {
    success: Option<bool>,
}

impl MutationIntegrationsUnlinkSurfaceMappingResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationIntegrationsUnlinkSurfaceMappingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationIntegrationsUnlinkSurfaceMappingResponseBuilder::success)
    pub fn build(self) -> Result<MutationIntegrationsUnlinkSurfaceMappingResponse, BuildError> {
        Ok(MutationIntegrationsUnlinkSurfaceMappingResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

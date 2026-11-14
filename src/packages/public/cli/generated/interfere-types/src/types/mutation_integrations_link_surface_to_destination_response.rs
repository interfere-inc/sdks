pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationIntegrationsLinkSurfaceToDestinationResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationIntegrationsLinkSurfaceToDestinationResponse {
    pub fn builder() -> MutationIntegrationsLinkSurfaceToDestinationResponseBuilder {
        <MutationIntegrationsLinkSurfaceToDestinationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationIntegrationsLinkSurfaceToDestinationResponseBuilder {
    success: Option<bool>,
}

impl MutationIntegrationsLinkSurfaceToDestinationResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationIntegrationsLinkSurfaceToDestinationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationIntegrationsLinkSurfaceToDestinationResponseBuilder::success)
    pub fn build(self) -> Result<MutationIntegrationsLinkSurfaceToDestinationResponse, BuildError> {
        Ok(MutationIntegrationsLinkSurfaceToDestinationResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

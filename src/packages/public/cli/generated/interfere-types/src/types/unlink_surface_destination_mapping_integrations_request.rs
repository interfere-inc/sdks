pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnlinkSurfaceDestinationMappingIntegrationsRequest {
    #[serde(default)]
    pub args: UnlinkSurfaceDestinationMappingIntegrationsRequestArgs,
}

impl UnlinkSurfaceDestinationMappingIntegrationsRequest {
    pub fn builder() -> UnlinkSurfaceDestinationMappingIntegrationsRequestBuilder {
        <UnlinkSurfaceDestinationMappingIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnlinkSurfaceDestinationMappingIntegrationsRequestBuilder {
    args: Option<UnlinkSurfaceDestinationMappingIntegrationsRequestArgs>,
}

impl UnlinkSurfaceDestinationMappingIntegrationsRequestBuilder {
    pub fn args(mut self, value: UnlinkSurfaceDestinationMappingIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnlinkSurfaceDestinationMappingIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UnlinkSurfaceDestinationMappingIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<UnlinkSurfaceDestinationMappingIntegrationsRequest, BuildError> {
        Ok(UnlinkSurfaceDestinationMappingIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnlinkSurfaceMappingIntegrationsRequest {
    #[serde(default)]
    pub args: UnlinkSurfaceMappingIntegrationsRequestArgs,
}

impl UnlinkSurfaceMappingIntegrationsRequest {
    pub fn builder() -> UnlinkSurfaceMappingIntegrationsRequestBuilder {
        <UnlinkSurfaceMappingIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnlinkSurfaceMappingIntegrationsRequestBuilder {
    args: Option<UnlinkSurfaceMappingIntegrationsRequestArgs>,
}

impl UnlinkSurfaceMappingIntegrationsRequestBuilder {
    pub fn args(mut self, value: UnlinkSurfaceMappingIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnlinkSurfaceMappingIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UnlinkSurfaceMappingIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<UnlinkSurfaceMappingIntegrationsRequest, BuildError> {
        Ok(UnlinkSurfaceMappingIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


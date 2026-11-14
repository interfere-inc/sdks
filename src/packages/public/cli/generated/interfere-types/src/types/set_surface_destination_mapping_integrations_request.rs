pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetSurfaceDestinationMappingIntegrationsRequest {
    #[serde(default)]
    pub args: SetSurfaceDestinationMappingIntegrationsRequestArgs,
}

impl SetSurfaceDestinationMappingIntegrationsRequest {
    pub fn builder() -> SetSurfaceDestinationMappingIntegrationsRequestBuilder {
        <SetSurfaceDestinationMappingIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetSurfaceDestinationMappingIntegrationsRequestBuilder {
    args: Option<SetSurfaceDestinationMappingIntegrationsRequestArgs>,
}

impl SetSurfaceDestinationMappingIntegrationsRequestBuilder {
    pub fn args(mut self, value: SetSurfaceDestinationMappingIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetSurfaceDestinationMappingIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetSurfaceDestinationMappingIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<SetSurfaceDestinationMappingIntegrationsRequest, BuildError> {
        Ok(SetSurfaceDestinationMappingIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


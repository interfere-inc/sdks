pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkSurfaceToDestinationIntegrationsRequest {
    #[serde(default)]
    pub args: LinkSurfaceToDestinationIntegrationsRequestArgs,
}

impl LinkSurfaceToDestinationIntegrationsRequest {
    pub fn builder() -> LinkSurfaceToDestinationIntegrationsRequestBuilder {
        <LinkSurfaceToDestinationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkSurfaceToDestinationIntegrationsRequestBuilder {
    args: Option<LinkSurfaceToDestinationIntegrationsRequestArgs>,
}

impl LinkSurfaceToDestinationIntegrationsRequestBuilder {
    pub fn args(mut self, value: LinkSurfaceToDestinationIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinkSurfaceToDestinationIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](LinkSurfaceToDestinationIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<LinkSurfaceToDestinationIntegrationsRequest, BuildError> {
        Ok(LinkSurfaceToDestinationIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


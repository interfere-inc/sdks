pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSurfaceMappingIntegrationsRequest {
    #[serde(default)]
    pub args: UpdateSurfaceMappingIntegrationsRequestArgs,
}

impl UpdateSurfaceMappingIntegrationsRequest {
    pub fn builder() -> UpdateSurfaceMappingIntegrationsRequestBuilder {
        <UpdateSurfaceMappingIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSurfaceMappingIntegrationsRequestBuilder {
    args: Option<UpdateSurfaceMappingIntegrationsRequestArgs>,
}

impl UpdateSurfaceMappingIntegrationsRequestBuilder {
    pub fn args(mut self, value: UpdateSurfaceMappingIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSurfaceMappingIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateSurfaceMappingIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<UpdateSurfaceMappingIntegrationsRequest, BuildError> {
        Ok(UpdateSurfaceMappingIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


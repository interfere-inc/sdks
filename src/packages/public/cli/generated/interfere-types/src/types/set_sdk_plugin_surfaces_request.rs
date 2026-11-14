pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetSdkPluginSurfacesRequest {
    pub args: SetSdkPluginSurfacesRequestArgs,
}

impl SetSdkPluginSurfacesRequest {
    pub fn builder() -> SetSdkPluginSurfacesRequestBuilder {
        <SetSdkPluginSurfacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetSdkPluginSurfacesRequestBuilder {
    args: Option<SetSdkPluginSurfacesRequestArgs>,
}

impl SetSdkPluginSurfacesRequestBuilder {
    pub fn args(mut self, value: SetSdkPluginSurfacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetSdkPluginSurfacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetSdkPluginSurfacesRequestBuilder::args)
    pub fn build(self) -> Result<SetSdkPluginSurfacesRequest, BuildError> {
        Ok(SetSdkPluginSurfacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


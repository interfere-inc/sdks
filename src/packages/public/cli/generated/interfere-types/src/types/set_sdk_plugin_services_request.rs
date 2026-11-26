pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetSdkPluginServicesRequest {
    pub args: SetSdkPluginServicesRequestArgs,
}

impl SetSdkPluginServicesRequest {
    pub fn builder() -> SetSdkPluginServicesRequestBuilder {
        <SetSdkPluginServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetSdkPluginServicesRequestBuilder {
    args: Option<SetSdkPluginServicesRequestArgs>,
}

impl SetSdkPluginServicesRequestBuilder {
    pub fn args(mut self, value: SetSdkPluginServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetSdkPluginServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetSdkPluginServicesRequestBuilder::args)
    pub fn build(self) -> Result<SetSdkPluginServicesRequest, BuildError> {
        Ok(SetSdkPluginServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


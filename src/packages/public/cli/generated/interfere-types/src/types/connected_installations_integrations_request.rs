pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectedInstallationsIntegrationsRequest {
    #[serde(default)]
    pub args: ConnectedInstallationsIntegrationsRequestArgs,
}

impl ConnectedInstallationsIntegrationsRequest {
    pub fn builder() -> ConnectedInstallationsIntegrationsRequestBuilder {
        <ConnectedInstallationsIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectedInstallationsIntegrationsRequestBuilder {
    args: Option<ConnectedInstallationsIntegrationsRequestArgs>,
}

impl ConnectedInstallationsIntegrationsRequestBuilder {
    pub fn args(mut self, value: ConnectedInstallationsIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectedInstallationsIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ConnectedInstallationsIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<ConnectedInstallationsIntegrationsRequest, BuildError> {
        Ok(ConnectedInstallationsIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


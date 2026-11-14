pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InstallationsIntegrationsRequest {
    #[serde(default)]
    pub args: InstallationsIntegrationsRequestArgs,
}

impl InstallationsIntegrationsRequest {
    pub fn builder() -> InstallationsIntegrationsRequestBuilder {
        <InstallationsIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallationsIntegrationsRequestBuilder {
    args: Option<InstallationsIntegrationsRequestArgs>,
}

impl InstallationsIntegrationsRequestBuilder {
    pub fn args(mut self, value: InstallationsIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InstallationsIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](InstallationsIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<InstallationsIntegrationsRequest, BuildError> {
        Ok(InstallationsIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


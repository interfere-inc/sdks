pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InstallationIntegrationsRequest {
    #[serde(default)]
    pub args: InstallationIntegrationsRequestArgs,
}

impl InstallationIntegrationsRequest {
    pub fn builder() -> InstallationIntegrationsRequestBuilder {
        <InstallationIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallationIntegrationsRequestBuilder {
    args: Option<InstallationIntegrationsRequestArgs>,
}

impl InstallationIntegrationsRequestBuilder {
    pub fn args(mut self, value: InstallationIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InstallationIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](InstallationIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<InstallationIntegrationsRequest, BuildError> {
        Ok(InstallationIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


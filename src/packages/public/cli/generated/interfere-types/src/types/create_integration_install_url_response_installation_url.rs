pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateIntegrationInstallUrlResponseInstallationUrl {
    pub kind: CreateIntegrationInstallUrlResponseInstallationUrlKind,
    #[serde(rename = "installationUrl")]
    #[serde(default)]
    pub installation_url: String,
}

impl CreateIntegrationInstallUrlResponseInstallationUrl {
    pub fn builder() -> CreateIntegrationInstallUrlResponseInstallationUrlBuilder {
        <CreateIntegrationInstallUrlResponseInstallationUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateIntegrationInstallUrlResponseInstallationUrlBuilder {
    kind: Option<CreateIntegrationInstallUrlResponseInstallationUrlKind>,
    installation_url: Option<String>,
}

impl CreateIntegrationInstallUrlResponseInstallationUrlBuilder {
    pub fn kind(mut self, value: CreateIntegrationInstallUrlResponseInstallationUrlKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn installation_url(mut self, value: impl Into<String>) -> Self {
        self.installation_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateIntegrationInstallUrlResponseInstallationUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CreateIntegrationInstallUrlResponseInstallationUrlBuilder::kind)
    /// - [`installation_url`](CreateIntegrationInstallUrlResponseInstallationUrlBuilder::installation_url)
    pub fn build(self) -> Result<CreateIntegrationInstallUrlResponseInstallationUrl, BuildError> {
        Ok(CreateIntegrationInstallUrlResponseInstallationUrl {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            installation_url: self.installation_url.ok_or_else(|| BuildError::missing_field("installation_url"))?,
        })
    }
}

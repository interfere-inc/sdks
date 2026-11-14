pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateIntegrationInstallUrlResponseRegisterUrl {
    pub kind: CreateIntegrationInstallUrlResponseRegisterUrlKind,
    #[serde(rename = "registerUrl")]
    #[serde(default)]
    pub register_url: String,
}

impl CreateIntegrationInstallUrlResponseRegisterUrl {
    pub fn builder() -> CreateIntegrationInstallUrlResponseRegisterUrlBuilder {
        <CreateIntegrationInstallUrlResponseRegisterUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateIntegrationInstallUrlResponseRegisterUrlBuilder {
    kind: Option<CreateIntegrationInstallUrlResponseRegisterUrlKind>,
    register_url: Option<String>,
}

impl CreateIntegrationInstallUrlResponseRegisterUrlBuilder {
    pub fn kind(mut self, value: CreateIntegrationInstallUrlResponseRegisterUrlKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn register_url(mut self, value: impl Into<String>) -> Self {
        self.register_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateIntegrationInstallUrlResponseRegisterUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CreateIntegrationInstallUrlResponseRegisterUrlBuilder::kind)
    /// - [`register_url`](CreateIntegrationInstallUrlResponseRegisterUrlBuilder::register_url)
    pub fn build(self) -> Result<CreateIntegrationInstallUrlResponseRegisterUrl, BuildError> {
        Ok(CreateIntegrationInstallUrlResponseRegisterUrl {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            register_url: self.register_url.ok_or_else(|| BuildError::missing_field("register_url"))?,
        })
    }
}

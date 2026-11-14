pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWorkspaceApiKeyResponse {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: CreateWorkspaceApiKeyResponseApiKey,
}

impl CreateWorkspaceApiKeyResponse {
    pub fn builder() -> CreateWorkspaceApiKeyResponseBuilder {
        <CreateWorkspaceApiKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWorkspaceApiKeyResponseBuilder {
    api_key: Option<CreateWorkspaceApiKeyResponseApiKey>,
}

impl CreateWorkspaceApiKeyResponseBuilder {
    pub fn api_key(mut self, value: CreateWorkspaceApiKeyResponseApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWorkspaceApiKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CreateWorkspaceApiKeyResponseBuilder::api_key)
    pub fn build(self) -> Result<CreateWorkspaceApiKeyResponse, BuildError> {
        Ok(CreateWorkspaceApiKeyResponse {
            api_key: self.api_key.ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}

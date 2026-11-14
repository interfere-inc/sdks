pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWorkspaceApiKeyResponseApiKey {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// Private API credential returned only on creation.
    #[serde(default)]
    pub secret: String,
}

impl CreateWorkspaceApiKeyResponseApiKey {
    pub fn builder() -> CreateWorkspaceApiKeyResponseApiKeyBuilder {
        <CreateWorkspaceApiKeyResponseApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWorkspaceApiKeyResponseApiKeyBuilder {
    id: Option<String>,
    name: Option<String>,
    secret: Option<String>,
}

impl CreateWorkspaceApiKeyResponseApiKeyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWorkspaceApiKeyResponseApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateWorkspaceApiKeyResponseApiKeyBuilder::id)
    /// - [`name`](CreateWorkspaceApiKeyResponseApiKeyBuilder::name)
    /// - [`secret`](CreateWorkspaceApiKeyResponseApiKeyBuilder::secret)
    pub fn build(self) -> Result<CreateWorkspaceApiKeyResponseApiKey, BuildError> {
        Ok(CreateWorkspaceApiKeyResponseApiKey {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            secret: self.secret.ok_or_else(|| BuildError::missing_field("secret"))?,
        })
    }
}

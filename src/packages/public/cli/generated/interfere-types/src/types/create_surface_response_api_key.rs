pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateSurfaceResponseApiKey {
    #[serde(default)]
    pub id: String,
    pub name: CreateSurfaceResponseApiKeyName,
    #[serde(default)]
    pub secret: String,
}

impl CreateSurfaceResponseApiKey {
    pub fn builder() -> CreateSurfaceResponseApiKeyBuilder {
        <CreateSurfaceResponseApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSurfaceResponseApiKeyBuilder {
    id: Option<String>,
    name: Option<CreateSurfaceResponseApiKeyName>,
    secret: Option<String>,
}

impl CreateSurfaceResponseApiKeyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: CreateSurfaceResponseApiKeyName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSurfaceResponseApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSurfaceResponseApiKeyBuilder::id)
    /// - [`name`](CreateSurfaceResponseApiKeyBuilder::name)
    /// - [`secret`](CreateSurfaceResponseApiKeyBuilder::secret)
    pub fn build(self) -> Result<CreateSurfaceResponseApiKey, BuildError> {
        Ok(CreateSurfaceResponseApiKey {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            secret: self.secret.ok_or_else(|| BuildError::missing_field("secret"))?,
        })
    }
}

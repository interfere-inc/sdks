pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSurfacePublicKeyResponse {
    /// Publishable credential for browser SDK configuration.
    #[serde(default)]
    pub content: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
}

impl CreateSurfacePublicKeyResponse {
    pub fn builder() -> CreateSurfacePublicKeyResponseBuilder {
        <CreateSurfacePublicKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSurfacePublicKeyResponseBuilder {
    content: Option<String>,
    name: Option<String>,
}

impl CreateSurfacePublicKeyResponseBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSurfacePublicKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](CreateSurfacePublicKeyResponseBuilder::content)
    /// - [`name`](CreateSurfacePublicKeyResponseBuilder::name)
    pub fn build(self) -> Result<CreateSurfacePublicKeyResponse, BuildError> {
        Ok(CreateSurfacePublicKeyResponse {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

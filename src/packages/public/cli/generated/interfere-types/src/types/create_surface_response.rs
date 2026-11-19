pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateSurfaceResponse {
    #[serde(rename = "apiKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<CreateSurfaceResponseApiKey>,
    #[serde(rename = "publicKey")]
    #[serde(default)]
    pub public_key: String,
    pub surface: CreateSurfaceResponseSurface,
}

impl CreateSurfaceResponse {
    pub fn builder() -> CreateSurfaceResponseBuilder {
        <CreateSurfaceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSurfaceResponseBuilder {
    api_key: Option<CreateSurfaceResponseApiKey>,
    public_key: Option<String>,
    surface: Option<CreateSurfaceResponseSurface>,
}

impl CreateSurfaceResponseBuilder {
    pub fn api_key(mut self, value: CreateSurfaceResponseApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn public_key(mut self, value: impl Into<String>) -> Self {
        self.public_key = Some(value.into());
        self
    }

    pub fn surface(mut self, value: CreateSurfaceResponseSurface) -> Self {
        self.surface = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSurfaceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`public_key`](CreateSurfaceResponseBuilder::public_key)
    /// - [`surface`](CreateSurfaceResponseBuilder::surface)
    pub fn build(self) -> Result<CreateSurfaceResponse, BuildError> {
        Ok(CreateSurfaceResponse {
            api_key: self.api_key,
            public_key: self.public_key.ok_or_else(|| BuildError::missing_field("public_key"))?,
            surface: self.surface.ok_or_else(|| BuildError::missing_field("surface"))?,
        })
    }
}

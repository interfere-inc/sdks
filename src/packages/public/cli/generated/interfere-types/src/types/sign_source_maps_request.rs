pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignSourceMapsRequest {
    #[serde(default)]
    pub files: Vec<SignSourceMapFile>,
    /// Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release.
    #[serde(skip)]
    pub environment: Option<String>,
}

impl SignSourceMapsRequest {
    pub fn builder() -> SignSourceMapsRequestBuilder {
        <SignSourceMapsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignSourceMapsRequestBuilder {
    files: Option<Vec<SignSourceMapFile>>,
    environment: Option<String>,
}

impl SignSourceMapsRequestBuilder {
    pub fn files(mut self, value: Vec<SignSourceMapFile>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignSourceMapsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](SignSourceMapsRequestBuilder::files)
    pub fn build(self) -> Result<SignSourceMapsRequest, BuildError> {
        Ok(SignSourceMapsRequest {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
            environment: self.environment,
        })
    }
}


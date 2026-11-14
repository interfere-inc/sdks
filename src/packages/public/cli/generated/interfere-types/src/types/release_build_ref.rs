pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ReleaseBuildRef
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseBuildRef {
    #[serde(default)]
    pub hash: String,
}

impl ReleaseBuildRef {
    pub fn builder() -> ReleaseBuildRefBuilder {
        <ReleaseBuildRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseBuildRefBuilder {
    hash: Option<String>,
}

impl ReleaseBuildRefBuilder {
    pub fn hash(mut self, value: impl Into<String>) -> Self {
        self.hash = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseBuildRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hash`](ReleaseBuildRefBuilder::hash)
    pub fn build(self) -> Result<ReleaseBuildRef, BuildError> {
        Ok(ReleaseBuildRef {
            hash: self.hash.ok_or_else(|| BuildError::missing_field("hash"))?,
        })
    }
}

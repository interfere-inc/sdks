pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationApiKeysUpdateNameResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationApiKeysUpdateNameResponse {
    pub fn builder() -> MutationApiKeysUpdateNameResponseBuilder {
        <MutationApiKeysUpdateNameResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationApiKeysUpdateNameResponseBuilder {
    success: Option<bool>,
}

impl MutationApiKeysUpdateNameResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationApiKeysUpdateNameResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationApiKeysUpdateNameResponseBuilder::success)
    pub fn build(self) -> Result<MutationApiKeysUpdateNameResponse, BuildError> {
        Ok(MutationApiKeysUpdateNameResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

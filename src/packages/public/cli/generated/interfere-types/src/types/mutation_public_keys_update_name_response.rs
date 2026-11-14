pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationPublicKeysUpdateNameResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationPublicKeysUpdateNameResponse {
    pub fn builder() -> MutationPublicKeysUpdateNameResponseBuilder {
        <MutationPublicKeysUpdateNameResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationPublicKeysUpdateNameResponseBuilder {
    success: Option<bool>,
}

impl MutationPublicKeysUpdateNameResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationPublicKeysUpdateNameResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationPublicKeysUpdateNameResponseBuilder::success)
    pub fn build(self) -> Result<MutationPublicKeysUpdateNameResponse, BuildError> {
        Ok(MutationPublicKeysUpdateNameResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesUnlinkRepositoryResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesUnlinkRepositoryResponse {
    pub fn builder() -> MutationServicesUnlinkRepositoryResponseBuilder {
        <MutationServicesUnlinkRepositoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesUnlinkRepositoryResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesUnlinkRepositoryResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesUnlinkRepositoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesUnlinkRepositoryResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesUnlinkRepositoryResponse, BuildError> {
        Ok(MutationServicesUnlinkRepositoryResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

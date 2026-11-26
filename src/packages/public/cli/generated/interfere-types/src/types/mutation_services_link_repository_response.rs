pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesLinkRepositoryResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesLinkRepositoryResponse {
    pub fn builder() -> MutationServicesLinkRepositoryResponseBuilder {
        <MutationServicesLinkRepositoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesLinkRepositoryResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesLinkRepositoryResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesLinkRepositoryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesLinkRepositoryResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesLinkRepositoryResponse, BuildError> {
        Ok(MutationServicesLinkRepositoryResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

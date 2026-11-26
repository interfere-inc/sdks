pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesArchiveResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesArchiveResponse {
    pub fn builder() -> MutationServicesArchiveResponseBuilder {
        <MutationServicesArchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesArchiveResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesArchiveResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesArchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesArchiveResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesArchiveResponse, BuildError> {
        Ok(MutationServicesArchiveResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesUnarchiveResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesUnarchiveResponse {
    pub fn builder() -> MutationServicesUnarchiveResponseBuilder {
        <MutationServicesUnarchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesUnarchiveResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesUnarchiveResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesUnarchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesUnarchiveResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesUnarchiveResponse, BuildError> {
        Ok(MutationServicesUnarchiveResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

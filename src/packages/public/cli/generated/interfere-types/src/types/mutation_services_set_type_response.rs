pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationServicesSetTypeResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationServicesSetTypeResponse {
    pub fn builder() -> MutationServicesSetTypeResponseBuilder {
        <MutationServicesSetTypeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationServicesSetTypeResponseBuilder {
    success: Option<bool>,
}

impl MutationServicesSetTypeResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationServicesSetTypeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationServicesSetTypeResponseBuilder::success)
    pub fn build(self) -> Result<MutationServicesSetTypeResponse, BuildError> {
        Ok(MutationServicesSetTypeResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationEnvironmentsSetProductionResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationEnvironmentsSetProductionResponse {
    pub fn builder() -> MutationEnvironmentsSetProductionResponseBuilder {
        <MutationEnvironmentsSetProductionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationEnvironmentsSetProductionResponseBuilder {
    success: Option<bool>,
}

impl MutationEnvironmentsSetProductionResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationEnvironmentsSetProductionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationEnvironmentsSetProductionResponseBuilder::success)
    pub fn build(self) -> Result<MutationEnvironmentsSetProductionResponse, BuildError> {
        Ok(MutationEnvironmentsSetProductionResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

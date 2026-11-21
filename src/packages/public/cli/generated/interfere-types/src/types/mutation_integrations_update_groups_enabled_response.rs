pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationIntegrationsUpdateGroupsEnabledResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationIntegrationsUpdateGroupsEnabledResponse {
    pub fn builder() -> MutationIntegrationsUpdateGroupsEnabledResponseBuilder {
        <MutationIntegrationsUpdateGroupsEnabledResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationIntegrationsUpdateGroupsEnabledResponseBuilder {
    success: Option<bool>,
}

impl MutationIntegrationsUpdateGroupsEnabledResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationIntegrationsUpdateGroupsEnabledResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationIntegrationsUpdateGroupsEnabledResponseBuilder::success)
    pub fn build(self) -> Result<MutationIntegrationsUpdateGroupsEnabledResponse, BuildError> {
        Ok(MutationIntegrationsUpdateGroupsEnabledResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}

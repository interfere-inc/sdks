pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateGroupsEnabledIntegrationsRequest {
    #[serde(default)]
    pub args: UpdateGroupsEnabledIntegrationsRequestArgs,
}

impl UpdateGroupsEnabledIntegrationsRequest {
    pub fn builder() -> UpdateGroupsEnabledIntegrationsRequestBuilder {
        <UpdateGroupsEnabledIntegrationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateGroupsEnabledIntegrationsRequestBuilder {
    args: Option<UpdateGroupsEnabledIntegrationsRequestArgs>,
}

impl UpdateGroupsEnabledIntegrationsRequestBuilder {
    pub fn args(mut self, value: UpdateGroupsEnabledIntegrationsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateGroupsEnabledIntegrationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateGroupsEnabledIntegrationsRequestBuilder::args)
    pub fn build(self) -> Result<UpdateGroupsEnabledIntegrationsRequest, BuildError> {
        Ok(UpdateGroupsEnabledIntegrationsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


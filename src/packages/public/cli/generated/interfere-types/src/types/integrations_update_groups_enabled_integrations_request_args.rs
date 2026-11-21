pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateGroupsEnabledIntegrationsRequestArgs {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(default)]
    pub enabled: bool,
}

impl UpdateGroupsEnabledIntegrationsRequestArgs {
    pub fn builder() -> UpdateGroupsEnabledIntegrationsRequestArgsBuilder {
        <UpdateGroupsEnabledIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateGroupsEnabledIntegrationsRequestArgsBuilder {
    connection_id: Option<String>,
    enabled: Option<bool>,
}

impl UpdateGroupsEnabledIntegrationsRequestArgsBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateGroupsEnabledIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](UpdateGroupsEnabledIntegrationsRequestArgsBuilder::connection_id)
    /// - [`enabled`](UpdateGroupsEnabledIntegrationsRequestArgsBuilder::enabled)
    pub fn build(self) -> Result<UpdateGroupsEnabledIntegrationsRequestArgs, BuildError> {
        Ok(UpdateGroupsEnabledIntegrationsRequestArgs {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}

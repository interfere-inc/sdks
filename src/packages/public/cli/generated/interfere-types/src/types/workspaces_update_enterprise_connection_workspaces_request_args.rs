pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEnterpriseConnectionWorkspacesRequestArgs {
    #[serde(rename = "disableAdditionalIdentifications")]
    #[serde(default)]
    pub disable_additional_identifications: bool,
    #[serde(default)]
    pub id: String,
}

impl UpdateEnterpriseConnectionWorkspacesRequestArgs {
    pub fn builder() -> UpdateEnterpriseConnectionWorkspacesRequestArgsBuilder {
        <UpdateEnterpriseConnectionWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEnterpriseConnectionWorkspacesRequestArgsBuilder {
    disable_additional_identifications: Option<bool>,
    id: Option<String>,
}

impl UpdateEnterpriseConnectionWorkspacesRequestArgsBuilder {
    pub fn disable_additional_identifications(mut self, value: bool) -> Self {
        self.disable_additional_identifications = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEnterpriseConnectionWorkspacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`disable_additional_identifications`](UpdateEnterpriseConnectionWorkspacesRequestArgsBuilder::disable_additional_identifications)
    /// - [`id`](UpdateEnterpriseConnectionWorkspacesRequestArgsBuilder::id)
    pub fn build(self) -> Result<UpdateEnterpriseConnectionWorkspacesRequestArgs, BuildError> {
        Ok(UpdateEnterpriseConnectionWorkspacesRequestArgs {
            disable_additional_identifications: self.disable_additional_identifications.ok_or_else(|| BuildError::missing_field("disable_additional_identifications"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

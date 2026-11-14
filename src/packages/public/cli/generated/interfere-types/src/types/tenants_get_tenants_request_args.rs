pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTenantsRequestArgs {
    #[serde(rename = "endUserOrgId")]
    #[serde(default)]
    pub end_user_org_id: String,
}

impl GetTenantsRequestArgs {
    pub fn builder() -> GetTenantsRequestArgsBuilder {
        <GetTenantsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTenantsRequestArgsBuilder {
    end_user_org_id: Option<String>,
}

impl GetTenantsRequestArgsBuilder {
    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTenantsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_user_org_id`](GetTenantsRequestArgsBuilder::end_user_org_id)
    pub fn build(self) -> Result<GetTenantsRequestArgs, BuildError> {
        Ok(GetTenantsRequestArgs {
            end_user_org_id: self.end_user_org_id.ok_or_else(|| BuildError::missing_field("end_user_org_id"))?,
        })
    }
}

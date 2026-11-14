pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateContextsTenantsRequestTenantsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "endUserOrgId")]
    #[serde(default)]
    pub end_user_org_id: String,
}

impl UpdateContextsTenantsRequestTenantsItem {
    pub fn builder() -> UpdateContextsTenantsRequestTenantsItemBuilder {
        <UpdateContextsTenantsRequestTenantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateContextsTenantsRequestTenantsItemBuilder {
    tier: Option<String>,
    end_user_org_id: Option<String>,
}

impl UpdateContextsTenantsRequestTenantsItemBuilder {
    pub fn tier(mut self, value: impl Into<String>) -> Self {
        self.tier = Some(value.into());
        self
    }

    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateContextsTenantsRequestTenantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_user_org_id`](UpdateContextsTenantsRequestTenantsItemBuilder::end_user_org_id)
    pub fn build(self) -> Result<UpdateContextsTenantsRequestTenantsItem, BuildError> {
        Ok(UpdateContextsTenantsRequestTenantsItem {
            tier: self.tier,
            end_user_org_id: self.end_user_org_id.ok_or_else(|| BuildError::missing_field("end_user_org_id"))?,
        })
    }
}

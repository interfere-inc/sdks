pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeTenantsResponseAbsorb {
    #[serde(rename = "endUserOrgId")]
    #[serde(default)]
    pub end_user_org_id: String,
}

impl MergeTenantsResponseAbsorb {
    pub fn builder() -> MergeTenantsResponseAbsorbBuilder {
        <MergeTenantsResponseAbsorbBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeTenantsResponseAbsorbBuilder {
    end_user_org_id: Option<String>,
}

impl MergeTenantsResponseAbsorbBuilder {
    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MergeTenantsResponseAbsorb`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_user_org_id`](MergeTenantsResponseAbsorbBuilder::end_user_org_id)
    pub fn build(self) -> Result<MergeTenantsResponseAbsorb, BuildError> {
        Ok(MergeTenantsResponseAbsorb {
            end_user_org_id: self.end_user_org_id.ok_or_else(|| BuildError::missing_field("end_user_org_id"))?,
        })
    }
}

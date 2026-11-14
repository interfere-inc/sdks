pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeTenantsResponseKeep {
    #[serde(rename = "endUserOrgId")]
    #[serde(default)]
    pub end_user_org_id: String,
}

impl MergeTenantsResponseKeep {
    pub fn builder() -> MergeTenantsResponseKeepBuilder {
        <MergeTenantsResponseKeepBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeTenantsResponseKeepBuilder {
    end_user_org_id: Option<String>,
}

impl MergeTenantsResponseKeepBuilder {
    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MergeTenantsResponseKeep`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_user_org_id`](MergeTenantsResponseKeepBuilder::end_user_org_id)
    pub fn build(self) -> Result<MergeTenantsResponseKeep, BuildError> {
        Ok(MergeTenantsResponseKeep {
            end_user_org_id: self.end_user_org_id.ok_or_else(|| BuildError::missing_field("end_user_org_id"))?,
        })
    }
}

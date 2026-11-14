pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SeenUserTenantsRequestArgs {
    pub edge: SeenUserTenantsRequestArgsEdge,
    #[serde(rename = "endUserOrgId")]
    #[serde(default)]
    pub end_user_org_id: String,
}

impl SeenUserTenantsRequestArgs {
    pub fn builder() -> SeenUserTenantsRequestArgsBuilder {
        <SeenUserTenantsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SeenUserTenantsRequestArgsBuilder {
    edge: Option<SeenUserTenantsRequestArgsEdge>,
    end_user_org_id: Option<String>,
}

impl SeenUserTenantsRequestArgsBuilder {
    pub fn edge(mut self, value: SeenUserTenantsRequestArgsEdge) -> Self {
        self.edge = Some(value);
        self
    }

    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SeenUserTenantsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`edge`](SeenUserTenantsRequestArgsBuilder::edge)
    /// - [`end_user_org_id`](SeenUserTenantsRequestArgsBuilder::end_user_org_id)
    pub fn build(self) -> Result<SeenUserTenantsRequestArgs, BuildError> {
        Ok(SeenUserTenantsRequestArgs {
            edge: self.edge.ok_or_else(|| BuildError::missing_field("edge"))?,
            end_user_org_id: self.end_user_org_id.ok_or_else(|| BuildError::missing_field("end_user_org_id"))?,
        })
    }
}

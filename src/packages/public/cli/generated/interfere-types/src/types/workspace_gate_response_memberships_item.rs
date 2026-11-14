pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGateResponseMembershipsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub organization: WorkspaceSummary,
}

impl WorkspaceGateResponseMembershipsItem {
    pub fn builder() -> WorkspaceGateResponseMembershipsItemBuilder {
        <WorkspaceGateResponseMembershipsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGateResponseMembershipsItemBuilder {
    id: Option<String>,
    organization: Option<WorkspaceSummary>,
}

impl WorkspaceGateResponseMembershipsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization(mut self, value: WorkspaceSummary) -> Self {
        self.organization = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGateResponseMembershipsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkspaceGateResponseMembershipsItemBuilder::id)
    /// - [`organization`](WorkspaceGateResponseMembershipsItemBuilder::organization)
    pub fn build(self) -> Result<WorkspaceGateResponseMembershipsItem, BuildError> {
        Ok(WorkspaceGateResponseMembershipsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization: self.organization.ok_or_else(|| BuildError::missing_field("organization"))?,
        })
    }
}

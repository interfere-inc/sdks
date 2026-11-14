pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGateResponseInvitationsItem {
    #[serde(rename = "clerkInvitationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clerk_invitation_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub organization: WorkspaceSummary,
}

impl WorkspaceGateResponseInvitationsItem {
    pub fn builder() -> WorkspaceGateResponseInvitationsItemBuilder {
        <WorkspaceGateResponseInvitationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGateResponseInvitationsItemBuilder {
    clerk_invitation_id: Option<String>,
    id: Option<String>,
    organization: Option<WorkspaceSummary>,
}

impl WorkspaceGateResponseInvitationsItemBuilder {
    pub fn clerk_invitation_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_invitation_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization(mut self, value: WorkspaceSummary) -> Self {
        self.organization = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGateResponseInvitationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkspaceGateResponseInvitationsItemBuilder::id)
    /// - [`organization`](WorkspaceGateResponseInvitationsItemBuilder::organization)
    pub fn build(self) -> Result<WorkspaceGateResponseInvitationsItem, BuildError> {
        Ok(WorkspaceGateResponseInvitationsItem {
            clerk_invitation_id: self.clerk_invitation_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization: self.organization.ok_or_else(|| BuildError::missing_field("organization"))?,
        })
    }
}

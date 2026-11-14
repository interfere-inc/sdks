pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGateResponse {
    #[serde(rename = "canCreateWorkspace")]
    #[serde(default)]
    pub can_create_workspace: bool,
    #[serde(default)]
    pub invitations: Vec<WorkspaceGateResponseInvitationsItem>,
    #[serde(default)]
    pub memberships: Vec<WorkspaceGateResponseMembershipsItem>,
    #[serde(default)]
    pub suggestions: Vec<WorkspaceGateResponseSuggestionsItem>,
}

impl WorkspaceGateResponse {
    pub fn builder() -> WorkspaceGateResponseBuilder {
        <WorkspaceGateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGateResponseBuilder {
    can_create_workspace: Option<bool>,
    invitations: Option<Vec<WorkspaceGateResponseInvitationsItem>>,
    memberships: Option<Vec<WorkspaceGateResponseMembershipsItem>>,
    suggestions: Option<Vec<WorkspaceGateResponseSuggestionsItem>>,
}

impl WorkspaceGateResponseBuilder {
    pub fn can_create_workspace(mut self, value: bool) -> Self {
        self.can_create_workspace = Some(value);
        self
    }

    pub fn invitations(mut self, value: Vec<WorkspaceGateResponseInvitationsItem>) -> Self {
        self.invitations = Some(value);
        self
    }

    pub fn memberships(mut self, value: Vec<WorkspaceGateResponseMembershipsItem>) -> Self {
        self.memberships = Some(value);
        self
    }

    pub fn suggestions(mut self, value: Vec<WorkspaceGateResponseSuggestionsItem>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`can_create_workspace`](WorkspaceGateResponseBuilder::can_create_workspace)
    /// - [`invitations`](WorkspaceGateResponseBuilder::invitations)
    /// - [`memberships`](WorkspaceGateResponseBuilder::memberships)
    /// - [`suggestions`](WorkspaceGateResponseBuilder::suggestions)
    pub fn build(self) -> Result<WorkspaceGateResponse, BuildError> {
        Ok(WorkspaceGateResponse {
            can_create_workspace: self.can_create_workspace.ok_or_else(|| BuildError::missing_field("can_create_workspace"))?,
            invitations: self.invitations.ok_or_else(|| BuildError::missing_field("invitations"))?,
            memberships: self.memberships.ok_or_else(|| BuildError::missing_field("memberships"))?,
            suggestions: self.suggestions.ok_or_else(|| BuildError::missing_field("suggestions"))?,
        })
    }
}

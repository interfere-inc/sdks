pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGateResponseSuggestionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub organization: WorkspaceSummary,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl WorkspaceGateResponseSuggestionsItem {
    pub fn builder() -> WorkspaceGateResponseSuggestionsItemBuilder {
        <WorkspaceGateResponseSuggestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGateResponseSuggestionsItemBuilder {
    id: Option<String>,
    organization: Option<WorkspaceSummary>,
    request_id: Option<String>,
}

impl WorkspaceGateResponseSuggestionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization(mut self, value: WorkspaceSummary) -> Self {
        self.organization = Some(value);
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGateResponseSuggestionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkspaceGateResponseSuggestionsItemBuilder::id)
    /// - [`organization`](WorkspaceGateResponseSuggestionsItemBuilder::organization)
    pub fn build(self) -> Result<WorkspaceGateResponseSuggestionsItem, BuildError> {
        Ok(WorkspaceGateResponseSuggestionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization: self.organization.ok_or_else(|| BuildError::missing_field("organization"))?,
            request_id: self.request_id,
        })
    }
}

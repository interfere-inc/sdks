pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers {
    #[serde(rename = "affectedUsers")]
    #[serde(default)]
    pub affected_users: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersKind,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersBuilder {
    affected_users: Option<i64>,
    environment: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersKind>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersBuilder {
    pub fn affected_users(mut self, value: i64) -> Self {
        self.affected_users = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers`].
    /// This method will fail if any of the following fields are not set:
    /// - [`affected_users`](ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersBuilder::affected_users)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsersBuilder::kind)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemAffectedUsers {
            affected_users: self.affected_users.ok_or_else(|| BuildError::missing_field("affected_users"))?,
            environment: self.environment,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}

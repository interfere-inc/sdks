pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub confirmed: String,
    #[serde(rename = "affectedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_users: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsersBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsersBuilder {
    version: Option<String>,
    confirmed: Option<String>,
    affected_users: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsersBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn confirmed(mut self, value: impl Into<String>) -> Self {
        self.confirmed = Some(value.into());
        self
    }

    pub fn affected_users(mut self, value: i64) -> Self {
        self.affected_users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsersBuilder::version)
    /// - [`confirmed`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsersBuilder::confirmed)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenAffectedUsers {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            confirmed: self.confirmed.ok_or_else(|| BuildError::missing_field("confirmed"))?,
            affected_users: self.affected_users,
        })
    }
}

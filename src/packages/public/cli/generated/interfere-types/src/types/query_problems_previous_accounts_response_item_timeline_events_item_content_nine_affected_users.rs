pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub confirmed: String,
    #[serde(rename = "affectedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_users: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsersBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsersBuilder {
    version: Option<String>,
    confirmed: Option<String>,
    affected_users: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsersBuilder {
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsersBuilder::version)
    /// - [`confirmed`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsersBuilder::confirmed)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNineAffectedUsers {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            confirmed: self.confirmed.ok_or_else(|| BuildError::missing_field("confirmed"))?,
            affected_users: self.affected_users,
        })
    }
}

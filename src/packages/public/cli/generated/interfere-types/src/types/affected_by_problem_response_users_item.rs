pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffectedByProblemResponseUsersItem {
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<i64>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "sessionCount")]
    #[serde(default)]
    pub session_count: i64,
    #[serde(rename = "sessionCountCoverage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_count_coverage: Option<AffectedByProblemResponseUsersItemSessionCountCoverage>,
    #[serde(rename = "lastEncounteredDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_encountered_day: Option<String>,
}

impl AffectedByProblemResponseUsersItem {
    pub fn builder() -> AffectedByProblemResponseUsersItemBuilder {
        <AffectedByProblemResponseUsersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffectedByProblemResponseUsersItemBuilder {
    organization_id: Option<String>,
    identifier: Option<String>,
    email: Option<String>,
    name: Option<String>,
    avatar: Option<String>,
    country: Option<String>,
    first_seen_at: Option<i64>,
    last_seen_at: Option<i64>,
    id: Option<String>,
    session_count: Option<i64>,
    session_count_coverage: Option<AffectedByProblemResponseUsersItemSessionCountCoverage>,
    last_encountered_day: Option<String>,
}

impl AffectedByProblemResponseUsersItemBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn avatar(mut self, value: impl Into<String>) -> Self {
        self.avatar = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn first_seen_at(mut self, value: i64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn last_seen_at(mut self, value: i64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn session_count(mut self, value: i64) -> Self {
        self.session_count = Some(value);
        self
    }

    pub fn session_count_coverage(mut self, value: AffectedByProblemResponseUsersItemSessionCountCoverage) -> Self {
        self.session_count_coverage = Some(value);
        self
    }

    pub fn last_encountered_day(mut self, value: impl Into<String>) -> Self {
        self.last_encountered_day = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AffectedByProblemResponseUsersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](AffectedByProblemResponseUsersItemBuilder::organization_id)
    /// - [`id`](AffectedByProblemResponseUsersItemBuilder::id)
    /// - [`session_count`](AffectedByProblemResponseUsersItemBuilder::session_count)
    pub fn build(self) -> Result<AffectedByProblemResponseUsersItem, BuildError> {
        Ok(AffectedByProblemResponseUsersItem {
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            identifier: self.identifier,
            email: self.email,
            name: self.name,
            avatar: self.avatar,
            country: self.country,
            first_seen_at: self.first_seen_at,
            last_seen_at: self.last_seen_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            session_count: self.session_count.ok_or_else(|| BuildError::missing_field("session_count"))?,
            session_count_coverage: self.session_count_coverage,
            last_encountered_day: self.last_encountered_day,
        })
    }
}

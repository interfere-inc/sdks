pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConsumerLookupResponse {
    #[serde(rename = "activeProblems")]
    #[serde(default)]
    pub active_problems: Vec<ConsumerLookupResponseActiveProblemsItem>,
    #[serde(rename = "anonymousNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "endUserOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_org_id: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "firstSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<i64>,
    #[serde(rename = "mergedIntoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    /// Auth provider that surfaced this identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ConsumerLookupResponseSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<serde_json::Value>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: i64,
    /// The most recent problem this user hit, or null when none.
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<ConsumerLookupResponseLastError>,
    /// Up to 10 sessions from the last 30 days, newest first.
    #[serde(rename = "recentSessions")]
    #[serde(default)]
    pub recent_sessions: Vec<SessionSummary>,
}

impl ConsumerLookupResponse {
    pub fn builder() -> ConsumerLookupResponseBuilder {
        <ConsumerLookupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerLookupResponseBuilder {
    active_problems: Option<Vec<ConsumerLookupResponseActiveProblemsItem>>,
    anonymous_number: Option<f64>,
    avatar: Option<String>,
    country: Option<String>,
    created_at: Option<i64>,
    deleted_at: Option<i64>,
    email: Option<String>,
    end_user_org_id: Option<String>,
    first_seen_at: Option<i64>,
    id: Option<String>,
    identifier: Option<String>,
    last_seen_at: Option<i64>,
    merged_into_id: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
    source: Option<ConsumerLookupResponseSource>,
    traits: Option<serde_json::Value>,
    updated_at: Option<i64>,
    last_error: Option<ConsumerLookupResponseLastError>,
    recent_sessions: Option<Vec<SessionSummary>>,
}

impl ConsumerLookupResponseBuilder {
    pub fn active_problems(mut self, value: Vec<ConsumerLookupResponseActiveProblemsItem>) -> Self {
        self.active_problems = Some(value);
        self
    }

    pub fn anonymous_number(mut self, value: f64) -> Self {
        self.anonymous_number = Some(value);
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

    pub fn created_at(mut self, value: i64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: i64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    pub fn first_seen_at(mut self, value: i64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: i64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn merged_into_id(mut self, value: impl Into<String>) -> Self {
        self.merged_into_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: ConsumerLookupResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn traits(mut self, value: serde_json::Value) -> Self {
        self.traits = Some(value);
        self
    }

    pub fn updated_at(mut self, value: i64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn last_error(mut self, value: ConsumerLookupResponseLastError) -> Self {
        self.last_error = Some(value);
        self
    }

    pub fn recent_sessions(mut self, value: Vec<SessionSummary>) -> Self {
        self.recent_sessions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConsumerLookupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_problems`](ConsumerLookupResponseBuilder::active_problems)
    /// - [`created_at`](ConsumerLookupResponseBuilder::created_at)
    /// - [`id`](ConsumerLookupResponseBuilder::id)
    /// - [`organization_id`](ConsumerLookupResponseBuilder::organization_id)
    /// - [`updated_at`](ConsumerLookupResponseBuilder::updated_at)
    /// - [`recent_sessions`](ConsumerLookupResponseBuilder::recent_sessions)
    pub fn build(self) -> Result<ConsumerLookupResponse, BuildError> {
        Ok(ConsumerLookupResponse {
            active_problems: self.active_problems.ok_or_else(|| BuildError::missing_field("active_problems"))?,
            anonymous_number: self.anonymous_number,
            avatar: self.avatar,
            country: self.country,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            email: self.email,
            end_user_org_id: self.end_user_org_id,
            first_seen_at: self.first_seen_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identifier: self.identifier,
            last_seen_at: self.last_seen_at,
            merged_into_id: self.merged_into_id,
            name: self.name,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            source: self.source,
            traits: self.traits,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            last_error: self.last_error,
            recent_sessions: self.recent_sessions.ok_or_else(|| BuildError::missing_field("recent_sessions"))?,
        })
    }
}

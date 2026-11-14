pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConsumerDetailsResponse {
    #[serde(rename = "activeProblems")]
    #[serde(default)]
    pub active_problems: Vec<ConsumerDetailsResponseActiveProblemsItem>,
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
    pub source: Option<ConsumerDetailsResponseSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<serde_json::Value>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: i64,
}

impl ConsumerDetailsResponse {
    pub fn builder() -> ConsumerDetailsResponseBuilder {
        <ConsumerDetailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConsumerDetailsResponseBuilder {
    active_problems: Option<Vec<ConsumerDetailsResponseActiveProblemsItem>>,
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
    source: Option<ConsumerDetailsResponseSource>,
    traits: Option<serde_json::Value>,
    updated_at: Option<i64>,
}

impl ConsumerDetailsResponseBuilder {
    pub fn active_problems(mut self, value: Vec<ConsumerDetailsResponseActiveProblemsItem>) -> Self {
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

    pub fn source(mut self, value: ConsumerDetailsResponseSource) -> Self {
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

    /// Consumes the builder and constructs a [`ConsumerDetailsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_problems`](ConsumerDetailsResponseBuilder::active_problems)
    /// - [`created_at`](ConsumerDetailsResponseBuilder::created_at)
    /// - [`id`](ConsumerDetailsResponseBuilder::id)
    /// - [`organization_id`](ConsumerDetailsResponseBuilder::organization_id)
    /// - [`updated_at`](ConsumerDetailsResponseBuilder::updated_at)
    pub fn build(self) -> Result<ConsumerDetailsResponse, BuildError> {
        Ok(ConsumerDetailsResponse {
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
        })
    }
}

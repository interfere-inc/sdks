pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryFindingsListByProblemResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(rename = "errorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    #[serde(rename = "errorSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_source: Option<String>,
    #[serde(rename = "firstSeenAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub first_seen_at: f64,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last_seen_at: f64,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "problemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_id: Option<String>,
    #[serde(rename = "releaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_id: Option<String>,
    #[serde(rename = "retryCount")]
    #[serde(default)]
    pub retry_count: i64,
    pub status: QueryFindingsListByProblemResponseItemStatus,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
    #[serde(rename = "topInAppFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_in_app_file: Option<String>,
    #[serde(rename = "topInAppFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_in_app_function: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<QueryFindingsListByProblemResponseItemSurface>,
}

impl QueryFindingsListByProblemResponseItem {
    pub fn builder() -> QueryFindingsListByProblemResponseItemBuilder {
        <QueryFindingsListByProblemResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryFindingsListByProblemResponseItemBuilder {
    environment: Option<String>,
    error_name: Option<String>,
    error_source: Option<String>,
    first_seen_at: Option<f64>,
    id: Option<String>,
    last_seen_at: Option<f64>,
    organization_id: Option<String>,
    problem_id: Option<String>,
    release_id: Option<String>,
    retry_count: Option<i64>,
    status: Option<QueryFindingsListByProblemResponseItemStatus>,
    surface_id: Option<String>,
    top_in_app_file: Option<String>,
    top_in_app_function: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    surface: Option<QueryFindingsListByProblemResponseItemSurface>,
}

impl QueryFindingsListByProblemResponseItemBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn error_name(mut self, value: impl Into<String>) -> Self {
        self.error_name = Some(value.into());
        self
    }

    pub fn error_source(mut self, value: impl Into<String>) -> Self {
        self.error_source = Some(value.into());
        self
    }

    pub fn first_seen_at(mut self, value: f64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: f64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn release_id(mut self, value: impl Into<String>) -> Self {
        self.release_id = Some(value.into());
        self
    }

    pub fn retry_count(mut self, value: i64) -> Self {
        self.retry_count = Some(value);
        self
    }

    pub fn status(mut self, value: QueryFindingsListByProblemResponseItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn top_in_app_file(mut self, value: impl Into<String>) -> Self {
        self.top_in_app_file = Some(value.into());
        self
    }

    pub fn top_in_app_function(mut self, value: impl Into<String>) -> Self {
        self.top_in_app_function = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn surface(mut self, value: QueryFindingsListByProblemResponseItemSurface) -> Self {
        self.surface = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryFindingsListByProblemResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_seen_at`](QueryFindingsListByProblemResponseItemBuilder::first_seen_at)
    /// - [`id`](QueryFindingsListByProblemResponseItemBuilder::id)
    /// - [`last_seen_at`](QueryFindingsListByProblemResponseItemBuilder::last_seen_at)
    /// - [`organization_id`](QueryFindingsListByProblemResponseItemBuilder::organization_id)
    /// - [`retry_count`](QueryFindingsListByProblemResponseItemBuilder::retry_count)
    /// - [`status`](QueryFindingsListByProblemResponseItemBuilder::status)
    /// - [`created_at`](QueryFindingsListByProblemResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryFindingsListByProblemResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryFindingsListByProblemResponseItem, BuildError> {
        Ok(QueryFindingsListByProblemResponseItem {
            environment: self.environment,
            error_name: self.error_name,
            error_source: self.error_source,
            first_seen_at: self.first_seen_at.ok_or_else(|| BuildError::missing_field("first_seen_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_seen_at: self.last_seen_at.ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            problem_id: self.problem_id,
            release_id: self.release_id,
            retry_count: self.retry_count.ok_or_else(|| BuildError::missing_field("retry_count"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            surface_id: self.surface_id,
            top_in_app_file: self.top_in_app_file,
            top_in_app_function: self.top_in_app_function,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            surface: self.surface,
        })
    }
}

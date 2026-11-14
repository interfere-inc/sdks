pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryConsumerDevicesByEndUserResponseItem {
    #[serde(rename = "anonymousNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_number: Option<i64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
    #[serde(rename = "endUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_id: Option<String>,
    #[serde(rename = "firstSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub first_seen_at: Option<f64>,
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_seen_at: Option<f64>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "surfaceId")]
    #[serde(default)]
    pub surface_id: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<QueryConsumerDevicesByEndUserResponseItemSurface>,
}

impl QueryConsumerDevicesByEndUserResponseItem {
    pub fn builder() -> QueryConsumerDevicesByEndUserResponseItemBuilder {
        <QueryConsumerDevicesByEndUserResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumerDevicesByEndUserResponseItemBuilder {
    anonymous_number: Option<i64>,
    created_at: Option<f64>,
    device_id: Option<String>,
    end_user_id: Option<String>,
    first_seen_at: Option<f64>,
    last_seen_at: Option<f64>,
    organization_id: Option<String>,
    surface_id: Option<String>,
    updated_at: Option<f64>,
    surface: Option<QueryConsumerDevicesByEndUserResponseItemSurface>,
}

impl QueryConsumerDevicesByEndUserResponseItemBuilder {
    pub fn anonymous_number(mut self, value: i64) -> Self {
        self.anonymous_number = Some(value);
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn end_user_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_id = Some(value.into());
        self
    }

    pub fn first_seen_at(mut self, value: f64) -> Self {
        self.first_seen_at = Some(value);
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

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn surface(mut self, value: QueryConsumerDevicesByEndUserResponseItemSurface) -> Self {
        self.surface = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumerDevicesByEndUserResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QueryConsumerDevicesByEndUserResponseItemBuilder::created_at)
    /// - [`device_id`](QueryConsumerDevicesByEndUserResponseItemBuilder::device_id)
    /// - [`organization_id`](QueryConsumerDevicesByEndUserResponseItemBuilder::organization_id)
    /// - [`surface_id`](QueryConsumerDevicesByEndUserResponseItemBuilder::surface_id)
    /// - [`updated_at`](QueryConsumerDevicesByEndUserResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryConsumerDevicesByEndUserResponseItem, BuildError> {
        Ok(QueryConsumerDevicesByEndUserResponseItem {
            anonymous_number: self.anonymous_number,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            device_id: self.device_id.ok_or_else(|| BuildError::missing_field("device_id"))?,
            end_user_id: self.end_user_id,
            first_seen_at: self.first_seen_at,
            last_seen_at: self.last_seen_at,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            surface_id: self.surface_id.ok_or_else(|| BuildError::missing_field("surface_id"))?,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            surface: self.surface,
        })
    }
}

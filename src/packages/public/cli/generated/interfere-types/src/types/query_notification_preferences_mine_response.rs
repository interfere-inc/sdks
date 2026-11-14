pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryNotificationPreferencesMineResponse {
    #[serde(default)]
    pub categories: HashMap<String, QueryNotificationPreferencesMineResponseCategoriesValue>,
    #[serde(rename = "channelTypes")]
    #[serde(default)]
    pub channel_types: HashMap<String, bool>,
    #[serde(rename = "commercialSubscribed")]
    #[serde(default)]
    pub commercial_subscribed: bool,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub workflows: HashMap<String, QueryNotificationPreferencesMineResponseWorkflowsValue>,
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
}

impl QueryNotificationPreferencesMineResponse {
    pub fn builder() -> QueryNotificationPreferencesMineResponseBuilder {
        <QueryNotificationPreferencesMineResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryNotificationPreferencesMineResponseBuilder {
    categories: Option<HashMap<String, QueryNotificationPreferencesMineResponseCategoriesValue>>,
    channel_types: Option<HashMap<String, bool>>,
    commercial_subscribed: Option<bool>,
    id: Option<String>,
    user_id: Option<String>,
    workflows: Option<HashMap<String, QueryNotificationPreferencesMineResponseWorkflowsValue>>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryNotificationPreferencesMineResponseBuilder {
    pub fn categories(mut self, value: HashMap<String, QueryNotificationPreferencesMineResponseCategoriesValue>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn channel_types(mut self, value: HashMap<String, bool>) -> Self {
        self.channel_types = Some(value);
        self
    }

    pub fn commercial_subscribed(mut self, value: bool) -> Self {
        self.commercial_subscribed = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn workflows(mut self, value: HashMap<String, QueryNotificationPreferencesMineResponseWorkflowsValue>) -> Self {
        self.workflows = Some(value);
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

    /// Consumes the builder and constructs a [`QueryNotificationPreferencesMineResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`categories`](QueryNotificationPreferencesMineResponseBuilder::categories)
    /// - [`channel_types`](QueryNotificationPreferencesMineResponseBuilder::channel_types)
    /// - [`commercial_subscribed`](QueryNotificationPreferencesMineResponseBuilder::commercial_subscribed)
    /// - [`id`](QueryNotificationPreferencesMineResponseBuilder::id)
    /// - [`user_id`](QueryNotificationPreferencesMineResponseBuilder::user_id)
    /// - [`workflows`](QueryNotificationPreferencesMineResponseBuilder::workflows)
    /// - [`created_at`](QueryNotificationPreferencesMineResponseBuilder::created_at)
    /// - [`updated_at`](QueryNotificationPreferencesMineResponseBuilder::updated_at)
    pub fn build(self) -> Result<QueryNotificationPreferencesMineResponse, BuildError> {
        Ok(QueryNotificationPreferencesMineResponse {
            categories: self.categories.ok_or_else(|| BuildError::missing_field("categories"))?,
            channel_types: self.channel_types.ok_or_else(|| BuildError::missing_field("channel_types"))?,
            commercial_subscribed: self.commercial_subscribed.ok_or_else(|| BuildError::missing_field("commercial_subscribed"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            workflows: self.workflows.ok_or_else(|| BuildError::missing_field("workflows"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

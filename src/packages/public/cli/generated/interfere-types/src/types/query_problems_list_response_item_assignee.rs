pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsListResponseItemAssignee {
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
    #[serde(rename = "clerkUserId")]
    #[serde(default)]
    pub clerk_user_id: String,
    #[serde(rename = "deactivatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deactivated_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(rename = "desktopAppSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub desktop_app_seen_at: Option<f64>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "emailPrimary")]
    #[serde(default)]
    pub email_primary: String,
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default)]
    pub initials: String,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "openLinksInDesktop")]
    #[serde(default)]
    pub open_links_in_desktop: bool,
    pub source: QueryProblemsListResponseItemAssigneeSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl QueryProblemsListResponseItemAssignee {
    pub fn builder() -> QueryProblemsListResponseItemAssigneeBuilder {
        <QueryProblemsListResponseItemAssigneeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsListResponseItemAssigneeBuilder {
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    clerk_user_id: Option<String>,
    deactivated_at: Option<f64>,
    department: Option<String>,
    desktop_app_seen_at: Option<f64>,
    display_name: Option<String>,
    email_primary: Option<String>,
    first_name: Option<String>,
    id: Option<String>,
    image_url: Option<String>,
    initials: Option<String>,
    last_name: Option<String>,
    locale: Option<String>,
    open_links_in_desktop: Option<bool>,
    source: Option<QueryProblemsListResponseItemAssigneeSource>,
    timezone: Option<String>,
    title: Option<String>,
}

impl QueryProblemsListResponseItemAssigneeBuilder {
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

    pub fn clerk_user_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_user_id = Some(value.into());
        self
    }

    pub fn deactivated_at(mut self, value: f64) -> Self {
        self.deactivated_at = Some(value);
        self
    }

    pub fn department(mut self, value: impl Into<String>) -> Self {
        self.department = Some(value.into());
        self
    }

    pub fn desktop_app_seen_at(mut self, value: f64) -> Self {
        self.desktop_app_seen_at = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn email_primary(mut self, value: impl Into<String>) -> Self {
        self.email_primary = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn initials(mut self, value: impl Into<String>) -> Self {
        self.initials = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn open_links_in_desktop(mut self, value: bool) -> Self {
        self.open_links_in_desktop = Some(value);
        self
    }

    pub fn source(mut self, value: QueryProblemsListResponseItemAssigneeSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsListResponseItemAssignee`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QueryProblemsListResponseItemAssigneeBuilder::created_at)
    /// - [`updated_at`](QueryProblemsListResponseItemAssigneeBuilder::updated_at)
    /// - [`clerk_user_id`](QueryProblemsListResponseItemAssigneeBuilder::clerk_user_id)
    /// - [`display_name`](QueryProblemsListResponseItemAssigneeBuilder::display_name)
    /// - [`email_primary`](QueryProblemsListResponseItemAssigneeBuilder::email_primary)
    /// - [`id`](QueryProblemsListResponseItemAssigneeBuilder::id)
    /// - [`initials`](QueryProblemsListResponseItemAssigneeBuilder::initials)
    /// - [`open_links_in_desktop`](QueryProblemsListResponseItemAssigneeBuilder::open_links_in_desktop)
    /// - [`source`](QueryProblemsListResponseItemAssigneeBuilder::source)
    pub fn build(self) -> Result<QueryProblemsListResponseItemAssignee, BuildError> {
        Ok(QueryProblemsListResponseItemAssignee {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            clerk_user_id: self.clerk_user_id.ok_or_else(|| BuildError::missing_field("clerk_user_id"))?,
            deactivated_at: self.deactivated_at,
            department: self.department,
            desktop_app_seen_at: self.desktop_app_seen_at,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            email_primary: self.email_primary.ok_or_else(|| BuildError::missing_field("email_primary"))?,
            first_name: self.first_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_url: self.image_url,
            initials: self.initials.ok_or_else(|| BuildError::missing_field("initials"))?,
            last_name: self.last_name,
            locale: self.locale,
            open_links_in_desktop: self.open_links_in_desktop.ok_or_else(|| BuildError::missing_field("open_links_in_desktop"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            timezone: self.timezone,
            title: self.title,
        })
    }
}

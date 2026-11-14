pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuerySupportMessagesForThreadResponseItem {
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
    pub author: QuerySupportMessagesForThreadResponseItemAuthor,
    #[serde(rename = "authorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_user_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "sentAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub sent_at: f64,
    #[serde(default)]
    pub text: String,
    #[serde(rename = "threadId")]
    #[serde(default)]
    pub thread_id: String,
    #[serde(rename = "authorUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_user: Option<QuerySupportMessagesForThreadResponseItemAuthorUser>,
}

impl QuerySupportMessagesForThreadResponseItem {
    pub fn builder() -> QuerySupportMessagesForThreadResponseItemBuilder {
        <QuerySupportMessagesForThreadResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySupportMessagesForThreadResponseItemBuilder {
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    author: Option<QuerySupportMessagesForThreadResponseItemAuthor>,
    author_user_id: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    sent_at: Option<f64>,
    text: Option<String>,
    thread_id: Option<String>,
    author_user: Option<QuerySupportMessagesForThreadResponseItemAuthorUser>,
}

impl QuerySupportMessagesForThreadResponseItemBuilder {
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

    pub fn author(mut self, value: QuerySupportMessagesForThreadResponseItemAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn author_user_id(mut self, value: impl Into<String>) -> Self {
        self.author_user_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn sent_at(mut self, value: f64) -> Self {
        self.sent_at = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    pub fn author_user(mut self, value: QuerySupportMessagesForThreadResponseItemAuthorUser) -> Self {
        self.author_user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuerySupportMessagesForThreadResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QuerySupportMessagesForThreadResponseItemBuilder::created_at)
    /// - [`updated_at`](QuerySupportMessagesForThreadResponseItemBuilder::updated_at)
    /// - [`author`](QuerySupportMessagesForThreadResponseItemBuilder::author)
    /// - [`id`](QuerySupportMessagesForThreadResponseItemBuilder::id)
    /// - [`organization_id`](QuerySupportMessagesForThreadResponseItemBuilder::organization_id)
    /// - [`sent_at`](QuerySupportMessagesForThreadResponseItemBuilder::sent_at)
    /// - [`text`](QuerySupportMessagesForThreadResponseItemBuilder::text)
    /// - [`thread_id`](QuerySupportMessagesForThreadResponseItemBuilder::thread_id)
    pub fn build(self) -> Result<QuerySupportMessagesForThreadResponseItem, BuildError> {
        Ok(QuerySupportMessagesForThreadResponseItem {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            author: self.author.ok_or_else(|| BuildError::missing_field("author"))?,
            author_user_id: self.author_user_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            sent_at: self.sent_at.ok_or_else(|| BuildError::missing_field("sent_at"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
            author_user: self.author_user,
        })
    }
}

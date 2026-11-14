pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItem {
    #[serde(rename = "authorAgentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_agent_id: Option<String>,
    #[serde(rename = "authorConnectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_connection_id: Option<String>,
    #[serde(rename = "authorType")]
    pub author_type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemAuthorType,
    #[serde(rename = "authorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_user_id: Option<String>,
    #[serde(rename = "bodyText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    pub content: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContent,
    #[serde(rename = "requestContract")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contract: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract>,
    #[serde(rename = "editedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub edited_at: Option<f64>,
    #[serde(rename = "eventTimestamp")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub event_timestamp: f64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub mentions: Vec<String>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub position: i64,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemType,
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
    #[serde(default)]
    pub reactions: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem>,
    #[serde(default)]
    pub references: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItem {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder {
    author_agent_id: Option<String>,
    author_connection_id: Option<String>,
    author_type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemAuthorType>,
    author_user_id: Option<String>,
    body_text: Option<String>,
    content: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContent>,
    request_contract: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract>,
    edited_at: Option<f64>,
    event_timestamp: Option<f64>,
    id: Option<String>,
    mentions: Option<Vec<String>>,
    organization_id: Option<String>,
    parent_id: Option<String>,
    position: Option<i64>,
    problem_id: Option<String>,
    sequence: Option<i64>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemType>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    reactions: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem>>,
    references: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem>>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder {
    pub fn author_agent_id(mut self, value: impl Into<String>) -> Self {
        self.author_agent_id = Some(value.into());
        self
    }

    pub fn author_connection_id(mut self, value: impl Into<String>) -> Self {
        self.author_connection_id = Some(value.into());
        self
    }

    pub fn author_type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemAuthorType) -> Self {
        self.author_type = Some(value);
        self
    }

    pub fn author_user_id(mut self, value: impl Into<String>) -> Self {
        self.author_user_id = Some(value.into());
        self
    }

    pub fn body_text(mut self, value: impl Into<String>) -> Self {
        self.body_text = Some(value.into());
        self
    }

    pub fn content(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn request_contract(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContract) -> Self {
        self.request_contract = Some(value);
        self
    }

    pub fn edited_at(mut self, value: f64) -> Self {
        self.edited_at = Some(value);
        self
    }

    pub fn event_timestamp(mut self, value: f64) -> Self {
        self.event_timestamp = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mentions(mut self, value: Vec<String>) -> Self {
        self.mentions = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemType) -> Self {
        self.r#type = Some(value);
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

    pub fn reactions(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem>) -> Self {
        self.reactions = Some(value);
        self
    }

    pub fn references(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem>) -> Self {
        self.references = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`author_type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::author_type)
    /// - [`content`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::content)
    /// - [`event_timestamp`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::event_timestamp)
    /// - [`id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::id)
    /// - [`mentions`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::mentions)
    /// - [`organization_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::organization_id)
    /// - [`position`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::position)
    /// - [`problem_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::problem_id)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::r#type)
    /// - [`created_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::created_at)
    /// - [`updated_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::updated_at)
    /// - [`reactions`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::reactions)
    /// - [`references`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemBuilder::references)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItem, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItem {
            author_agent_id: self.author_agent_id,
            author_connection_id: self.author_connection_id,
            author_type: self.author_type.ok_or_else(|| BuildError::missing_field("author_type"))?,
            author_user_id: self.author_user_id,
            body_text: self.body_text,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            request_contract: self.request_contract,
            edited_at: self.edited_at,
            event_timestamp: self.event_timestamp.ok_or_else(|| BuildError::missing_field("event_timestamp"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            mentions: self.mentions.ok_or_else(|| BuildError::missing_field("mentions"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            parent_id: self.parent_id,
            position: self.position.ok_or_else(|| BuildError::missing_field("position"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            sequence: self.sequence,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            reactions: self.reactions.ok_or_else(|| BuildError::missing_field("reactions"))?,
            references: self.references.ok_or_else(|| BuildError::missing_field("references"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionBatchEventsResponseSessionsValueItem {
    #[serde(rename = "actionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambiguous: Option<String>,
    #[serde(default)]
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default)]
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<SessionBatchEventsResponseSessionsValueItemProblem>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub uuid: String,
    #[serde(rename = "viewId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
}

impl SessionBatchEventsResponseSessionsValueItem {
    pub fn builder() -> SessionBatchEventsResponseSessionsValueItemBuilder {
        <SessionBatchEventsResponseSessionsValueItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionBatchEventsResponseSessionsValueItemBuilder {
    action_id: Option<String>,
    ambiguous: Option<String>,
    event: Option<String>,
    fingerprint: Option<String>,
    payload: Option<String>,
    problem: Option<SessionBatchEventsResponseSessionsValueItemProblem>,
    timestamp: Option<i64>,
    uuid: Option<String>,
    view_id: Option<String>,
}

impl SessionBatchEventsResponseSessionsValueItemBuilder {
    pub fn action_id(mut self, value: impl Into<String>) -> Self {
        self.action_id = Some(value.into());
        self
    }

    pub fn ambiguous(mut self, value: impl Into<String>) -> Self {
        self.ambiguous = Some(value.into());
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn payload(mut self, value: impl Into<String>) -> Self {
        self.payload = Some(value.into());
        self
    }

    pub fn problem(mut self, value: SessionBatchEventsResponseSessionsValueItemProblem) -> Self {
        self.problem = Some(value);
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn view_id(mut self, value: impl Into<String>) -> Self {
        self.view_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionBatchEventsResponseSessionsValueItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](SessionBatchEventsResponseSessionsValueItemBuilder::event)
    /// - [`payload`](SessionBatchEventsResponseSessionsValueItemBuilder::payload)
    /// - [`timestamp`](SessionBatchEventsResponseSessionsValueItemBuilder::timestamp)
    /// - [`uuid`](SessionBatchEventsResponseSessionsValueItemBuilder::uuid)
    pub fn build(self) -> Result<SessionBatchEventsResponseSessionsValueItem, BuildError> {
        Ok(SessionBatchEventsResponseSessionsValueItem {
            action_id: self.action_id,
            ambiguous: self.ambiguous,
            event: self.event.ok_or_else(|| BuildError::missing_field("event"))?,
            fingerprint: self.fingerprint,
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
            problem: self.problem,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            view_id: self.view_id,
        })
    }
}

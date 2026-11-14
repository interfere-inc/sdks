pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for sessionBatchEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionBatchEventsQueryRequest {
    #[serde(default)]
    pub date_from: String,
    #[serde(default)]
    pub date_to: String,
    /// Repeat this parameter for multiple app (surface) UUID:session ID identities. CSV values are also accepted, with a total limit of 10 identities after splitting.
    #[serde(default)]
    pub session_record_ids: Vec<Option<String>>,
}

impl SessionBatchEventsQueryRequest {
    pub fn builder() -> SessionBatchEventsQueryRequestBuilder {
        <SessionBatchEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionBatchEventsQueryRequestBuilder {
    date_from: Option<String>,
    date_to: Option<String>,
    session_record_ids: Option<Vec<Option<String>>>,
}

impl SessionBatchEventsQueryRequestBuilder {
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    pub fn session_record_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.session_record_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionBatchEventsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](SessionBatchEventsQueryRequestBuilder::date_from)
    /// - [`date_to`](SessionBatchEventsQueryRequestBuilder::date_to)
    /// - [`session_record_ids`](SessionBatchEventsQueryRequestBuilder::session_record_ids)
    pub fn build(self) -> Result<SessionBatchEventsQueryRequest, BuildError> {
        Ok(SessionBatchEventsQueryRequest {
            date_from: self.date_from.ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self.date_to.ok_or_else(|| BuildError::missing_field("date_to"))?,
            session_record_ids: self.session_record_ids.ok_or_else(|| BuildError::missing_field("session_record_ids"))?,
        })
    }
}


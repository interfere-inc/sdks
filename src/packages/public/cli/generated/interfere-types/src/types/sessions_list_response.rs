pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionsListResponse {
    #[serde(default)]
    pub sessions: Vec<SessionSummary>,
    /// True when more sessions may exist beyond this page, including a page cut short by the read budget.
    #[serde(rename = "hasMore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// Send these fields with the next request to continue after this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<SessionsListResponseCursor>,
}

impl SessionsListResponse {
    pub fn builder() -> SessionsListResponseBuilder {
        <SessionsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsListResponseBuilder {
    sessions: Option<Vec<SessionSummary>>,
    has_more: Option<bool>,
    cursor: Option<SessionsListResponseCursor>,
}

impl SessionsListResponseBuilder {
    pub fn sessions(mut self, value: Vec<SessionSummary>) -> Self {
        self.sessions = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn cursor(mut self, value: SessionsListResponseCursor) -> Self {
        self.cursor = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sessions`](SessionsListResponseBuilder::sessions)
    pub fn build(self) -> Result<SessionsListResponse, BuildError> {
        Ok(SessionsListResponse {
            sessions: self.sessions.ok_or_else(|| BuildError::missing_field("sessions"))?,
            has_more: self.has_more,
            cursor: self.cursor,
        })
    }
}

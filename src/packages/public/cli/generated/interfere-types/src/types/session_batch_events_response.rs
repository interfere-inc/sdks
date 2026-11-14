pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionBatchEventsResponse {
    #[serde(default)]
    pub sessions: HashMap<String, Vec<SessionBatchEventsResponseSessionsValueItem>>,
}

impl SessionBatchEventsResponse {
    pub fn builder() -> SessionBatchEventsResponseBuilder {
        <SessionBatchEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionBatchEventsResponseBuilder {
    sessions: Option<HashMap<String, Vec<SessionBatchEventsResponseSessionsValueItem>>>,
}

impl SessionBatchEventsResponseBuilder {
    pub fn sessions(mut self, value: HashMap<String, Vec<SessionBatchEventsResponseSessionsValueItem>>) -> Self {
        self.sessions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionBatchEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sessions`](SessionBatchEventsResponseBuilder::sessions)
    pub fn build(self) -> Result<SessionBatchEventsResponse, BuildError> {
        Ok(SessionBatchEventsResponse {
            sessions: self.sessions.ok_or_else(|| BuildError::missing_field("sessions"))?,
        })
    }
}

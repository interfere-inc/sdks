pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionsListResponseCursor {
    #[serde(default)]
    pub cursor_last_event_at: String,
    #[serde(default)]
    pub cursor_session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_surface_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_device_id: Option<String>,
}

impl SessionsListResponseCursor {
    pub fn builder() -> SessionsListResponseCursorBuilder {
        <SessionsListResponseCursorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionsListResponseCursorBuilder {
    cursor_last_event_at: Option<String>,
    cursor_session_id: Option<String>,
    cursor_surface_id: Option<String>,
    cursor_device_id: Option<String>,
}

impl SessionsListResponseCursorBuilder {
    pub fn cursor_last_event_at(mut self, value: impl Into<String>) -> Self {
        self.cursor_last_event_at = Some(value.into());
        self
    }

    pub fn cursor_session_id(mut self, value: impl Into<String>) -> Self {
        self.cursor_session_id = Some(value.into());
        self
    }

    pub fn cursor_surface_id(mut self, value: impl Into<String>) -> Self {
        self.cursor_surface_id = Some(value.into());
        self
    }

    pub fn cursor_device_id(mut self, value: impl Into<String>) -> Self {
        self.cursor_device_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionsListResponseCursor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cursor_last_event_at`](SessionsListResponseCursorBuilder::cursor_last_event_at)
    /// - [`cursor_session_id`](SessionsListResponseCursorBuilder::cursor_session_id)
    pub fn build(self) -> Result<SessionsListResponseCursor, BuildError> {
        Ok(SessionsListResponseCursor {
            cursor_last_event_at: self.cursor_last_event_at.ok_or_else(|| BuildError::missing_field("cursor_last_event_at"))?,
            cursor_session_id: self.cursor_session_id.ok_or_else(|| BuildError::missing_field("cursor_session_id"))?,
            cursor_surface_id: self.cursor_surface_id,
            cursor_device_id: self.cursor_device_id,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListQueryRequest {
    /// Last session lastEventAt, converted from epoch milliseconds to UTC YYYY-MM-DD HH:mm:ss. Send together with cursor_session_id, cursor_surface_id and cursor_device_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_last_event_at: Option<String>,
    /// Last session sessionId from the previous page; breaks ties in descending last-event time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_session_id: Option<String>,
    /// Last session surfaceId from the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_surface_id: Option<String>,
    /// Last session deviceId from the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_device_id: Option<String>,
    /// First UTC day to include, formatted YYYY-MM-DD.
    #[serde(default)]
    pub date_from: NaiveDate,
    /// Last UTC day to include, formatted YYYY-MM-DD.
    #[serde(default)]
    pub date_to: NaiveDate,
    /// Maximum sessions per page; defaults to 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
    /// Keeps sessions that visited a URL containing this text, case-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListQueryRequest {
    pub fn builder() -> ListQueryRequestBuilder {
        <ListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListQueryRequestBuilder {
    cursor_last_event_at: Option<String>,
    cursor_session_id: Option<String>,
    cursor_surface_id: Option<String>,
    cursor_device_id: Option<String>,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
    limit: Option<i64>,
    surface_id: Option<String>,
    url: Option<String>,
}

impl ListQueryRequestBuilder {
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

    pub fn date_from(mut self, value: NaiveDate) -> Self {
        self.date_from = Some(value);
        self
    }

    pub fn date_to(mut self, value: NaiveDate) -> Self {
        self.date_to = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](ListQueryRequestBuilder::date_from)
    /// - [`date_to`](ListQueryRequestBuilder::date_to)
    pub fn build(self) -> Result<ListQueryRequest, BuildError> {
        Ok(ListQueryRequest {
            cursor_last_event_at: self.cursor_last_event_at,
            cursor_session_id: self.cursor_session_id,
            cursor_surface_id: self.cursor_surface_id,
            cursor_device_id: self.cursor_device_id,
            date_from: self.date_from.ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self.date_to.ok_or_else(|| BuildError::missing_field("date_to"))?,
            limit: self.limit,
            surface_id: self.surface_id,
            url: self.url,
        })
    }
}


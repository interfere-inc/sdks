pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for sessionReplayIndex
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionReplayIndexQueryRequest {
    #[serde(default)]
    pub date_from: String,
    #[serde(default)]
    pub date_to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<i64>,
}

impl SessionReplayIndexQueryRequest {
    pub fn builder() -> SessionReplayIndexQueryRequestBuilder {
        <SessionReplayIndexQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionReplayIndexQueryRequestBuilder {
    date_from: Option<String>,
    date_to: Option<String>,
    after: Option<i64>,
}

impl SessionReplayIndexQueryRequestBuilder {
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    pub fn after(mut self, value: i64) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionReplayIndexQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](SessionReplayIndexQueryRequestBuilder::date_from)
    /// - [`date_to`](SessionReplayIndexQueryRequestBuilder::date_to)
    pub fn build(self) -> Result<SessionReplayIndexQueryRequest, BuildError> {
        Ok(SessionReplayIndexQueryRequest {
            date_from: self.date_from.ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self.date_to.ok_or_else(|| BuildError::missing_field("date_to"))?,
            after: self.after,
        })
    }
}


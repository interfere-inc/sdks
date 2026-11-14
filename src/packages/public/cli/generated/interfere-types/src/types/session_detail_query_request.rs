pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for sessionDetail
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionDetailQueryRequest {
    #[serde(default)]
    pub date_from: String,
    #[serde(default)]
    pub date_to: String,
}

impl SessionDetailQueryRequest {
    pub fn builder() -> SessionDetailQueryRequestBuilder {
        <SessionDetailQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionDetailQueryRequestBuilder {
    date_from: Option<String>,
    date_to: Option<String>,
}

impl SessionDetailQueryRequestBuilder {
    pub fn date_from(mut self, value: impl Into<String>) -> Self {
        self.date_from = Some(value.into());
        self
    }

    pub fn date_to(mut self, value: impl Into<String>) -> Self {
        self.date_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionDetailQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_from`](SessionDetailQueryRequestBuilder::date_from)
    /// - [`date_to`](SessionDetailQueryRequestBuilder::date_to)
    pub fn build(self) -> Result<SessionDetailQueryRequest, BuildError> {
        Ok(SessionDetailQueryRequest {
            date_from: self.date_from.ok_or_else(|| BuildError::missing_field("date_from"))?,
            date_to: self.date_to.ok_or_else(|| BuildError::missing_field("date_to"))?,
        })
    }
}


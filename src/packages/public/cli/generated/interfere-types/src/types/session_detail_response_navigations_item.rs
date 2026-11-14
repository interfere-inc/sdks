pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionDetailResponseNavigationsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambiguous: Option<String>,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub r#type: String,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub timestamp: i64,
    #[serde(rename = "viewId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_id: Option<String>,
}

impl SessionDetailResponseNavigationsItem {
    pub fn builder() -> SessionDetailResponseNavigationsItemBuilder {
        <SessionDetailResponseNavigationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionDetailResponseNavigationsItemBuilder {
    ambiguous: Option<String>,
    url: Option<String>,
    r#type: Option<String>,
    timestamp: Option<i64>,
    view_id: Option<String>,
}

impl SessionDetailResponseNavigationsItemBuilder {
    pub fn ambiguous(mut self, value: impl Into<String>) -> Self {
        self.ambiguous = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn view_id(mut self, value: impl Into<String>) -> Self {
        self.view_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionDetailResponseNavigationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](SessionDetailResponseNavigationsItemBuilder::url)
    /// - [`r#type`](SessionDetailResponseNavigationsItemBuilder::r#type)
    /// - [`timestamp`](SessionDetailResponseNavigationsItemBuilder::timestamp)
    pub fn build(self) -> Result<SessionDetailResponseNavigationsItem, BuildError> {
        Ok(SessionDetailResponseNavigationsItem {
            ambiguous: self.ambiguous,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            timestamp: self.timestamp.ok_or_else(|| BuildError::missing_field("timestamp"))?,
            view_id: self.view_id,
        })
    }
}

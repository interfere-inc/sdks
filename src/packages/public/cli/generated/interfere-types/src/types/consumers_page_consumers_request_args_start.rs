pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PageConsumersRequestArgsStart {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_seen_at: Option<f64>,
}

impl PageConsumersRequestArgsStart {
    pub fn builder() -> PageConsumersRequestArgsStartBuilder {
        <PageConsumersRequestArgsStartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageConsumersRequestArgsStartBuilder {
    id: Option<String>,
    last_seen_at: Option<f64>,
}

impl PageConsumersRequestArgsStartBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: f64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageConsumersRequestArgsStart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PageConsumersRequestArgsStartBuilder::id)
    pub fn build(self) -> Result<PageConsumersRequestArgsStart, BuildError> {
        Ok(PageConsumersRequestArgsStart {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_seen_at: self.last_seen_at,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAuditRequestArgsStart {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub id: String,
}

impl ListAuditRequestArgsStart {
    pub fn builder() -> ListAuditRequestArgsStartBuilder {
        <ListAuditRequestArgsStartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuditRequestArgsStartBuilder {
    created_at: Option<f64>,
    id: Option<String>,
}

impl ListAuditRequestArgsStartBuilder {
    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAuditRequestArgsStart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListAuditRequestArgsStartBuilder::created_at)
    /// - [`id`](ListAuditRequestArgsStartBuilder::id)
    pub fn build(self) -> Result<ListAuditRequestArgsStart, BuildError> {
        Ok(ListAuditRequestArgsStart {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}

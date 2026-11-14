pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ServiceActivityResponseServicesItem {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    pub last_seen_at: i64,
}

impl ServiceActivityResponseServicesItem {
    pub fn builder() -> ServiceActivityResponseServicesItemBuilder {
        <ServiceActivityResponseServicesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServiceActivityResponseServicesItemBuilder {
    service_id: Option<String>,
    last_seen_at: Option<i64>,
}

impl ServiceActivityResponseServicesItemBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: i64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServiceActivityResponseServicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](ServiceActivityResponseServicesItemBuilder::service_id)
    /// - [`last_seen_at`](ServiceActivityResponseServicesItemBuilder::last_seen_at)
    pub fn build(self) -> Result<ServiceActivityResponseServicesItem, BuildError> {
        Ok(ServiceActivityResponseServicesItem {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            last_seen_at: self.last_seen_at.ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
        })
    }
}

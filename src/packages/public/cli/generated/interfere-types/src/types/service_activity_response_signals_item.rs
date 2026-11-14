pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ServiceActivityResponseSignalsItem {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    pub traces: Vec<i64>,
    #[serde(default)]
    pub logs: Vec<i64>,
    #[serde(default)]
    pub metrics: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ServiceActivityResponseSignalsItemPlatform>,
}

impl ServiceActivityResponseSignalsItem {
    pub fn builder() -> ServiceActivityResponseSignalsItemBuilder {
        <ServiceActivityResponseSignalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServiceActivityResponseSignalsItemBuilder {
    service_id: Option<String>,
    traces: Option<Vec<i64>>,
    logs: Option<Vec<i64>>,
    metrics: Option<Vec<i64>>,
    platform: Option<ServiceActivityResponseSignalsItemPlatform>,
}

impl ServiceActivityResponseSignalsItemBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn traces(mut self, value: Vec<i64>) -> Self {
        self.traces = Some(value);
        self
    }

    pub fn logs(mut self, value: Vec<i64>) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn metrics(mut self, value: Vec<i64>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn platform(mut self, value: ServiceActivityResponseSignalsItemPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServiceActivityResponseSignalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](ServiceActivityResponseSignalsItemBuilder::service_id)
    /// - [`traces`](ServiceActivityResponseSignalsItemBuilder::traces)
    /// - [`logs`](ServiceActivityResponseSignalsItemBuilder::logs)
    /// - [`metrics`](ServiceActivityResponseSignalsItemBuilder::metrics)
    pub fn build(self) -> Result<ServiceActivityResponseSignalsItem, BuildError> {
        Ok(ServiceActivityResponseSignalsItem {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            traces: self.traces.ok_or_else(|| BuildError::missing_field("traces"))?,
            logs: self.logs.ok_or_else(|| BuildError::missing_field("logs"))?,
            metrics: self.metrics.ok_or_else(|| BuildError::missing_field("metrics"))?,
            platform: self.platform,
        })
    }
}

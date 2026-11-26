pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ServiceActivityResponse {
    #[serde(default)]
    pub services: Vec<ServiceActivityResponseServicesItem>,
    #[serde(default)]
    pub buckets: Vec<i64>,
    #[serde(default)]
    pub signals: Vec<ServiceActivityResponseSignalsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdks: Option<Vec<ServiceActivityResponseSdksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retired: Option<Vec<String>>,
}

impl ServiceActivityResponse {
    pub fn builder() -> ServiceActivityResponseBuilder {
        <ServiceActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServiceActivityResponseBuilder {
    services: Option<Vec<ServiceActivityResponseServicesItem>>,
    buckets: Option<Vec<i64>>,
    signals: Option<Vec<ServiceActivityResponseSignalsItem>>,
    sdks: Option<Vec<ServiceActivityResponseSdksItem>>,
    retired: Option<Vec<String>>,
}

impl ServiceActivityResponseBuilder {
    pub fn services(mut self, value: Vec<ServiceActivityResponseServicesItem>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn buckets(mut self, value: Vec<i64>) -> Self {
        self.buckets = Some(value);
        self
    }

    pub fn signals(mut self, value: Vec<ServiceActivityResponseSignalsItem>) -> Self {
        self.signals = Some(value);
        self
    }

    pub fn sdks(mut self, value: Vec<ServiceActivityResponseSdksItem>) -> Self {
        self.sdks = Some(value);
        self
    }

    pub fn retired(mut self, value: Vec<String>) -> Self {
        self.retired = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServiceActivityResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`services`](ServiceActivityResponseBuilder::services)
    /// - [`buckets`](ServiceActivityResponseBuilder::buckets)
    /// - [`signals`](ServiceActivityResponseBuilder::signals)
    pub fn build(self) -> Result<ServiceActivityResponse, BuildError> {
        Ok(ServiceActivityResponse {
            services: self.services.ok_or_else(|| BuildError::missing_field("services"))?,
            buckets: self.buckets.ok_or_else(|| BuildError::missing_field("buckets"))?,
            signals: self.signals.ok_or_else(|| BuildError::missing_field("signals"))?,
            sdks: self.sdks,
            retired: self.retired,
        })
    }
}

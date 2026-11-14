pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ServiceActivityResponseSdksItem {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub spans: i64,
}

impl ServiceActivityResponseSdksItem {
    pub fn builder() -> ServiceActivityResponseSdksItemBuilder {
        <ServiceActivityResponseSdksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServiceActivityResponseSdksItemBuilder {
    service_id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    spans: Option<i64>,
}

impl ServiceActivityResponseSdksItemBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn spans(mut self, value: i64) -> Self {
        self.spans = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServiceActivityResponseSdksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](ServiceActivityResponseSdksItemBuilder::service_id)
    /// - [`name`](ServiceActivityResponseSdksItemBuilder::name)
    /// - [`version`](ServiceActivityResponseSdksItemBuilder::version)
    /// - [`spans`](ServiceActivityResponseSdksItemBuilder::spans)
    pub fn build(self) -> Result<ServiceActivityResponseSdksItem, BuildError> {
        Ok(ServiceActivityResponseSdksItem {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            spans: self.spans.ok_or_else(|| BuildError::missing_field("spans"))?,
        })
    }
}

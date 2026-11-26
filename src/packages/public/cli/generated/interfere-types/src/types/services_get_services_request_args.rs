pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetServicesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
}

impl GetServicesRequestArgs {
    pub fn builder() -> GetServicesRequestArgsBuilder {
        <GetServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetServicesRequestArgsBuilder {
    service_id: Option<String>,
}

impl GetServicesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](GetServicesRequestArgsBuilder::service_id)
    pub fn build(self) -> Result<GetServicesRequestArgs, BuildError> {
        Ok(GetServicesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
        })
    }
}

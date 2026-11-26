pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnarchiveServicesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
}

impl UnarchiveServicesRequestArgs {
    pub fn builder() -> UnarchiveServicesRequestArgsBuilder {
        <UnarchiveServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnarchiveServicesRequestArgsBuilder {
    service_id: Option<String>,
}

impl UnarchiveServicesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnarchiveServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](UnarchiveServicesRequestArgsBuilder::service_id)
    pub fn build(self) -> Result<UnarchiveServicesRequestArgs, BuildError> {
        Ok(UnarchiveServicesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
        })
    }
}

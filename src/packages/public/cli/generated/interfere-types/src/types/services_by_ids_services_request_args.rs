pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ByIdsServicesRequestArgs {
    #[serde(rename = "serviceIds")]
    #[serde(default)]
    pub service_ids: Vec<String>,
}

impl ByIdsServicesRequestArgs {
    pub fn builder() -> ByIdsServicesRequestArgsBuilder {
        <ByIdsServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ByIdsServicesRequestArgsBuilder {
    service_ids: Option<Vec<String>>,
}

impl ByIdsServicesRequestArgsBuilder {
    pub fn service_ids(mut self, value: Vec<String>) -> Self {
        self.service_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ByIdsServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_ids`](ByIdsServicesRequestArgsBuilder::service_ids)
    pub fn build(self) -> Result<ByIdsServicesRequestArgs, BuildError> {
        Ok(ByIdsServicesRequestArgs {
            service_ids: self.service_ids.ok_or_else(|| BuildError::missing_field("service_ids"))?,
        })
    }
}

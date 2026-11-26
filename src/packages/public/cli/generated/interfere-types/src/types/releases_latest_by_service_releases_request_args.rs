pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LatestByServiceReleasesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(rename = "primaryEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_environment: Option<String>,
}

impl LatestByServiceReleasesRequestArgs {
    pub fn builder() -> LatestByServiceReleasesRequestArgsBuilder {
        <LatestByServiceReleasesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LatestByServiceReleasesRequestArgsBuilder {
    service_id: Option<String>,
    primary_environment: Option<String>,
}

impl LatestByServiceReleasesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn primary_environment(mut self, value: impl Into<String>) -> Self {
        self.primary_environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LatestByServiceReleasesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](LatestByServiceReleasesRequestArgsBuilder::service_id)
    pub fn build(self) -> Result<LatestByServiceReleasesRequestArgs, BuildError> {
        Ok(LatestByServiceReleasesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            primary_environment: self.primary_environment,
        })
    }
}

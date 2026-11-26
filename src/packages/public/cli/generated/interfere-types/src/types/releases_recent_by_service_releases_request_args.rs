pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecentByServiceReleasesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub limit: f64,
}

impl RecentByServiceReleasesRequestArgs {
    pub fn builder() -> RecentByServiceReleasesRequestArgsBuilder {
        <RecentByServiceReleasesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecentByServiceReleasesRequestArgsBuilder {
    service_id: Option<String>,
    limit: Option<f64>,
}

impl RecentByServiceReleasesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: f64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecentByServiceReleasesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](RecentByServiceReleasesRequestArgsBuilder::service_id)
    /// - [`limit`](RecentByServiceReleasesRequestArgsBuilder::limit)
    pub fn build(self) -> Result<RecentByServiceReleasesRequestArgs, BuildError> {
        Ok(RecentByServiceReleasesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
        })
    }
}

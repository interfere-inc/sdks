pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetAnonymousUserTrackingServicesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(default)]
    pub enabled: bool,
}

impl SetAnonymousUserTrackingServicesRequestArgs {
    pub fn builder() -> SetAnonymousUserTrackingServicesRequestArgsBuilder {
        <SetAnonymousUserTrackingServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousUserTrackingServicesRequestArgsBuilder {
    service_id: Option<String>,
    enabled: Option<bool>,
}

impl SetAnonymousUserTrackingServicesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousUserTrackingServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](SetAnonymousUserTrackingServicesRequestArgsBuilder::service_id)
    /// - [`enabled`](SetAnonymousUserTrackingServicesRequestArgsBuilder::enabled)
    pub fn build(self) -> Result<SetAnonymousUserTrackingServicesRequestArgs, BuildError> {
        Ok(SetAnonymousUserTrackingServicesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkDestinationServicesRequestArgs {
    /// Identifier of the existing integration installation.
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    /// Destination project or Worker identifier exposed by the integration, or "cli" for the CLI.
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
}

impl LinkDestinationServicesRequestArgs {
    pub fn builder() -> LinkDestinationServicesRequestArgsBuilder {
        <LinkDestinationServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkDestinationServicesRequestArgsBuilder {
    connection_id: Option<String>,
    resource_id: Option<String>,
    service_id: Option<String>,
}

impl LinkDestinationServicesRequestArgsBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkDestinationServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](LinkDestinationServicesRequestArgsBuilder::connection_id)
    /// - [`resource_id`](LinkDestinationServicesRequestArgsBuilder::resource_id)
    /// - [`service_id`](LinkDestinationServicesRequestArgsBuilder::service_id)
    pub fn build(self) -> Result<LinkDestinationServicesRequestArgs, BuildError> {
        Ok(LinkDestinationServicesRequestArgs {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            resource_id: self.resource_id.ok_or_else(|| BuildError::missing_field("resource_id"))?,
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkRepositoryServicesRequestArgs {
    /// Identifier of the existing integration installation.
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    /// Repository identifier exposed by the integration.
    #[serde(rename = "repositoryId")]
    #[serde(default)]
    pub repository_id: String,
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    /// Repository-relative directory that holds this service's code, such as packages/services/api.
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl LinkRepositoryServicesRequestArgs {
    pub fn builder() -> LinkRepositoryServicesRequestArgsBuilder {
        <LinkRepositoryServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkRepositoryServicesRequestArgsBuilder {
    connection_id: Option<String>,
    repository_id: Option<String>,
    service_id: Option<String>,
    working_directory: Option<String>,
}

impl LinkRepositoryServicesRequestArgsBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn repository_id(mut self, value: impl Into<String>) -> Self {
        self.repository_id = Some(value.into());
        self
    }

    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn working_directory(mut self, value: impl Into<String>) -> Self {
        self.working_directory = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LinkRepositoryServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](LinkRepositoryServicesRequestArgsBuilder::connection_id)
    /// - [`repository_id`](LinkRepositoryServicesRequestArgsBuilder::repository_id)
    /// - [`service_id`](LinkRepositoryServicesRequestArgsBuilder::service_id)
    pub fn build(self) -> Result<LinkRepositoryServicesRequestArgs, BuildError> {
        Ok(LinkRepositoryServicesRequestArgs {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            repository_id: self.repository_id.ok_or_else(|| BuildError::missing_field("repository_id"))?,
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            working_directory: self.working_directory,
        })
    }
}

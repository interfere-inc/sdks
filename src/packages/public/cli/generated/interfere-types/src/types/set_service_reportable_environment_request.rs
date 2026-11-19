pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetServiceReportableEnvironmentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Provider resource that tells apart services sharing a namespace and name, such as vercel:prj_abc for one Vercel project. Omit it to address the only service with this namespace and name; null or an empty string addresses the service without a resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Only exceptions whose deployment.environment.name equals this value, compared case-insensitively, create findings and problems. Null reports every environment.
    #[serde(rename = "reportableEnvironment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportable_environment: Option<String>,
}

impl SetServiceReportableEnvironmentRequest {
    pub fn builder() -> SetServiceReportableEnvironmentRequestBuilder {
        <SetServiceReportableEnvironmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetServiceReportableEnvironmentRequestBuilder {
    namespace: Option<String>,
    name: Option<String>,
    resource: Option<String>,
    reportable_environment: Option<String>,
}

impl SetServiceReportableEnvironmentRequestBuilder {
    pub fn namespace(mut self, value: impl Into<String>) -> Self {
        self.namespace = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    pub fn reportable_environment(mut self, value: impl Into<String>) -> Self {
        self.reportable_environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetServiceReportableEnvironmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SetServiceReportableEnvironmentRequestBuilder::name)
    pub fn build(self) -> Result<SetServiceReportableEnvironmentRequest, BuildError> {
        Ok(SetServiceReportableEnvironmentRequest {
            namespace: self.namespace,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            resource: self.resource,
            reportable_environment: self.reportable_environment,
        })
    }
}


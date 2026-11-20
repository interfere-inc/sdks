pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteServiceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Provider resource that tells apart services sharing a namespace and name, such as vercel:prj_abc for one Vercel project. Omit it to address the only service with this namespace and name; when several share it, the call answers SERVICE_RESOURCE_REQUIRED with their resources. Null or an empty string addresses the service without a resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

impl DeleteServiceRequest {
    pub fn builder() -> DeleteServiceRequestBuilder {
        <DeleteServiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteServiceRequestBuilder {
    namespace: Option<String>,
    name: Option<String>,
    resource: Option<String>,
}

impl DeleteServiceRequestBuilder {
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

    /// Consumes the builder and constructs a [`DeleteServiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DeleteServiceRequestBuilder::name)
    pub fn build(self) -> Result<DeleteServiceRequest, BuildError> {
        Ok(DeleteServiceRequest {
            namespace: self.namespace,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            resource: self.resource,
        })
    }
}


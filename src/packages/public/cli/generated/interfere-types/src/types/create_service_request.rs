pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateServiceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Provider resource that tells apart services sharing a namespace and name, such as vercel:prj_abc for one Vercel project. Omit it to address the only service with this namespace and name; when several share it, the call answers SERVICE_RESOURCE_REQUIRED with their resources. Null or an empty string addresses the service without a resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The kind of data source the user chose. It is stored with type_source user, so telemetry never changes it, and it replaces a type inferred from earlier telemetry.
    pub r#type: CreateServiceRequestType,
}

impl CreateServiceRequest {
    pub fn builder() -> CreateServiceRequestBuilder {
        <CreateServiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServiceRequestBuilder {
    namespace: Option<String>,
    name: Option<String>,
    resource: Option<String>,
    r#type: Option<CreateServiceRequestType>,
}

impl CreateServiceRequestBuilder {
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

    pub fn r#type(mut self, value: CreateServiceRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateServiceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateServiceRequestBuilder::name)
    /// - [`r#type`](CreateServiceRequestBuilder::r#type)
    pub fn build(self) -> Result<CreateServiceRequest, BuildError> {
        Ok(CreateServiceRequest {
            namespace: self.namespace,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            resource: self.resource,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}


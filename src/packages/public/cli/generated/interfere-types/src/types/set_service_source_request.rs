pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetServiceSourceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Provider resource that tells apart services sharing a namespace and name, such as vercel:prj_abc for one Vercel project. Omit it to address the only service with this namespace and name; null or an empty string addresses the service without a resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Repository that holds this service's code, or null to remove the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SetServiceSourceRequestSource>,
}

impl SetServiceSourceRequest {
    pub fn builder() -> SetServiceSourceRequestBuilder {
        <SetServiceSourceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetServiceSourceRequestBuilder {
    namespace: Option<String>,
    name: Option<String>,
    resource: Option<String>,
    source: Option<SetServiceSourceRequestSource>,
}

impl SetServiceSourceRequestBuilder {
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

    pub fn source(mut self, value: SetServiceSourceRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetServiceSourceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SetServiceSourceRequestBuilder::name)
    pub fn build(self) -> Result<SetServiceSourceRequest, BuildError> {
        Ok(SetServiceSourceRequest {
            namespace: self.namespace,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            resource: self.resource,
            source: self.source,
        })
    }
}


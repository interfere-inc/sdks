pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemAffectedServicesResponseServicesItem {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    pub name: String,
    /// Provider resource that tells apart services sharing a namespace and name, such as vercel:prj_abc; null for a service without one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    pub r#type: ProblemAffectedServicesResponseServicesItemType,
    #[serde(default)]
    pub traces: i64,
}

impl ProblemAffectedServicesResponseServicesItem {
    pub fn builder() -> ProblemAffectedServicesResponseServicesItemBuilder {
        <ProblemAffectedServicesResponseServicesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemAffectedServicesResponseServicesItemBuilder {
    id: Option<String>,
    namespace: Option<String>,
    name: Option<String>,
    resource: Option<String>,
    r#type: Option<ProblemAffectedServicesResponseServicesItemType>,
    traces: Option<i64>,
}

impl ProblemAffectedServicesResponseServicesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn r#type(mut self, value: ProblemAffectedServicesResponseServicesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn traces(mut self, value: i64) -> Self {
        self.traces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemAffectedServicesResponseServicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProblemAffectedServicesResponseServicesItemBuilder::id)
    /// - [`name`](ProblemAffectedServicesResponseServicesItemBuilder::name)
    /// - [`r#type`](ProblemAffectedServicesResponseServicesItemBuilder::r#type)
    /// - [`traces`](ProblemAffectedServicesResponseServicesItemBuilder::traces)
    pub fn build(self) -> Result<ProblemAffectedServicesResponseServicesItem, BuildError> {
        Ok(ProblemAffectedServicesResponseServicesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            namespace: self.namespace,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            resource: self.resource,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            traces: self.traces.ok_or_else(|| BuildError::missing_field("traces"))?,
        })
    }
}

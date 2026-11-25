pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceOne {
    pub kind: RequiredEvidenceOneKind,
    #[serde(default)]
    pub provider: String,
    pub dependency: RequiredEvidenceOneDependency,
    pub capability: RequiredEvidenceOneCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: RequiredEvidenceOneIncidentScope,
}

impl RequiredEvidenceOne {
    pub fn builder() -> RequiredEvidenceOneBuilder {
        <RequiredEvidenceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceOneBuilder {
    kind: Option<RequiredEvidenceOneKind>,
    provider: Option<String>,
    dependency: Option<RequiredEvidenceOneDependency>,
    capability: Option<RequiredEvidenceOneCapability>,
    incident_scope: Option<RequiredEvidenceOneIncidentScope>,
}

impl RequiredEvidenceOneBuilder {
    pub fn kind(mut self, value: RequiredEvidenceOneKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn dependency(mut self, value: RequiredEvidenceOneDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    pub fn capability(mut self, value: RequiredEvidenceOneCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: RequiredEvidenceOneIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](RequiredEvidenceOneBuilder::kind)
    /// - [`provider`](RequiredEvidenceOneBuilder::provider)
    /// - [`dependency`](RequiredEvidenceOneBuilder::dependency)
    /// - [`capability`](RequiredEvidenceOneBuilder::capability)
    /// - [`incident_scope`](RequiredEvidenceOneBuilder::incident_scope)
    pub fn build(self) -> Result<RequiredEvidenceOne, BuildError> {
        Ok(RequiredEvidenceOne {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
        })
    }
}

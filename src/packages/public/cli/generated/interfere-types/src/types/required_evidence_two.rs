pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceTwo {
    pub kind: RequiredEvidenceTwoKind,
    #[serde(default)]
    pub provider: String,
    pub dependency: RequiredEvidenceTwoDependency,
    pub capability: RequiredEvidenceTwoCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: RequiredEvidenceTwoIncidentScope,
}

impl RequiredEvidenceTwo {
    pub fn builder() -> RequiredEvidenceTwoBuilder {
        <RequiredEvidenceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceTwoBuilder {
    kind: Option<RequiredEvidenceTwoKind>,
    provider: Option<String>,
    dependency: Option<RequiredEvidenceTwoDependency>,
    capability: Option<RequiredEvidenceTwoCapability>,
    incident_scope: Option<RequiredEvidenceTwoIncidentScope>,
}

impl RequiredEvidenceTwoBuilder {
    pub fn kind(mut self, value: RequiredEvidenceTwoKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn dependency(mut self, value: RequiredEvidenceTwoDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    pub fn capability(mut self, value: RequiredEvidenceTwoCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: RequiredEvidenceTwoIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](RequiredEvidenceTwoBuilder::kind)
    /// - [`provider`](RequiredEvidenceTwoBuilder::provider)
    /// - [`dependency`](RequiredEvidenceTwoBuilder::dependency)
    /// - [`capability`](RequiredEvidenceTwoBuilder::capability)
    /// - [`incident_scope`](RequiredEvidenceTwoBuilder::incident_scope)
    pub fn build(self) -> Result<RequiredEvidenceTwo, BuildError> {
        Ok(RequiredEvidenceTwo {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceThree {
    pub kind: RequiredEvidenceThreeKind,
    #[serde(default)]
    pub provider: String,
    pub dependency: RequiredEvidenceThreeDependency,
    pub capability: RequiredEvidenceThreeCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: RequiredEvidenceThreeIncidentScope,
}

impl RequiredEvidenceThree {
    pub fn builder() -> RequiredEvidenceThreeBuilder {
        <RequiredEvidenceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceThreeBuilder {
    kind: Option<RequiredEvidenceThreeKind>,
    provider: Option<String>,
    dependency: Option<RequiredEvidenceThreeDependency>,
    capability: Option<RequiredEvidenceThreeCapability>,
    incident_scope: Option<RequiredEvidenceThreeIncidentScope>,
}

impl RequiredEvidenceThreeBuilder {
    pub fn kind(mut self, value: RequiredEvidenceThreeKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn dependency(mut self, value: RequiredEvidenceThreeDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    pub fn capability(mut self, value: RequiredEvidenceThreeCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: RequiredEvidenceThreeIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](RequiredEvidenceThreeBuilder::kind)
    /// - [`provider`](RequiredEvidenceThreeBuilder::provider)
    /// - [`dependency`](RequiredEvidenceThreeBuilder::dependency)
    /// - [`capability`](RequiredEvidenceThreeBuilder::capability)
    /// - [`incident_scope`](RequiredEvidenceThreeBuilder::incident_scope)
    pub fn build(self) -> Result<RequiredEvidenceThree, BuildError> {
        Ok(RequiredEvidenceThree {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
        })
    }
}

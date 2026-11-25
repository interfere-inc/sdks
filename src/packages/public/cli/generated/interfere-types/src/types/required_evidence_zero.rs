pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceZero {
    pub kind: RequiredEvidenceZeroKind,
    #[serde(default)]
    pub provider: String,
    pub dependency: RequiredEvidenceZeroDependency,
    pub capability: RequiredEvidenceZeroCapability,
    #[serde(rename = "incidentScope")]
    #[serde(default)]
    pub incident_scope: RequiredEvidenceZeroIncidentScope,
}

impl RequiredEvidenceZero {
    pub fn builder() -> RequiredEvidenceZeroBuilder {
        <RequiredEvidenceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceZeroBuilder {
    kind: Option<RequiredEvidenceZeroKind>,
    provider: Option<String>,
    dependency: Option<RequiredEvidenceZeroDependency>,
    capability: Option<RequiredEvidenceZeroCapability>,
    incident_scope: Option<RequiredEvidenceZeroIncidentScope>,
}

impl RequiredEvidenceZeroBuilder {
    pub fn kind(mut self, value: RequiredEvidenceZeroKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn dependency(mut self, value: RequiredEvidenceZeroDependency) -> Self {
        self.dependency = Some(value);
        self
    }

    pub fn capability(mut self, value: RequiredEvidenceZeroCapability) -> Self {
        self.capability = Some(value);
        self
    }

    pub fn incident_scope(mut self, value: RequiredEvidenceZeroIncidentScope) -> Self {
        self.incident_scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](RequiredEvidenceZeroBuilder::kind)
    /// - [`provider`](RequiredEvidenceZeroBuilder::provider)
    /// - [`dependency`](RequiredEvidenceZeroBuilder::dependency)
    /// - [`capability`](RequiredEvidenceZeroBuilder::capability)
    /// - [`incident_scope`](RequiredEvidenceZeroBuilder::incident_scope)
    pub fn build(self) -> Result<RequiredEvidenceZero, BuildError> {
        Ok(RequiredEvidenceZero {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            dependency: self.dependency.ok_or_else(|| BuildError::missing_field("dependency"))?,
            capability: self.capability.ok_or_else(|| BuildError::missing_field("capability"))?,
            incident_scope: self.incident_scope.ok_or_else(|| BuildError::missing_field("incident_scope"))?,
        })
    }
}

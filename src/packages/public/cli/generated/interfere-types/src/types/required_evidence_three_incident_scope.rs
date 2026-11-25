pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequiredEvidenceThreeIncidentScope {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    /// The incident service id (the Problem's serviceId) or a repository Datasource id. Before MCP connection, keep this existing incident anchor. Use an authorized MCP evidence Datasource id only after discovery; never invent one to request a connection.
    #[serde(rename = "sourceId")]
    #[serde(default)]
    pub source_id: String,
    /// Exact requested trace/span id, replay session id, release id, device id, repository file path, or known provider record id. Before MCP discovery, keep the known incident record id; use an MCP resource URI only after authorized discovery. Never invent a URI or use an arbitrary record from the same service.
    #[serde(rename = "recordId")]
    #[serde(default)]
    pub record_id: String,
    #[serde(rename = "windowStart")]
    #[serde(default)]
    pub window_start: i64,
    #[serde(rename = "windowEnd")]
    #[serde(default)]
    pub window_end: i64,
}

impl RequiredEvidenceThreeIncidentScope {
    pub fn builder() -> RequiredEvidenceThreeIncidentScopeBuilder {
        <RequiredEvidenceThreeIncidentScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequiredEvidenceThreeIncidentScopeBuilder {
    problem_id: Option<String>,
    source_id: Option<String>,
    record_id: Option<String>,
    window_start: Option<i64>,
    window_end: Option<i64>,
}

impl RequiredEvidenceThreeIncidentScopeBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: impl Into<String>) -> Self {
        self.source_id = Some(value.into());
        self
    }

    pub fn record_id(mut self, value: impl Into<String>) -> Self {
        self.record_id = Some(value.into());
        self
    }

    pub fn window_start(mut self, value: i64) -> Self {
        self.window_start = Some(value);
        self
    }

    pub fn window_end(mut self, value: i64) -> Self {
        self.window_end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequiredEvidenceThreeIncidentScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](RequiredEvidenceThreeIncidentScopeBuilder::problem_id)
    /// - [`source_id`](RequiredEvidenceThreeIncidentScopeBuilder::source_id)
    /// - [`record_id`](RequiredEvidenceThreeIncidentScopeBuilder::record_id)
    /// - [`window_start`](RequiredEvidenceThreeIncidentScopeBuilder::window_start)
    /// - [`window_end`](RequiredEvidenceThreeIncidentScopeBuilder::window_end)
    pub fn build(self) -> Result<RequiredEvidenceThreeIncidentScope, BuildError> {
        Ok(RequiredEvidenceThreeIncidentScope {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            source_id: self.source_id.ok_or_else(|| BuildError::missing_field("source_id"))?,
            record_id: self.record_id.ok_or_else(|| BuildError::missing_field("record_id"))?,
            window_start: self.window_start.ok_or_else(|| BuildError::missing_field("window_start"))?,
            window_end: self.window_end.ok_or_else(|| BuildError::missing_field("window_end"))?,
        })
    }
}

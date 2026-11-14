pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetServiceSourceRequestSource {
    /// Repository Datasource that holds the code this service runs.
    #[serde(rename = "datasourceId")]
    #[serde(default)]
    pub datasource_id: String,
    /// Repository-relative directory that holds this service's code, such as packages/services/api.
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl SetServiceSourceRequestSource {
    pub fn builder() -> SetServiceSourceRequestSourceBuilder {
        <SetServiceSourceRequestSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetServiceSourceRequestSourceBuilder {
    datasource_id: Option<String>,
    working_directory: Option<String>,
}

impl SetServiceSourceRequestSourceBuilder {
    pub fn datasource_id(mut self, value: impl Into<String>) -> Self {
        self.datasource_id = Some(value.into());
        self
    }

    pub fn working_directory(mut self, value: impl Into<String>) -> Self {
        self.working_directory = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetServiceSourceRequestSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`datasource_id`](SetServiceSourceRequestSourceBuilder::datasource_id)
    pub fn build(self) -> Result<SetServiceSourceRequestSource, BuildError> {
        Ok(SetServiceSourceRequestSource {
            datasource_id: self.datasource_id.ok_or_else(|| BuildError::missing_field("datasource_id"))?,
            working_directory: self.working_directory,
        })
    }
}

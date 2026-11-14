pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MineSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl MineSettingsRequest {
    pub fn builder() -> MineSettingsRequestBuilder {
        <MineSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MineSettingsRequestBuilder {
    args: Option<HashMap<String, serde_json::Value>>,
}

impl MineSettingsRequestBuilder {
    pub fn args(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MineSettingsRequest`].
    pub fn build(self) -> Result<MineSettingsRequest, BuildError> {
        Ok(MineSettingsRequest {
            args: self.args,
        })
    }
}


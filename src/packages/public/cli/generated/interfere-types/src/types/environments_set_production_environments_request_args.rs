pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetProductionEnvironmentsRequestArgs {
    /// Workspace environment name as reported by the SDK, lowercased, such as production.
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isProduction")]
    #[serde(default)]
    pub is_production: bool,
}

impl SetProductionEnvironmentsRequestArgs {
    pub fn builder() -> SetProductionEnvironmentsRequestArgsBuilder {
        <SetProductionEnvironmentsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetProductionEnvironmentsRequestArgsBuilder {
    name: Option<String>,
    is_production: Option<bool>,
}

impl SetProductionEnvironmentsRequestArgsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetProductionEnvironmentsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](SetProductionEnvironmentsRequestArgsBuilder::name)
    /// - [`is_production`](SetProductionEnvironmentsRequestArgsBuilder::is_production)
    pub fn build(self) -> Result<SetProductionEnvironmentsRequestArgs, BuildError> {
        Ok(SetProductionEnvironmentsRequestArgs {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_production: self.is_production.ok_or_else(|| BuildError::missing_field("is_production"))?,
        })
    }
}

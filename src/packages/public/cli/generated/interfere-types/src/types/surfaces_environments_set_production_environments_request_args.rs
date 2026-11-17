pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetProductionEnvironmentsRequestArgs2 {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "isProduction")]
    #[serde(default)]
    pub is_production: bool,
}

impl SetProductionEnvironmentsRequestArgs2 {
    pub fn builder() -> SetProductionEnvironmentsRequestArgs2Builder {
        <SetProductionEnvironmentsRequestArgs2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetProductionEnvironmentsRequestArgs2Builder {
    id: Option<String>,
    is_production: Option<bool>,
}

impl SetProductionEnvironmentsRequestArgs2Builder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetProductionEnvironmentsRequestArgs2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetProductionEnvironmentsRequestArgs2Builder::id)
    /// - [`is_production`](SetProductionEnvironmentsRequestArgs2Builder::is_production)
    pub fn build(self) -> Result<SetProductionEnvironmentsRequestArgs2, BuildError> {
        Ok(SetProductionEnvironmentsRequestArgs2 {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_production: self.is_production.ok_or_else(|| BuildError::missing_field("is_production"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetProductionEnvironmentsRequest2 {
    #[serde(default)]
    pub args: SetProductionEnvironmentsRequestArgs2,
}

impl SetProductionEnvironmentsRequest2 {
    pub fn builder() -> SetProductionEnvironmentsRequest2Builder {
        <SetProductionEnvironmentsRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetProductionEnvironmentsRequest2Builder {
    args: Option<SetProductionEnvironmentsRequestArgs2>,
}

impl SetProductionEnvironmentsRequest2Builder {
    pub fn args(mut self, value: SetProductionEnvironmentsRequestArgs2) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetProductionEnvironmentsRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetProductionEnvironmentsRequest2Builder::args)
    pub fn build(self) -> Result<SetProductionEnvironmentsRequest2, BuildError> {
        Ok(SetProductionEnvironmentsRequest2 {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


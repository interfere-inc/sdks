pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteEnvironmentsRequest2 {
    #[serde(default)]
    pub args: DeleteEnvironmentsRequestArgs2,
}

impl DeleteEnvironmentsRequest2 {
    pub fn builder() -> DeleteEnvironmentsRequest2Builder {
        <DeleteEnvironmentsRequest2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteEnvironmentsRequest2Builder {
    args: Option<DeleteEnvironmentsRequestArgs2>,
}

impl DeleteEnvironmentsRequest2Builder {
    pub fn args(mut self, value: DeleteEnvironmentsRequestArgs2) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteEnvironmentsRequest2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](DeleteEnvironmentsRequest2Builder::args)
    pub fn build(self) -> Result<DeleteEnvironmentsRequest2, BuildError> {
        Ok(DeleteEnvironmentsRequest2 {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}


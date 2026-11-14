pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersListResponseItemSourceZero {
    pub name: QueryConsumersListResponseItemSourceZeroName,
    pub r#type: QueryConsumersListResponseItemSourceZeroType,
}

impl QueryConsumersListResponseItemSourceZero {
    pub fn builder() -> QueryConsumersListResponseItemSourceZeroBuilder {
        <QueryConsumersListResponseItemSourceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersListResponseItemSourceZeroBuilder {
    name: Option<QueryConsumersListResponseItemSourceZeroName>,
    r#type: Option<QueryConsumersListResponseItemSourceZeroType>,
}

impl QueryConsumersListResponseItemSourceZeroBuilder {
    pub fn name(mut self, value: QueryConsumersListResponseItemSourceZeroName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersListResponseItemSourceZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersListResponseItemSourceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersListResponseItemSourceZeroBuilder::name)
    /// - [`r#type`](QueryConsumersListResponseItemSourceZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersListResponseItemSourceZero, BuildError> {
        Ok(QueryConsumersListResponseItemSourceZero {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersListResponseItemSourceOne {
    pub name: QueryConsumersListResponseItemSourceOneName,
    pub r#type: QueryConsumersListResponseItemSourceOneType,
}

impl QueryConsumersListResponseItemSourceOne {
    pub fn builder() -> QueryConsumersListResponseItemSourceOneBuilder {
        <QueryConsumersListResponseItemSourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersListResponseItemSourceOneBuilder {
    name: Option<QueryConsumersListResponseItemSourceOneName>,
    r#type: Option<QueryConsumersListResponseItemSourceOneType>,
}

impl QueryConsumersListResponseItemSourceOneBuilder {
    pub fn name(mut self, value: QueryConsumersListResponseItemSourceOneName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryConsumersListResponseItemSourceOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersListResponseItemSourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersListResponseItemSourceOneBuilder::name)
    /// - [`r#type`](QueryConsumersListResponseItemSourceOneBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersListResponseItemSourceOne, BuildError> {
        Ok(QueryConsumersListResponseItemSourceOne {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

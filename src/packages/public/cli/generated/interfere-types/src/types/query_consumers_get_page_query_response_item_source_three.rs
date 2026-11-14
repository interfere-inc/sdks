pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersGetPageQueryResponseItemSourceThree {
    #[serde(default)]
    pub name: String,
    pub r#type: QueryConsumersGetPageQueryResponseItemSourceThreeType,
}

impl QueryConsumersGetPageQueryResponseItemSourceThree {
    pub fn builder() -> QueryConsumersGetPageQueryResponseItemSourceThreeBuilder {
        <QueryConsumersGetPageQueryResponseItemSourceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersGetPageQueryResponseItemSourceThreeBuilder {
    name: Option<String>,
    r#type: Option<QueryConsumersGetPageQueryResponseItemSourceThreeType>,
}

impl QueryConsumersGetPageQueryResponseItemSourceThreeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: QueryConsumersGetPageQueryResponseItemSourceThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersGetPageQueryResponseItemSourceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersGetPageQueryResponseItemSourceThreeBuilder::name)
    /// - [`r#type`](QueryConsumersGetPageQueryResponseItemSourceThreeBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersGetPageQueryResponseItemSourceThree, BuildError> {
        Ok(QueryConsumersGetPageQueryResponseItemSourceThree {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

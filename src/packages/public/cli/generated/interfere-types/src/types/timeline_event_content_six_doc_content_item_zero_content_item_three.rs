pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentSixDocContentItemZeroContentItemThree {
    pub r#type: TimelineEventContentSixDocContentItemZeroContentItemThreeType,
}

impl TimelineEventContentSixDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentSixDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentSixDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentSixDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<TimelineEventContentSixDocContentItemZeroContentItemThreeType>,
}

impl TimelineEventContentSixDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: TimelineEventContentSixDocContentItemZeroContentItemThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentSixDocContentItemZeroContentItemThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentSixDocContentItemZeroContentItemThreeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentSixDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentSixDocContentItemZeroContentItemThree {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

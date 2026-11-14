pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentFourDocContentItemZeroContentItemThree {
    pub r#type: TimelineEventContentFourDocContentItemZeroContentItemThreeType,
}

impl TimelineEventContentFourDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentFourDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentFourDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentFourDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<TimelineEventContentFourDocContentItemZeroContentItemThreeType>,
}

impl TimelineEventContentFourDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: TimelineEventContentFourDocContentItemZeroContentItemThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentFourDocContentItemZeroContentItemThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentFourDocContentItemZeroContentItemThreeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentFourDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentFourDocContentItemZeroContentItemThree {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

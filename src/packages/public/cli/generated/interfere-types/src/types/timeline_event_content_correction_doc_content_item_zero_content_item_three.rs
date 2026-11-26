pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemThree {
    pub r#type: TimelineEventContentCorrectionDocContentItemZeroContentItemThreeType,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemThree {
    pub fn builder() -> TimelineEventContentCorrectionDocContentItemZeroContentItemThreeBuilder {
        <TimelineEventContentCorrectionDocContentItemZeroContentItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentCorrectionDocContentItemZeroContentItemThreeBuilder {
    r#type: Option<TimelineEventContentCorrectionDocContentItemZeroContentItemThreeType>,
}

impl TimelineEventContentCorrectionDocContentItemZeroContentItemThreeBuilder {
    pub fn r#type(mut self, value: TimelineEventContentCorrectionDocContentItemZeroContentItemThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentCorrectionDocContentItemZeroContentItemThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TimelineEventContentCorrectionDocContentItemZeroContentItemThreeBuilder::r#type)
    pub fn build(self) -> Result<TimelineEventContentCorrectionDocContentItemZeroContentItemThree, BuildError> {
        Ok(TimelineEventContentCorrectionDocContentItemZeroContentItemThree {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

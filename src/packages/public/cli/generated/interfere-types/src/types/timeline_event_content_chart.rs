pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentChart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<TimelineEventContentChartChart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram: Option<TimelineEventContentChartDiagram>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<TimelineEventContentChartDoc>,
    #[serde(rename = "compactText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact_text: Option<String>,
    #[serde(rename = "seenAgain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_again: Option<TimelineEventContentChartSeenAgain>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub version: String,
}

impl TimelineEventContentChart {
    pub fn builder() -> TimelineEventContentChartBuilder {
        <TimelineEventContentChartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartBuilder {
    chart: Option<TimelineEventContentChartChart>,
    diagram: Option<TimelineEventContentChartDiagram>,
    doc: Option<TimelineEventContentChartDoc>,
    compact_text: Option<String>,
    seen_again: Option<TimelineEventContentChartSeenAgain>,
    text: Option<String>,
    version: Option<String>,
}

impl TimelineEventContentChartBuilder {
    pub fn chart(mut self, value: TimelineEventContentChartChart) -> Self {
        self.chart = Some(value);
        self
    }

    pub fn diagram(mut self, value: TimelineEventContentChartDiagram) -> Self {
        self.diagram = Some(value);
        self
    }

    pub fn doc(mut self, value: TimelineEventContentChartDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn compact_text(mut self, value: impl Into<String>) -> Self {
        self.compact_text = Some(value.into());
        self
    }

    pub fn seen_again(mut self, value: TimelineEventContentChartSeenAgain) -> Self {
        self.seen_again = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TimelineEventContentChartBuilder::text)
    /// - [`version`](TimelineEventContentChartBuilder::version)
    pub fn build(self) -> Result<TimelineEventContentChart, BuildError> {
        Ok(TimelineEventContentChart {
            chart: self.chart,
            diagram: self.diagram,
            doc: self.doc,
            compact_text: self.compact_text,
            seen_again: self.seen_again,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}

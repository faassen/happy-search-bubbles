use scraper::{error::SelectorErrorKind, Html};
use thiserror::Error;

use super::Topic;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TopicParseError {
    #[error("Selector error: {0}")]
    SelectorError(String),
}

impl<'a> From<SelectorErrorKind<'a>> for TopicParseError {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        TopicParseError::SelectorError(error.to_string())
    }
}

impl Topic {
    pub fn parse_html(html: &Html) -> Result<Self, TopicParseError> {
        Ok(Topic {
            descriptions: vec![],
        })
    }
}

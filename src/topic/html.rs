use scraper::{error::SelectorErrorKind, Html, Selector};
use thiserror::Error;

use super::{Topic, TopicDescription};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TopicParseError {
    #[error("Topic description without href: {0}")]
    MissingHref(String),
    #[error("Invalid URL in topic description: {0}")]
    InvalidUrl(String),
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
        let description_selector = Selector::parse("a.bubble-wikipedia-topic")?;
        let descriptions = html
            .select(&description_selector)
            .map(|description| {
                let href = description
                    .value()
                    .attr("href")
                    .ok_or_else(|| TopicParseError::MissingHref(description.html()))?;
                Ok(TopicDescription::Wikipedia(href.try_into().map_err(
                    |_| TopicParseError::InvalidUrl(description.html()),
                )?))
            })
            .collect::<Result<Vec<_>, TopicParseError>>()?;
        Ok(Topic { descriptions })
    }
}

#[cfg(test)]
mod tests {
    use crate::topic::model::TopicDescription;

    use super::*;

    #[test]
    fn test_parse_wikipedia_topic_description() {
        let html = r#"<html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <div>
                    <a class="bubble-wikipedia-topic" href="https://en.wikipedia.org/wiki/HTML">HTML</a>
                </div>
            </body>
        </html>"#;
        let document = Html::parse_document(html);
        let topic = Topic::parse_html(&document).unwrap();
        assert_eq!(topic.descriptions.len(), 1);
        assert_eq!(
            topic.descriptions[0],
            TopicDescription::Wikipedia("https://en.wikipedia.org/wiki/HTML".try_into().unwrap())
        );
    }

    #[test]
    fn test_parse_wikipedia_topic_description_without_href() {
        let html = r#"<html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <div>
                    <a class="bubble-wikipedia-topic">HTML</a>
                </div>
            </body>
        </html>"#;
        let document = Html::parse_document(html);
        let err = Topic::parse_html(&document).unwrap_err();
        assert_eq!(
            err,
            TopicParseError::MissingHref(
                r#"<a class="bubble-wikipedia-topic">HTML</a>"#.to_string()
            )
        );
    }

    #[test]
    fn test_parse_wikipedia_topic_description_wrong_href() {
        let html = r#"<html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <div>
                    <a class="bubble-wikipedia-topic" href="very-broken">HTML</a>
                </div>
            </body>
        </html>"#;
        let document = Html::parse_document(html);
        let err = Topic::parse_html(&document).unwrap_err();
        assert_eq!(
            err,
            TopicParseError::InvalidUrl(
                r#"<a class="bubble-wikipedia-topic" href="very-broken">HTML</a>"#.to_string()
            )
        );
    }
}

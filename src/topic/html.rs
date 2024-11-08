use iri_string::types::IriAbsoluteString;
use scraper::{error::SelectorErrorKind, Html, Selector};
use thiserror::Error;

use super::{Topic, TopicCategory, TopicReference};

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
    pub(crate) fn parse_html(html: &Html) -> Result<Self, TopicParseError> {
        let wikipedia_references = Self::make_references(
            html,
            Selector::parse("a.bubble-wikipedia-topic")?,
            TopicCategory::Wikipedia,
        )?;

        let wikidata_references = Self::make_references(
            html,
            Selector::parse("a.bubble-wikidata-topic")?,
            TopicCategory::Wikidata,
        )?;
        let references = wikipedia_references
            .into_iter()
            .chain(wikidata_references)
            .collect();
        Ok(Topic { references })
    }

    fn make_references(
        html: &Html,
        selector: Selector,
        category: TopicCategory,
    ) -> Result<Vec<TopicReference>, TopicParseError> {
        html.select(&selector)
            .map(|description| {
                let href = description
                    .value()
                    .attr("href")
                    .ok_or_else(|| TopicParseError::MissingHref(description.html()))?;
                Ok(TopicReference {
                    uri: href
                        .try_into()
                        .map_err(|_| TopicParseError::InvalidUrl(description.html()))?,
                    category: category.clone(),
                    label: description.text().collect(),
                })
            })
            .collect::<Result<Vec<_>, TopicParseError>>()
    }
}

#[cfg(test)]
mod tests {

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
        assert_eq!(topic.references.len(), 1);
        assert_eq!(
            topic.references[0],
            TopicReference {
                uri: "https://en.wikipedia.org/wiki/HTML".try_into().unwrap(),
                label: "HTML".to_string(),
                category: TopicCategory::Wikipedia,
            }
        );
    }

    #[test]
    fn test_parse_wikidata_topic_description() {
        let html = r#"<html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <div>
                    <a class="bubble-wikidata-topic" href="https://www.wikidata.org/wiki/Q8811">HTML</a>
                </div>
            </body>
        </html>"#;
        let document = Html::parse_document(html);
        let topic = Topic::parse_html(&document).unwrap();
        assert_eq!(topic.references.len(), 1);
        assert_eq!(
            topic.references[0],
            TopicReference {
                uri: "https://www.wikidata.org/wiki/Q8811".try_into().unwrap(),
                label: "HTML".to_string(),
                category: TopicCategory::Wikidata,
            }
        );
    }

    #[test]
    fn test_parse_combined_topic_description() {
        let html = r#"<html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <div>
                    <a class="bubble-wikipedia-topic" href="https://en.wikipedia.org/wiki/HTML">HTML</a>
                    <a class="bubble-wikidata-topic" href="https://www.wikidata.org/wiki/Q8811">HTML</a>
                </div>
            </body>
        </html>"#;
        let document = Html::parse_document(html);
        let topic = Topic::parse_html(&document).unwrap();

        assert_eq!(
            topic.references,
            vec![
                TopicReference {
                    uri: "https://en.wikipedia.org/wiki/HTML".try_into().unwrap(),
                    label: "HTML".to_string(),
                    category: TopicCategory::Wikipedia,
                },
                TopicReference {
                    uri: "https://www.wikidata.org/wiki/Q8811".try_into().unwrap(),
                    label: "HTML".to_string(),
                    category: TopicCategory::Wikidata,
                }
            ]
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

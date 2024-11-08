use scraper::{error::SelectorErrorKind, Html, Selector};
use thiserror::Error;

use crate::{
    indexable::{Indexable, IndexableParseError},
    topic::{Topic, TopicParseError},
};

use super::model::{Bubble, BubbleReference};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BubbleParseError {
    #[error("Missing title element in HTML document")]
    MissingTitle,
    #[error("Missing href in expands: {0}")]
    MissingHref(String),
    #[error("Invalid URL in expands: {0}")]
    InvalidUrl(String),
    #[error("Could not parse topic: {0}")]
    TopicError(#[from] TopicParseError),
    #[error("Could not parse indexables: {0}")]
    IndexableError(#[from] IndexableParseError),
    #[error("Selector error: {0}")]
    SelectorError(String),
}

impl<'a> From<SelectorErrorKind<'a>> for BubbleParseError {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        BubbleParseError::SelectorError(error.to_string())
    }
}

impl Bubble {
    pub fn parse_html(document: &str) -> Result<Self, BubbleParseError> {
        let document = Html::parse_document(document);
        let title = Self::parse_title(&document)?;
        let indexables = Indexable::parse_indexables(&document)?;
        let excludes = Indexable::parse_excludes(&document)?;
        let expands = Self::make_expands(&document)?;

        Ok(Bubble {
            title,
            topic: Topic::parse_html(&document)?,
            expands,
            indexables,
            excludes,
        })
    }

    // the title is the first title element in the first head element
    fn parse_title(html: &Html) -> Result<String, BubbleParseError> {
        let head_selector = Selector::parse("head")?;
        let title_selector = Selector::parse("title")?;

        let mut head = html.select(&head_selector);

        if let Some(head) = head.next() {
            let mut title = head.select(&title_selector);
            if let Some(title) = title.next() {
                let mut title_text = title.text();
                if let Some(title_text) = title_text.next() {
                    Ok(title_text.to_string())
                } else {
                    Err(BubbleParseError::MissingTitle)
                }
            } else {
                Err(BubbleParseError::MissingTitle)
            }
        } else {
            unreachable!("Head element should always be present in HTML document");
        }
    }

    fn make_expands(html: &Html) -> Result<Vec<BubbleReference>, BubbleParseError> {
        let selector = Selector::parse("a.bubble-expand")?;
        html.select(&selector)
            .map(|expands| {
                let href = expands
                    .value()
                    .attr("href")
                    .ok_or_else(|| BubbleParseError::MissingHref(expands.html()))?;
                Ok(BubbleReference {
                    uri: href
                        .try_into()
                        .map_err(|_| BubbleParseError::InvalidUrl(expands.html()))?,
                    label: expands.text().collect(),
                })
            })
            .collect::<Result<Vec<_>, BubbleParseError>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        indexable::Scope,
        topic::{TopicCategory, TopicReference},
    };

    use super::*;

    #[test]
    fn test_title() {
        let html = r#"<html>
            <head>
                <title>Test Title</title>
            </head>
            <body>
                <h1>Test Body</h1>
            </body>
        </html>"#;
        let bubble = Bubble::parse_html(html).unwrap();
        assert_eq!(bubble.title, "Test Title");
    }

    #[test]
    fn test_title_missing() {
        let html = r#"<html>
            <head>
            </head>
            <body>
                <h1>Test Body</h1>
            </body>
        </html>"#;
        let err = Bubble::parse_html(html).unwrap_err();
        assert_eq!(err, BubbleParseError::MissingTitle);
    }

    #[test]
    fn test_title_text_missing() {
        let html = r#"<html>
            <head>
              <title></title>
            </head>
            <body>
                <h1>Test Body</h1>
            </body>
        </html>"#;
        let err = Bubble::parse_html(html).unwrap_err();
        assert_eq!(err, BubbleParseError::MissingTitle);
    }

    #[test]
    fn test_head_missing() {
        let html = r#"<html>
            <body>
                <h1>Test Body</h1>
            </body>
        </html>"#;
        let err = Bubble::parse_html(html).unwrap_err();
        assert_eq!(err, BubbleParseError::MissingTitle);
    }

    #[test]
    fn test_full() {
        let html = r#"<html>
            <head>
                <title>Test Title</title>
            </head>
            <body>
                <a class="bubble-wikidata-topic" href="https://www.wikidata.org/wiki/Q8811">HTML</a>
                <a class="bubble-expand" href="https://another.org/my-bubble">Another bubble!</a>
                <a class="bubble-search-page" href="https://example.com/a">Search page</a>
                <a class="bubble-search-site" href="https://example.com">Search site</a>
                <a class="bubble-search-path" href="https://example.com/b">Search path</a>
                <a class="bubble-exclude-page" href="https://example.com/d">Search page</a>
                <a class="bubble-exclude-site" href="https://another.com">Search site</a>
                <a class="bubble-exclude-path" href="https://example.com/c">Search path</a> 
            </body>
        </html>"#;
        let bubble = Bubble::parse_html(html).unwrap();
        assert_eq!(bubble.title, "Test Title");
        assert_eq!(
            bubble.topic.references(),
            vec![TopicReference::new(
                "https://www.wikidata.org/wiki/Q8811".try_into().unwrap(),
                "HTML".to_string(),
                TopicCategory::Wikidata
            )]
        );
        assert_eq!(
            bubble.expands,
            vec![BubbleReference {
                uri: "https://another.org/my-bubble".parse().unwrap(),
                label: "Another bubble!".to_string(),
            }]
        );
        assert_eq!(
            bubble.indexables,
            vec![
                Indexable::new(
                    "https://example.com/a".parse().unwrap(),
                    Scope::Page,
                    "Search page".to_string()
                ),
                Indexable::new(
                    "https://example.com".parse().unwrap(),
                    Scope::Site,
                    "Search site".to_string()
                ),
                Indexable::new(
                    "https://example.com/b".parse().unwrap(),
                    Scope::Path,
                    "Search path".to_string()
                ),
            ]
        );
        assert_eq!(
            bubble.excludes,
            vec![
                Indexable::new(
                    "https://example.com/d".parse().unwrap(),
                    Scope::Page,
                    "Search page".to_string()
                ),
                Indexable::new(
                    "https://another.com".parse().unwrap(),
                    Scope::Site,
                    "Search site".to_string()
                ),
                Indexable::new(
                    "https://example.com/c".parse().unwrap(),
                    Scope::Path,
                    "Search path".to_string()
                ),
            ]
        );
    }
}

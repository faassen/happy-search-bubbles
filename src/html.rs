#[derive(Debug, PartialEq, Eq)]
pub enum BubbleLoadingError {
    MissingTitle,
    SelectorError(String),
}

impl<'a> From<SelectorErrorKind<'a>> for BubbleLoadingError {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        BubbleLoadingError::SelectorError(error.to_string())
    }
}

use crate::model::{Bubble, Topic};

use scraper::{error::SelectorErrorKind, Html, Selector};

impl Bubble {
    fn parse_html(document: &str) -> Result<Self, BubbleLoadingError> {
        let document = Html::parse_document(document);
        let title = Self::parse_title(&document)?;
        Ok(Bubble {
            topic: Topic {
                title,
                descriptions: vec![],
            },
            indexables: vec![],
            not_indexables: vec![],
        })
    }

    // the title is the first title element in the first head element
    fn parse_title(html: &Html) -> Result<String, BubbleLoadingError> {
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
                    Err(BubbleLoadingError::MissingTitle)
                }
            } else {
                Err(BubbleLoadingError::MissingTitle)
            }
        } else {
            unreachable!("Head element should always be present in HTML document");
        }
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(bubble.topic.title, "Test Title");
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
        assert_eq!(err, BubbleLoadingError::MissingTitle);
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
        assert_eq!(err, BubbleLoadingError::MissingTitle);
    }

    #[test]
    fn test_head_missing() {
        let html = r#"<html>
            <body>
                <h1>Test Body</h1>
            </body>
        </html>"#;
        let err = Bubble::parse_html(html).unwrap_err();
        assert_eq!(err, BubbleLoadingError::MissingTitle);
    }
}

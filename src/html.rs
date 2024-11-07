#[derive(Debug)]
pub enum BubbleLoadingError<'a> {
    MissingHead,
    MultipleHeads,
    MissingTitle,
    MultipleTitles,
    SelectorError(SelectorErrorKind<'a>),
}

impl<'a> From<SelectorErrorKind<'a>> for BubbleLoadingError<'a> {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        BubbleLoadingError::SelectorError(error)
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

    fn parse_title<'a>(html: &Html) -> Result<String, BubbleLoadingError<'a>> {
        let head_selector = Selector::parse("head")?;
        let title_selector = Selector::parse("title")?;

        let head = html.select(&head_selector);
        let heads = head.collect::<Vec<_>>();
        match heads.len() {
            1 => {
                let head = heads[0];
                let title = head.select(&title_selector);
                let titles = title.collect::<Vec<_>>();
                match titles.len() {
                    1 => {
                        let title = titles[0];
                        let title_text = title.text().collect::<Vec<_>>();
                        match title_text.len() {
                            1 => Ok(title_text[0].to_string()),
                            _ => Err(BubbleLoadingError::MultipleTitles),
                        }
                    }
                    0 => Err(BubbleLoadingError::MissingTitle),
                    _ => Err(BubbleLoadingError::MultipleTitles),
                }
            }
            0 => Err(BubbleLoadingError::MissingHead),
            _ => Err(BubbleLoadingError::MultipleHeads),
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
            </body>"#;
        let bubble = Bubble::parse_html(html).unwrap();
        assert_eq!(bubble.topic.title, "Test Title");
    }
}

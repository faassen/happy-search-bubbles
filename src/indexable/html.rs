use iri_string::types::IriAbsoluteString;
use scraper::{error::SelectorErrorKind, Html, Selector};
use thiserror::Error;

use super::{Indexable, Scope};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum IndexableParseError {
    #[error("Search link without URL: {0}")]
    MissingHref(String),
    #[error("Invalid URL in search link: {0}")]
    InvalidUrl(String),
    #[error("Selector error: {0}")]
    SelectorError(String),
}

impl<'a> From<SelectorErrorKind<'a>> for IndexableParseError {
    fn from(error: SelectorErrorKind<'a>) -> Self {
        IndexableParseError::SelectorError(error.to_string())
    }
}

impl Indexable {
    pub fn parse_indexables(html: &Html) -> Result<Vec<Self>, IndexableParseError> {
        let pages =
            Self::make_indexables(html, Selector::parse("a.bubble-search-page")?, Scope::Page)?;
        let sites =
            Self::make_indexables(html, Selector::parse("a.bubble-search-site")?, Scope::Site)?;
        let paths =
            Self::make_indexables(html, Selector::parse("a.bubble-search-path")?, Scope::Path)?;
        let pages = pages.into_iter().chain(sites).chain(paths).collect();
        Ok(pages)
    }

    pub fn parse_excludes(html: &Html) -> Result<Vec<Self>, IndexableParseError> {
        let pages =
            Self::make_indexables(html, Selector::parse("a.bubble-exclude-page")?, Scope::Page)?;
        let sites =
            Self::make_indexables(html, Selector::parse("a.bubble-exclude-site")?, Scope::Site)?;
        let paths =
            Self::make_indexables(html, Selector::parse("a.bubble-exclude-path")?, Scope::Path)?;
        let pages = pages.into_iter().chain(sites).chain(paths).collect();
        Ok(pages)
    }

    fn make_indexables(
        html: &Html,
        selector: Selector,
        scope: Scope,
    ) -> Result<Vec<Indexable>, IndexableParseError> {
        html.select(&selector)
            .map(|indexable| {
                let href = indexable
                    .value()
                    .attr("href")
                    .ok_or_else(|| IndexableParseError::MissingHref(indexable.html()))?;
                Ok(Indexable {
                    uri: href
                        .try_into()
                        .map_err(|_| IndexableParseError::InvalidUrl(indexable.html()))?,
                    scope: scope.clone(),
                })
            })
            .collect::<Result<Vec<_>, IndexableParseError>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indexable_page() {
        let html = r#"
        <html>
            <body>
                <a class="bubble-search-page" href="https://example.com">Example</a>
            </body>
        </html>
        "#;
        let document = Html::parse_document(html);
        let indexables = Indexable::parse_indexables(&document);
        assert_eq!(
            indexables,
            Ok(vec![Indexable {
                uri: "https://example.com".parse().unwrap(),
                scope: Scope::Page,
            }])
        );
    }

    #[test]
    fn test_parse_indexable_mixture() {
        let html = r#"
        <html>
            <body>
                <a class="bubble-search-page" href="https://example.com/a">Example</a>
                <a class="bubble-search-site" href="https://example.com">Example</a>
                <a class="bubble-search-path" href="https://example.com/b">Example</a>
            </body>
        </html>
        "#;
        let document = Html::parse_document(html);
        let indexables = Indexable::parse_indexables(&document);
        assert_eq!(
            indexables,
            Ok(vec![
                Indexable {
                    uri: "https://example.com/a".parse().unwrap(),
                    scope: Scope::Page,
                },
                Indexable {
                    uri: "https://example.com".parse().unwrap(),
                    scope: Scope::Site,
                },
                Indexable {
                    uri: "https://example.com/b".parse().unwrap(),
                    scope: Scope::Path,
                }
            ])
        );
    }

    #[test]
    fn test_parse_exclude_mixture() {
        let html = r#"
        <html>
            <body>
                <a class="bubble-exclude-page" href="https://example.com/a">Example</a>
                <a class="bubble-exclude-site" href="https://example.com">Example</a>
                <a class="bubble-exclude-path" href="https://example.com/b">Example</a>
            </body>
        </html>
        "#;
        let document = Html::parse_document(html);
        let indexables = Indexable::parse_excludes(&document);
        assert_eq!(
            indexables,
            Ok(vec![
                Indexable {
                    uri: "https://example.com/a".parse().unwrap(),
                    scope: Scope::Page,
                },
                Indexable {
                    uri: "https://example.com".parse().unwrap(),
                    scope: Scope::Site,
                },
                Indexable {
                    uri: "https://example.com/b".parse().unwrap(),
                    scope: Scope::Path,
                }
            ])
        );
    }
}

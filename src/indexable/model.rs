use iri_string::types::IriAbsoluteString;

#[derive(Debug, PartialEq, Eq)]
pub struct Indexable {
    pub(super) uri: IriAbsoluteString,
    pub(super) scope: Scope,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Scope {
    Site, // all linked pages on given URL, as long as it's the same site
    Path, // only the page at the given path and anything linked under that path
    Page, // only this page, nothing else
}

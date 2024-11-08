use iri_string::types::IriAbsoluteString;

use crate::{indexable::Indexable, topic::Topic};

#[derive(Debug)]
pub struct Bubble {
    pub(super) title: String,
    pub(super) topic: Topic,
    pub(super) indexables: Vec<Indexable>,
    pub(super) not_indexables: Vec<Indexable>,
}

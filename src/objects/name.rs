use std::hash::{Hash, Hasher};

use crate::PdfElement;

pub struct Name {
    pub name: Vec<u8>,
}

impl PdfElement for Name {
    fn print(&self) -> Vec<u8> {
        [b"/", self.name.as_slice()].concat()
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Name {}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

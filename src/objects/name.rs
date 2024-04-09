use std::hash::{Hash, Hasher};

use crate::PdfElement;

pub struct PdfName {
    name: Vec<u8>,
}

impl PdfElement for PdfName {
    fn print(&self) -> Vec<u8> {
        [b"/", self.name.as_slice()].concat()
    }
}

impl PartialEq for PdfName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PdfName {}

impl Hash for PdfName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PdfName {
    pub fn new(text: &str) -> Self {
        PdfName { name: text.into() }
    }
}

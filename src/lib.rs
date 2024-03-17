use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub trait PdfElement {
    fn print(&self) -> Vec<u8>;
}

pub struct Document {
    pub header: Header,
    pub trailer: Trailer,
}

impl PdfElement for Document {
    fn print(&self) -> Vec<u8> {
        [
            self.header.print().as_slice(),
            b"\n\n",
            self.trailer.print().as_slice(),
        ]
        .concat()
    }
}

pub struct Header {
    pub pdf_version: Vec<u8>,
}

impl PdfElement for Header {
    fn print(&self) -> Vec<u8> {
        [b"%PDF-", self.pdf_version.as_slice()].concat()
    }
}

pub struct Trailer {
    pub dictionary: Dictionary,
}

impl PdfElement for Trailer {
    fn print(&self) -> Vec<u8> {
        [b"trailer\n", self.dictionary.print().as_slice()].concat()
    }
}

pub struct Reference {
    pub identifier: u32,
    pub update: u32,
}

impl PdfElement for Reference {
    fn print(&self) -> Vec<u8> {
        format!("{} {} R", self.identifier, self.update)
            .as_bytes()
            .to_vec()
    }
}

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

pub struct Dictionary {
    pub dict: HashMap<Name, Box<dyn PdfElement>>,
}

impl PdfElement for Dictionary {
    fn print(&self) -> Vec<u8> {
        let mut entries: Vec<u8> = Vec::new();
        entries.extend(b"<<");

        for (key, value) in &(self.dict) {
            entries.extend([b"\n", key.print().as_slice(), b" "].concat());
            entries.extend(value.print());
        }

        entries.extend(b"\n>>");

        entries
    }
}

use std::collections::HashMap;

use crate::{PdfElement, PdfName};

pub struct Dictionary {
    dict: HashMap<PdfName, Box<dyn PdfElement>>,
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

impl Dictionary {
    pub fn new() -> Dictionary {
        return Dictionary {
            dict: HashMap::new(),
        };
    }

    pub fn set(&mut self, name: &str, value: Box<dyn PdfElement>) -> &mut Self {
        self.dict.insert(PdfName::new(name), value);

        self
    }
}

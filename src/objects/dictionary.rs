use std::collections::HashMap;

use crate::{Name, PdfElement};

pub struct Dictionary {
    dict: HashMap<Name, Box<dyn PdfElement>>,
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
        self.dict.insert(Name { name: name.into() }, value);

        self
    }
}

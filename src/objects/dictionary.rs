use std::{any::Any, collections::BTreeMap};

use crate::{PdfElement, PdfName};

pub struct Dictionary {
    dict: BTreeMap<String, Box<dyn PdfElement>>,
}

impl PdfElement for Dictionary {
    fn print(&self) -> Vec<u8> {
        let mut entries: Vec<u8> = Vec::new();
        entries.extend(b"<<");

        for (key, value) in &(self.dict) {
            entries.extend([b"\n", PdfName::new(key).print().as_slice(), b" "].concat());
            entries.extend(value.print());
        }

        entries.extend(b"\n>>");

        entries
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Dictionary {
    pub fn new() -> Dictionary {
        return Dictionary {
            dict: BTreeMap::new(),
        };
    }

    pub fn insert(&mut self, name: &str, value: impl PdfElement + 'static) -> &mut Self {
        self.dict.insert(name.to_string(), Box::new(value));

        self
    }

    pub fn get(&self, key: &str) -> Option<&dyn PdfElement> {
        match self.dict.get(key) {
            None => None,
            Some(element) => Some(element.as_ref()),
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut dyn PdfElement> {
        match self.dict.get_mut(key) {
            None => None,
            Some(element) => Some(element.as_mut()),
        }
    }
}

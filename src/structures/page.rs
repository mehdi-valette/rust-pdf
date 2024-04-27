use crate::{
    document::Document,
    objects::{Dictionary, PdfArray, PdfName, Stream},
};

use super::{make_rectangle, Rectangle};

pub struct Page<'a> {
    document: &'a mut Document,
    dictionary: Dictionary,
}

impl<'a> Page<'a> {
    pub fn new(document: &'a mut Document) -> Page<'a> {
        let page_root = document.get_page_root_reference();

        let mut dictionary = Dictionary::new();
        dictionary
            .insert("Parent", page_root)
            .insert("Type", PdfName::new("Page"))
            .insert("Resources", Dictionary::new());

        Page {
            document,
            dictionary,
        }
    }

    pub fn get_dictionary(self) -> Dictionary {
        self.dictionary
    }

    pub fn add_font_type_1(&mut self, font_name: &str, font_family: &str) -> &mut Self {
        let mut font = Dictionary::new();
        font.insert("Type", PdfName::new("Font"))
            .insert("Subtype", PdfName::new("Type1"))
            .insert("BaseFont", PdfName::new(font_family));

        self.dictionary
            .get_mut("Resources")
            .expect("Resources don't exist")
            .as_any_mut()
            .downcast_mut::<Dictionary>()
            .expect("Resources isn't a dictionary")
            .insert(font_name, font);

        self
    }

    pub fn add_content(&mut self, data: Vec<u8>) -> &mut Self {
        let content_ref = self
            .document
            .push(Box::new(Stream::new(data)))
            .get_reference();

        match self.dictionary.get_mut("Contents") {
            None => {
                self.dictionary
                    .insert("Contents", PdfArray::make([content_ref].into()));
            }
            Some(dictionary) => {
                dictionary
                    .as_any_mut()
                    .downcast_mut::<PdfArray>()
                    .expect("content isn't an array")
                    .add(content_ref);
            }
        };

        self
    }

    pub fn set_media_box(&mut self, rect: Rectangle) -> &mut Self {
        self.dictionary.insert("MediaBox", make_rectangle(rect));

        self
    }
}

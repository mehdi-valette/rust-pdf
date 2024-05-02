use crate::{
    document::Document,
    objects::{Dictionary, PdfArray, PdfName, PdfString, Stream},
    PdfElement,
};
use encoding::{all::WINDOWS_1252, EncoderTrap, Encoding};

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
            .insert("BaseFont", PdfName::new(font_family))
            .insert("Encoding", PdfName::new("Utf-8"));

        self.dictionary
            .get_mut("Resources")
            .expect("Resources don't exist")
            .as_any_mut()
            .downcast_mut::<Dictionary>()
            .expect("Resources isn't a dictionary")
            .insert(font_name, font);

        self
    }

    pub fn add_text(&mut self, position: (f32, f32), text: &str) -> &mut Self {
        let text_binary = PdfString::from_vec(
            WINDOWS_1252
                .encode(text, EncoderTrap::Ignore)
                .expect("cannot encode into win-1252"),
            crate::objects::PdfStringEncoding::Literal,
        );

        let mut data: Vec<u8> = Vec::new();
        data.extend(b"BT /F1 12 Tf ");
        data.extend(format!("{} {} Td", position.0, position.1).as_bytes());
        data.extend(text_binary.print());
        data.extend(b"Tj ET");

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

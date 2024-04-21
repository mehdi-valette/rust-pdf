use crate::{Dictionary, Number, PdfElement};

pub struct Stream {
    dictionary: Dictionary,
    data: Vec<u8>,
}

impl Stream {
    pub fn new(data: Vec<u8>) -> Self {
        let mut dictionary = Dictionary::new();
        dictionary.insert("Length", Number::new(data.len() as f32));

        Stream { dictionary, data }
    }
}

impl PdfElement for Stream {
    fn print(&self) -> Vec<u8> {
        let mut text: Vec<u8> = Vec::new();

        text.extend(self.dictionary.print());
        text.extend(b"\nstream");
        text.extend(self.data.iter());
        text.extend(b"\nendstream");

        text
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

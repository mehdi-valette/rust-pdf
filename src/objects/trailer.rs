use uuid::Uuid;

use crate::{Dictionary, Number, PdfArray, PdfElement, PdfString, PdfStringEncoding};

pub fn print_trailer<'a>(xref_offset: &u32, reference_count: &u32) -> Vec<u8> {
    let mut text: Vec<u8> = Vec::new();
    let mut dictionary = Dictionary::new();
    let mut id_array: Vec<Box<dyn PdfElement>> = Vec::new();

    let identifier = PdfString::new(Uuid::new_v4().to_string(), PdfStringEncoding::Hexadecimal);
    let update = PdfString::new(Uuid::new_v4().to_string(), PdfStringEncoding::Hexadecimal);

    id_array.push(Box::new(identifier));
    id_array.push(Box::new(update));

    dictionary.set("Size", Box::new(Number::new(*reference_count as f32)));
    dictionary.set("ID", Box::new(PdfArray::make(id_array)));

    text.extend(b"trailer");
    text.extend(dictionary.print().as_slice());
    text.extend(b"\nstartxref\n");
    text.extend(format!("{}\n", xref_offset).as_bytes());
    text.extend(b"%%EOF");

    text
}

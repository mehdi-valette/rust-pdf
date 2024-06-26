use crate::objects::{Dictionary, Number, PdfArray, PdfString, PdfStringEncoding, Reference};
use crate::PdfElement;
use uuid::Uuid;

pub fn print_trailer<'a>(
    xref_offset: &u32,
    reference_count: &u32,
    catalog_reference: &Reference,
) -> Vec<u8> {
    let mut text: Vec<u8> = Vec::new();
    let mut dictionary = Dictionary::new();

    let identifier =
        PdfString::from_string(Uuid::new_v4().to_string(), PdfStringEncoding::Hexadecimal);
    let update = PdfString::from_string(Uuid::new_v4().to_string(), PdfStringEncoding::Hexadecimal);

    dictionary
        .insert("Size", Number::new(*reference_count as f32))
        .insert("ID", PdfArray::make(vec![identifier, update]))
        .insert("Root", catalog_reference.clone());

    text.extend(b"trailer\n");
    text.extend(dictionary.print().as_slice());
    text.extend(b"\nstartxref\n");
    text.extend(format!("{}\n", xref_offset).as_bytes());
    text.extend(b"%%EOF");

    text
}

use rust_pdf::document::Document;
use rust_pdf::objects::{Dictionary, Number, PdfArray, PdfName, Stream};

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.pdf")?;

    let mut document = Document::new();

    let page_root_reference = document.get_page_root_reference();
    let text_ref = document.push(Box::new(Stream::new(
        b"BT /F1 18 Tf 0 0 Td (Hello World) TjET".into(),
    )));

    let mut page_size = PdfArray::new();
    page_size
        .add(Number::new(0f32))
        .add(Number::new(0f32))
        .add(Number::new(595f32))
        .add(Number::new(79f32));

    let mut font_f1_dictionary = Dictionary::new();
    font_f1_dictionary
        .insert("Type", PdfName::new("Font"))
        .insert("Subtype", PdfName::new("Type1"))
        .insert("BaseFont", PdfName::new("Times-Roman"));

    let mut font_dictionary = Dictionary::new();
    font_dictionary.insert("F1", font_f1_dictionary);

    let mut resource_dictionary = Dictionary::new();
    resource_dictionary.insert("Font", font_dictionary);

    let mut first_page = Dictionary::new();
    first_page
        .insert("Type", PdfName::new("Page"))
        .insert("MediaBox", page_size)
        .insert("Resources", Dictionary::new())
        .insert("Parent", page_root_reference)
        .insert("Contents", text_ref.get_reference())
        .insert("Resources", resource_dictionary);

    document.add_page(first_page);

    let text = document.print();

    file.write(&text)?;

    Ok(())
}

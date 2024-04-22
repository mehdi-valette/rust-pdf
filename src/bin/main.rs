use rust_pdf::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.txt")?;

    let mut document = Document::new();

    let mut page_size = PdfArray::new();
    page_size
        .add(Number::new(0f32))
        .add(Number::new(0f32))
        .add(Number::new(99f32))
        .add(Number::new(9f32));

    let mut first_page = Dictionary::new();
    first_page
        .insert("Type", PdfName::new("Page"))
        .insert("MediaBox", page_size)
        .insert("Resources", Dictionary::new())
        .insert("Parent", Reference::new());

    document.add_page(first_page);

    let text = document.print();

    file.write(&text)?;

    Ok(())
}

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

    let first_page_reference = document.push(Box::new(first_page)).get_reference();

    let pages_ref = document
        .get_object(document.get_catalog())
        .expect("document has no catalog")
        .get_content()
        .as_any()
        .downcast_ref::<Dictionary>()
        .expect("catalog doesn't container a dictionary")
        .get("Pages")
        .expect("catalog's dictionary doesn't contain an entry \"pages\"")
        .as_any()
        .downcast_ref::<Reference>()
        .expect("catalog's \"pages\" isn't a reference")
        .clone();

    document
        .get_object_mut(pages_ref)
        .expect("document has no page tree")
        .get_content_mut()
        .as_any_mut()
        .downcast_mut::<Dictionary>()
        .expect("pages doesn't contain a dictionary")
        .get_mut("Kids")
        .expect("pages' dictionary should have an entry \"Kids\"")
        .as_any_mut()
        .downcast_mut::<PdfArray>()
        .expect("pages' \"Kids\" entry isn't an array")
        .add(first_page_reference);

    let text = document.print();

    file.write(&text)?;

    Ok(())
}

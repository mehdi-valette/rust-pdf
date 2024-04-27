use rust_pdf::document::Document;
use rust_pdf::structures::Page;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.pdf")?;

    let mut document = Document::new();

    let mut page = Page::new(&mut document);
    page.add_font_type_1("F1", "Times-Roman")
        .add_content(b"BT /F1 18 Tf 10 300 Td (Hello World) TjET".into());

    let page_dict = page.get_dictionary();

    document.add_page(page_dict);

    let text = document.print();

    file.write(&text)?;

    Ok(())
}

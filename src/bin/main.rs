use rust_pdf::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.txt")?;

    let mut document = Document::new();

    document.push(Box::new(PdfString::new(
        "I am an object".to_string(),
        PdfStringEncoding::Literal,
    )));

    document.push(Box::new(PdfBoolean::new(true)));

    let text = document.print();

    file.write(&text)?;

    Ok(())
}

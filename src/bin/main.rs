use rust_pdf::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.txt")?;

    let header = Header {
        pdf_version: "2.0".into(),
    };

    let mut trailer = Trailer {
        dictionary: Dictionary::new(),
    };

    trailer
        .dictionary
        .set(
            "identifier",
            Box::new(Reference {
                identifier: 45,
                update: 10,
            }),
        )
        .set("booleanTrue", Box::new(PdfBoolean::new(true)))
        .set("booleanFalse", Box::new(PdfBoolean::new(false)))
        .set(
            "stringLiteral",
            Box::new(PdfString::new(
                "Hello World !<>()",
                PdfStringEncoding::Literal,
            )),
        )
        .set(
            "stringHexa",
            Box::new(PdfString::new(
                "Hello World !<>()",
                PdfStringEncoding::Hexadecimal,
            )),
        );

    let document = Document { header, trailer };

    let text = &document.print();

    file.write(text)?;

    Ok(())
}

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

    let mut body: Vec<IndirectObject> = Vec::new();

    body.push(IndirectObject::new(
        1,
        0,
        Box::new(PdfString::new("I am an object", PdfStringEncoding::Literal)),
    ));

    body.push(IndirectObject::new(2, 0, Box::new(PdfBoolean::new(true))));

    trailer
        .dictionary
        .set(
            "identifier",
            Box::new(Reference {
                object_number: 45,
                genration_number: 10,
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

    let mut document = Document {
        header,
        body,
        trailer,
    };

    let text = document.print();

    file.write(&text)?;

    Ok(())
}

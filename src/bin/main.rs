use rust_pdf::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.txt")?;

    let header = Header {
        pdf_version: "2.0".into(),
    };

    let mut trailer = Trailer {
        dictionary: Dictionary {
            dict: HashMap::new(),
        },
    };

    trailer.dictionary.dict.insert(
        Name {
            name: "Test".into(),
        },
        Box::new(Reference {
            identifier: 45,
            update: 10,
        }),
    );

    trailer.dictionary.dict.insert(
        Name { name: "M".into() },
        Box::new(Reference {
            identifier: 12,
            update: 20,
        }),
    );

    let document = Document { header, trailer };

    let text = &document.print();

    file.write(text)?;

    Ok(())
}

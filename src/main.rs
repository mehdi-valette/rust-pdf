use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

trait PdfElement {
    fn print(&self) -> Vec<u8>;
}

struct Document {
    header: Header,
    trailer: Trailer,
}

impl PdfElement for Document {
    fn print(&self) -> Vec<u8> {
        let mut document = Vec::new();
        document.append(&mut self.header.print());
        document.append(&mut "\n\n".as_bytes().to_vec());
        document.append(&mut self.trailer.print());

        document
    }
}

struct Header {
    pdf_version: String,
}

impl PdfElement for Header {
    fn print(&self) -> Vec<u8> {
        format!("%{}", self.pdf_version).as_bytes().to_vec()
    }
}

struct Trailer {
    dictionary: Dictionary,
}

impl PdfElement for Trailer {
    fn print(&self) -> Vec<u8> {
        let mut trailer = "trailer\n".to_string().as_bytes().to_vec();
        trailer.append(&mut self.dictionary.print());
        trailer
    }
}

struct Dictionary {
    dict: HashMap<String, Box<dyn PdfElement>>,
}

impl PdfElement for Dictionary {
    fn print(&self) -> Vec<u8> {
        let mut entries: Vec<u8> = Vec::new();
        entries.append(&mut "<<".to_string().as_bytes().to_vec());

        for (key, value) in &(self.dict) {
            entries.append(&mut format!("\n/{} ", key).as_bytes().to_vec());
            entries.append(&mut value.print());
        }

        entries.append(&mut "\n>>".as_bytes().to_vec());

        return entries;
    }
}

struct Reference {
    identifier: u32,
    update: u32,
}

impl PdfElement for Reference {
    fn print(&self) -> Vec<u8> {
        format!("{} {} R", self.identifier, self.update)
            .as_bytes()
            .to_vec()
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.txt")?;

    let header = Header {
        pdf_version: "PDF-2.0".to_string(),
    };

    let mut trailer = Trailer {
        dictionary: Dictionary {
            dict: HashMap::new(),
        },
    };

    trailer.dictionary.dict.insert(
        "Test".to_string(),
        Box::new(Reference {
            identifier: 45,
            update: 10,
        }),
    );

    trailer.dictionary.dict.insert(
        "M".to_string(),
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

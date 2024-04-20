mod objects;
mod reference_table;

pub use objects::{
    print_trailer, Dictionary, Document, Header, IndirectObject, Number, PdfArray, PdfBoolean,
    PdfName, PdfString, PdfStringEncoding, Reference,
};

pub trait PdfElement {
    fn print(&self) -> Vec<u8>;
}

mod objects;
mod reference_table;

use std::any::Any;

pub use objects::{
    print_trailer, Dictionary, Document, Header, IndirectObject, Number, PdfArray, PdfBoolean,
    PdfName, PdfString, PdfStringEncoding, Reference, Stream,
};

pub trait PdfElement {
    fn print(&self) -> Vec<u8>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

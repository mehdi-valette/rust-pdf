pub mod document;
pub mod objects;
pub mod structures;

use std::any::Any;

pub trait PdfElement {
    fn print(&self) -> Vec<u8>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

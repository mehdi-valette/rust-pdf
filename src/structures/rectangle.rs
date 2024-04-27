use crate::objects::{Number, PdfArray};

pub type Rectangle = [f32; 4];

pub fn make_rectangle(arr: [f32; 4]) -> PdfArray {
    PdfArray::make(
        [
            Number::new(arr[0]),
            Number::new(arr[1]),
            Number::new(arr[2]),
            Number::new(arr[3]),
        ]
        .into(),
    )
}

use crate::objects::IndirectObject;
use crate::PdfElement;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Reference {
    object_number: u32,
    generation_number: u32,
}

impl Reference {
    pub fn new() -> Self {
        Reference {
            object_number: 0,
            generation_number: 0,
        }
    }

    pub fn make(object: &IndirectObject) -> Self {
        Reference {
            object_number: object.get_object_number().clone(),
            generation_number: object.get_generation_number().clone(),
        }
    }

    pub fn clone(&self) -> Self {
        Reference {
            object_number: self.object_number,
            generation_number: self.generation_number,
        }
    }
}

impl PdfElement for Reference {
    fn print(&self) -> Vec<u8> {
        format!("{} {} R", self.object_number, self.generation_number)
            .as_bytes()
            .to_vec()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// impl Ord for Reference {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match self.object_number.cmp(&other.object_number) {
//             Ordering::Equal => self.generation_number.cmp(&other.generation_number),
//             non_equal => non_equal,
//         }
//     }
// }

// impl PartialEq for Reference {
//     fn eq(&self, other: &Self) -> bool {
//         self.generation_number == other.generation_number
//             && self.object_number == other.object_number
//     }
// }

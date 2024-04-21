use crate::objects::IndirectObject;

pub fn print(body: &Vec<&IndirectObject>) -> Vec<u8> {
    let mut reference_table: Vec<u8> = Vec::new();

    reference_table.extend(b"xref\n");
    reference_table.extend(format!("{} 0\n", body.len()).as_bytes());
    reference_table.extend(b"0000000000 65535 f\n");

    for obj in body.iter() {
        reference_table.extend(byte_offset(obj.get_offset()));
        reference_table.push(b' ');
        reference_table.extend(generation_number(obj.get_generation_number()));
        reference_table.extend(b" n\n");
    }

    reference_table
}

fn byte_offset(number: &u32) -> [u8; 10] {
    let mut bytes: [u8; 10] = [b'0'; 10];

    let number_str = format!("{}", number);
    let number_len = number_str.len();
    let zeroes_count = 10 - number_len;

    for (index, character) in number_str.chars().enumerate() {
        bytes[zeroes_count + index] = character as u8;
    }

    bytes
}

fn generation_number(number: &u32) -> [u8; 5] {
    let mut bytes: [u8; 5] = [b'0'; 5];

    let number_str = format!("{}", number);
    let number_len = number_str.len();
    let zeroes_count = 5 - number_len;

    for (index, character) in number_str.chars().enumerate() {
        bytes[zeroes_count + index] = character as u8;
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_offset() {
        assert_eq!(&byte_offset(&0), b"0000000000");
        assert_eq!(&byte_offset(&5), b"0000000005");
        assert_eq!(&byte_offset(&25), b"0000000025");
        assert_eq!(&byte_offset(&63225), b"0000063225");
        assert_eq!(&byte_offset(&4294967295), b"4294967295");
    }

    #[test]
    fn test_generation_offset() {
        assert_eq!(&generation_number(&0), b"00000");
        assert_eq!(&generation_number(&5), b"00005");
        assert_eq!(&generation_number(&25), b"00025");
        assert_eq!(&generation_number(&63225), b"63225");
    }
}

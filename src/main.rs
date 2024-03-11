use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("test.txt")?;
    file.write(b"Hello, world!")?;
    Ok(())
}

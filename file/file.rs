use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

fn main() -> io::Result<()> {
    let mut f = File::open("file.rs")?;
    let mut buf=[0u8;12];

    // move the cursor 42 bytes from the start of the file
    f.seek(SeekFrom::Start(42))?;
    f.read(&mut buf).unwrap();
    println!("{:?}",buf);
    Ok(())
}

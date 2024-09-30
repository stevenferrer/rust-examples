use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::string::String;

fn main() -> io::Result<()> {
    let f = File::open("testdata/lorem.txt")?;

    let mut reader = BufReader::new(f);

    reader.seek(SeekFrom::Start(1))?;
    reader.seek(SeekFrom::Current(5))?;

    let mut buf = [0; 5];
    reader.read_exact(&mut buf)?;

    let content = String::from_utf8(buf.to_vec()).expect("conversion ok");
    println!("\"{}\"", content);

    Ok(())
}

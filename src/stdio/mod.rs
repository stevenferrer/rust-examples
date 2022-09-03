#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::io::SeekFrom;

    #[test]
    fn read_and_write() {
        let mut f = File::open("testdata/lorem.txt").unwrap();
        let mut buffer = [0; 10];

        // read up to 10 bytes
        let n = f.read(&mut buffer).unwrap();

        println!("The bytes: {:?}", &buffer[..n]);
    }

    #[test]
    fn seek_and_bufread() {
        let mut f = File::open("testdata/lorem.txt").unwrap();
        let mut buffer = [0; 10];

        // skip to the last 10 bytes of the file
        f.seek(SeekFrom::End(-10)).unwrap();

        // read up to 10 bytes
        let n = f.read(&mut buffer).unwrap();

        println!("The bytes: {:?}", &buffer[..n]);
    }

    #[test]
    fn bufread() {
        let f = File::open("testdata/lorem.txt").unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();

        // read a line into buffer
        reader.read_line(&mut buffer).unwrap();

        println!("{buffer}");
    }
}

use std::fs::{
    File,
};
use std::io::{
    BufRead,
    BufReader,
    Lines,
    stdin,
};


pub struct FileIterator<'b> {
    buffer: Lines<BufReader<&'b File>>
}

impl<'b> FileIterator<'b> {
    pub fn new(file: &'b File) -> FileIterator<'b> {
        let buffer = BufReader::new(file).lines();
        FileIterator {
            buffer: buffer
        }
    }
}

impl<'b> Iterator for FileIterator<'b> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.buffer.next() {
            return Some(line.unwrap())
        }

        None
    }
}


pub struct StdinIterator {}

impl StdinIterator {
    pub fn new() -> StdinIterator {
        StdinIterator {}
    }
}

impl Iterator for StdinIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        let data = stdin().read_line(&mut buffer).unwrap();

        if data > 0 {
            return Some(buffer)
        }

        None
    }
}

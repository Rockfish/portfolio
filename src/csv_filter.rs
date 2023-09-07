use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::{fs, io};

pub struct CsvFilter {
    lines: Lines<BufReader<File>>,
}

impl CsvFilter {
    pub fn new(reader: io::BufReader<fs::File>) -> Self {
        CsvFilter {
            lines: reader.lines(),
        }
    }
}

impl io::Read for CsvFilter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let line = self.lines.next();
            match line {
                None => return Ok(0),
                Some(result) => match result {
                    Err(e) => return Err(e),
                    Ok(line) => {
                        // good lines have commas
                        if line.contains(",") {
                            let temp = line.as_bytes();
                            let size = temp.len();
                            buf[..size].copy_from_slice(&temp[..size]);
                            buf[size] = b'\n';
                            return Ok(size + 1);
                        }
                    }
                },
            }
        }
    }
}

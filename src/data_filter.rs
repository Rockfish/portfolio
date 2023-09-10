use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::{fs, io};

pub struct DataFilter {
    lines: Lines<BufReader<File>>,
}

impl DataFilter {
    pub fn new(reader: io::BufReader<fs::File>) -> Self {
        DataFilter { lines: reader.lines() }
    }
}

// Fidelity Data has miscellaneous lines that need to be ignored.
// good lines have commas and don't start with a quote
impl io::Read for DataFilter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let line = self.lines.next();
            match line {
                None => return Ok(0),
                Some(result) => match result {
                    Err(e) => return Err(e),
                    Ok(line) => {
                        // good lines have commas and don't start with a quote
                        if line.contains(',') && !line.starts_with('"') {
                            // println!("{line}");
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

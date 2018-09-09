use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::str;

struct Line {
    offset: u64,
    text: Option<String>,
}

impl Line {
    fn new(offset: u64) -> Line {
        Line { offset, text: None }
    }
    fn set_text(&mut self, bytes: &[u8]) {
        self.text = Some(str::from_utf8(bytes).unwrap().to_string())
    }
}

/// A `Larry` is a "line array". It allows one to access a file as a
/// lazily-read array of lines. This allows efficient random access to large
/// files such as log files.
pub struct Larry {
    lines: Vec<Line>,
    pub file: File,
    length: u64,
}

impl Larry {
    /// Constructs a new `Larry`.
    ///
    /// Construction requires that the file be scanned for line-terminal byte
    /// sequences.
    ///
    /// # Examples
    /// ```
    /// use larry::Larry;
    /// use std::path::Path;
    ///
    /// let mut larry = Larry::new(Path::new("production.log"));
    /// ```
    /// # Errors
    /// Any `std::io::Error` arising while opening the file and scanning its contents
    /// for line endings will be returned.
    pub fn new(path: &Path) -> Result<Larry, std::io::Error> {
        match File::open(&path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut length = 0;
                let mut offset = 0;
                let mut last_was_0a = false;
                let mut last_was_0d = false;
                let mut lines = Vec::new();
                loop {
                    let length = {
                        match reader.fill_buf() {
                            Ok(buffer) => {
                                if buffer.len() == 0 {
                                    break;
                                }
                                for i in 0..buffer.len() {
                                    length += 1;
                                    match buffer[i] {
                                        0x0A => {
                                            if last_was_0d {
                                                last_was_0a = false;
                                                last_was_0d = false;
                                                lines.push(Line::new(offset));
                                                offset += length;
                                                length = 0;
                                            } else {
                                                if last_was_0a {
                                                    lines.push(Line::new(offset));
                                                    offset += length - 1;
                                                    length = 1;
                                                } else {
                                                    last_was_0a = true;
                                                }
                                            }
                                        }
                                        0x0D => {
                                            if last_was_0a {
                                                last_was_0a = false;
                                                last_was_0d = false;
                                                lines.push(Line::new(offset));
                                                offset += length;
                                                length = 0;
                                            } else {
                                                if last_was_0d {
                                                    lines.push(Line::new(offset));
                                                    offset += length - 1;
                                                    length = 1;
                                                } else {
                                                    last_was_0d = true;
                                                }
                                            }
                                        }
                                        _ => {
                                            if last_was_0a || last_was_0d {
                                                last_was_0a = false;
                                                last_was_0d = false;
                                                length -= 1;
                                                lines.push(Line::new(offset));
                                                offset += length;
                                                length = 1;
                                            }
                                        }
                                    }
                                }
                                buffer.len()
                            }
                            Err(io_err) => {
                                return Err(io_err);
                            }
                        }
                    };
                    reader.consume(length);
                }
                if length > 0 {
                    lines.push(Line::new(offset));
                    offset += length;
                }
                Ok(Larry {
                    lines: lines,
                    file: File::open(&path).unwrap(),
                    length: offset,
                })
            }
            Err(error) => Err(error),
        }
    }
    /// Obtain a particular line.
    ///
    /// # Examples
    /// ```
    /// use larry::Larry;
    /// use std::path::Path;
    ///
    /// let mut larry = Larry::new(Path::new("production.log"));
    /// print!("{}", larry.get(larry.len() - 1).unwrap()); // print the last line of the file
    /// ```
    /// # Errors
    /// Index bound errors if you ask for a line beyond the end of the file and
    /// IO errors if the file has changed since the larry was created.
    pub fn get(&mut self, i: usize) -> Result<String, String> {
        if i >= self.lines.len() {
            Err(format!(
                "index {} in file of only {} lines",
                i,
                self.lines.len()
            ))
        } else {
            if self.lines[i].text.is_some() {
                Ok(self.lines[i].text.clone().unwrap())
            } else {
                let length = if i == self.lines.len() - 1 {
                    self.length - self.lines[i].offset
                } else {
                    self.lines[i + 1].offset - self.lines[i].offset
                };
                let line = &mut self.lines[i];
                let mut buffer = vec![0; length as usize];
                if let Err(io_err) = self.file.seek(SeekFrom::Start(line.offset)) {
                    Err(io_err.to_string())
                } else if let Err(io_err) = self.file.read(&mut buffer) {
                    Err(io_err.to_string())
                } else {
                    line.set_text(&buffer);
                    Ok(line.text.clone().unwrap())
                }
            }
        }
    }
    /// Returns number of lines.
    ///
    /// # Examples
    /// ```
    /// use larry::Larry;
    /// use std::path::Path;
    ///
    /// let mut larry = Larry::new(Path::new("production.log"));
    /// println!("number of lines line: {}", larry.len());
    /// ```
    pub fn len(&self) -> usize {
        self.lines.len()
    }
}

/* ----------------------------------------------------------------------------

    MIT License

    Copyright (c) 2024 MW

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

---------------------------------------------------------------------------- */

use std::{collections::HashMap};
use std::fs::{File};
use std::io::{BufRead, BufReader};

use crate::env::{exit_with_error, OptionUnwrapExit};


pub struct FileParser {
    file_reader: BufReader<File>,
    path: String,
}

impl FileParser {
    pub fn init(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;

        Ok(Self {
            file_reader: BufReader::new(file),
            path: String::from(path)
        })
    }

    pub fn iter(&mut self) -> FileIterator {
        FileIterator {
            file_reader: &mut self.file_reader,
            path: self.path.as_str(),
            line_number: 0,
            symbols: HashMap::new()
        }
    }
}

pub struct FileIterator<'a> {
    file_reader: &'a mut BufReader<File>,
    path: &'a str,
    line_number: u32,
    symbols: HashMap<String, String>
}

impl<'a> FileIterator<'a> {
    fn read_line(&mut self) -> Option<String> {
        let mut buffer = String::with_capacity(80);
        
        while let Ok(count) = self.file_reader.read_line(&mut buffer) {
            self.line_number += 1;
            if count == 0 {
                return None;
            }
        
            let line = String::from(buffer.trim());
            buffer.clear();
            
            if (line.len() == 0) || (line.starts_with('#')) {
                continue;
            }

            return Some(line);
        }

        None
    }
    
    fn parse_pair(pair: &str) -> Option<(usize, usize)> {
        if let Some((x_str, y_str)) = pair.split_once(',') {
            if let Ok(x) = x_str.trim().parse::<usize>() {
                if let Ok(y) = y_str.trim().parse::<usize>() {
                    return Some((x, y));
                }
            }
        }

        None
    }
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.read_line()?;
        
        let first_char = line.chars().nth(0).unwrap_or('~');
        if first_char.is_numeric() {
            return Some(Self::parse_pair(&line).unwrap_or_exit(format!("error: cannot parse '{}' as a coordinate pair, at line {} of file '{}'", line, self.line_number, self.path)));
        }
        else {
            exit_with_error(format!("error: unrecognised character '{}', at line {} of file '{}'", first_char, self.line_number, self.path));
            None
        }
    }
}

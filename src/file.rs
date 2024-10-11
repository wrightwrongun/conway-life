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

use std::marker::PhantomData;
use std::{collections::HashMap};
use std::fs::{read, File};
use std::io::{BufRead, BufReader, Read};

use crate::env::{exit_with_error, OptionUnwrapDisplay, OptionUnwrapExit};


pub type BufferType = u8;
pub type BufferSlice<'t> = &'t [BufferType];

/// Simplifies reading from a file or a string.
/// 
/// 
pub struct ReadBuffer<'a> {
    buffer: Vec<u8>,
    _nothing: BufferSlice<'a>   // Unused - here to satisfy the 'a constraint!
}

impl<'a> ReadBuffer<'a> {
    /// Initialises the buffer with the contents of a file.
    pub fn from_path(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;

        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer);
        
        Ok(Self {
            buffer,
            _nothing: &[0]
        })
    }
    
    /// Initialises the buffer with the contents of a string.
    pub fn from_string(text: &str) -> Self {
        Self {
            buffer: Vec::from(text),
            _nothing: &[0]
        }
    }

    /// Gives a reference to the contents of the buffer.
    pub fn as_vec(&'a self) -> &'a Vec<u8> {
        &self.buffer
    }

    /// Gives a reference to the contents of the buffer.
    pub fn as_slice(&'a self) -> BufferSlice<'a> {
        self.buffer.as_slice()
    }

    /// Gives a reader for the contents of the buffer.
    pub fn reader(&'a self) -> BufReader<BufferSlice<'a>> {
        BufReader::new(self.as_slice())
    }
}


//---------------------------------------------------------------------------//


/// Parses the contents of a 'life' file.
/// 
/// A 'life' file contains grid & cell info to populate a grid for Conway's
/// Game of Life.
/// 
/// The consumer will see a collection of parsed tuples, the first of which
/// is the width & height of the grid, and the remainder the coordinates of
/// all of the grid's live cells.
pub struct FileParser<'a> {
    buffer: ReadBuffer<'a>,
    path: Option<String>,
}

impl<'a> FileParser<'a> {
    /// Initialise the parser from the contents of a string.
    pub fn from_string(contents: &str) -> Self {
        Self::init(ReadBuffer::from_string(contents), None)
    }

    /// Initialise the parser from the contents of a file.
    pub fn from_path(path: &str) -> std::io::Result<Self> {
        Ok(Self::init(ReadBuffer::from_path(path)?, Some(String::from(path))))
    }

    /// Initialise the parser with a pre-initialised buffer.
    pub fn init(buffer: ReadBuffer<'a>, path: Option<String>) -> Self {
        Self {
            buffer,
            path
        }
    }

    /// Gives an iterator over the parsed contents of the file.
    pub fn iter(&mut self) -> FileIterator {
        FileIterator {
            reader: self.buffer.reader(),
            path: self.path.clone(),
            line_number: 0,
            symbols: HashMap::new()
        }
    }

    /// Gives a reference to the internal buffer.
    pub fn get_buffer(&'a self) -> &ReadBuffer<'a> {
        &self.buffer
    }

    /// Gives the path, if any.
    pub fn get_path(&self) -> Option<String> {
        self.path.clone()
    }
}

/// Iterates over the parsed contents of a 'life' file.
pub struct FileIterator<'a> {
    reader: BufReader<BufferSlice<'a>>,
    path: Option<String>,
    line_number: u32,
    symbols: HashMap<String, String>
}

impl<'a> FileIterator<'a> {
    /// Reads a line, ignoring comments and whitespace.
    fn read_line(&mut self) -> Option<String> {
        let mut buffer = String::with_capacity(80);
        
        while let Ok(count) = self.reader.read_line(&mut buffer) {
            self.line_number += 1;
            if count == 0 {
                return None;
            }
        
            let line = String::from(buffer.trim());
            buffer.clear();
            
            if line.is_empty() || (line.starts_with('#')) {
                continue;
            }

            return Some(line);
        }

        None
    }
    
    /// Converts a string containing a numeric pair into a tuple
    /// of the pair's values.
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

    /// Gives the next parsed tuple.
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.read_line()?;
        
        let first_char = line.chars().nth(0).unwrap_or('~');
        if first_char.is_numeric() {
            Some(Self::parse_pair(&line).unwrap_or_exit(format!("error: cannot parse '{}' as a coordinate pair, at line {} of file '{}'", line, self.line_number, self.path.unwrap_display_or("*unknown*"))))
        }
        else {
            exit_with_error(format!("error: unrecognised character '{}', at line {} of file '{}'", first_char, self.line_number, self.path.unwrap_display_or("*unknown*")));
            None
        }
    }
}

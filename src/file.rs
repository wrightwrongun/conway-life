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

use std::collections::hash_map::Entry;
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


/// The cell coordinates are represented by a tuple.
type CellCoords = (usize, usize);

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
    allow_fatal: bool       // <--- Flag (normally set to true) to allow a
                            //      parsing error to cause a 'clean' exit.
                            //      If this is set to false, instead of an
                            //      exit, there is a panic. This allows tests
                            //      to behave correctly. [See 'set_test()'.]
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
            path,
            allow_fatal: true
        }
    }

    /// Gives an iterator over the parsed contents of the file.
    pub fn iter(&mut self) -> FileIterator {
        FileIterator {
            reader: self.buffer.reader(),
            path: self.path.clone(),
            line_number: 0,
            grid_dimensions: None,
            allow_fatal: self.allow_fatal,
            state: ParserState {
                symbols: HashMap::new(),
                symbol_name: None,
                buffered_cells: Vec::new()
            }
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

    // Sets a flag to cause a panic instead of a 'clean' exit
    // when 'FileIterator.fatal_error()' is called.
    pub fn set_test(&mut self) {
        self.allow_fatal = false;
    }
}

/// Iterates over the parsed contents of a 'life' file.
pub struct FileIterator<'a> {
    reader: BufReader<BufferSlice<'a>>,
    path: Option<String>,
    line_number: u32,
    grid_dimensions: Option<CellCoords>,
    allow_fatal: bool,
    state: ParserState
}

struct ParserState {
    symbols: HashMap<String, SymbolDefinition>,
    buffered_cells: Vec<CellCoords>,
    symbol_name: Option<String>
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

    fn fatal_error(&self, message: String) {
        let message = format!("error: {}, at line {} of file '{}'", message, self.line_number, self.path.unwrap_display_or("*unknown*"));
        
        if self.allow_fatal {
            exit_with_error(message);
        }
        else {
            panic!("{}", message);
        }
    }
    
    /// Converts a string containing a numeric pair into a tuple
    /// of the pair's values.
    fn parse_pair(pair: &str) -> Option<CellCoords> {
        if let Some((x_str, y_str)) = pair.split_once(',') {
            if let Ok(x) = x_str.trim().parse::<usize>() {
                if let Ok(y) = y_str.trim().parse::<usize>() {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn parse_symbol_define_start(&mut self, line: String) {
        let name = line[1..].trim();
        if name.is_empty() {
            self.fatal_error("symbol has no name".to_string());
        }
        
        if self.state.symbol_name.is_some() {
            self.fatal_error(format!("nested symbol definition '{}' is not supported", name));
        }

        let symbol = SymbolDefinition::new(name);

        if let Entry::Vacant(e) = self.state.symbols.entry(String::from(name)) {
            e.insert(symbol);
        }
        else {
            self.state.symbols.entry(String::from(name)).or_insert(symbol);
        }

        self.state.symbol_name = Some(String::from(name));
    }

    fn parse_symbol_define_end(&mut self) {
        if self.state.symbol_name.is_some() {
            self.state.symbol_name = None;
        }
        else {
            self.fatal_error("unexpected end of symbol definition".to_string());
        }
    }

    fn parse_symbol_use(&mut self, line: String) -> Option<CellCoords> {
        if let Some((name, cell)) = line.split_once(' ') {
            let name = name.trim();
            let offset = Self::parse_pair(cell).unwrap_or_exit(format!("error: cannot parse '{}' as a coordinate pair, at line {} of file '{}'", cell, self.line_number, self.path.unwrap_display_or("*unknown*")));
        
            // Can't use a symbol before specifying grid dimensions...
            if self.grid_dimensions.is_none() {
                self.fatal_error(format!("use of symbol '{}' before grid-size", name));
            }

            if let Some(symbol) = self.state.symbols.get(&String::from(name)) {
                let mut cells: Vec<CellCoords> = symbol.cells
                                                    .iter()
                                                    .map(|c| (c.0 + offset.0, c.1 + offset.1))
                                                    .collect();

                match self.state.symbol_name {
                    Some(ref name) => {
                        let symbol_entry = self.state.symbols.entry(name.clone()).and_modify(|symbol| symbol.push_cells(&cells));
                    },
                    None => {
                        self.state.buffered_cells.append(&mut cells);

                        return self.state.buffered_cells.pop();
                    }
                }
            }
            else {
                self.fatal_error(format!("unknown symbol '{}'", name));
            }
        }
        else {
            self.fatal_error(format!("bad symbol '{}'", line));
        }

        None
    }

    fn parse_cell(&mut self, line: String) -> Option<CellCoords> {
        let cell = Self::parse_pair(&line).unwrap_or_exit(format!("error: cannot parse '{}' as a coordinate pair, at line {} of file '{}'", line, self.line_number, self.path.unwrap_display_or("*unknown*")));
        match self.state.symbol_name {
            Some(ref name) => {
                self.state.symbols.entry(name.clone()).and_modify(|symbol| symbol.push_cell(cell));
                None
            },
            None => {
                // Has a grid-size been given (grid-size is the first pair)
                // in the file to be parsed...
                if self.grid_dimensions.is_some() {
                    self.validate_cell(cell.0, cell.1);
                }
                else {
                    self.grid_dimensions = Some(cell);
                }
                
                Some(cell)
            }
        }
    }

    fn validate_cell(&self, x: usize, y: usize) {
        if let Some((width, height)) = self.grid_dimensions {
            if (x >= width) || (y >= height) {
                self.fatal_error(format!("cell location ({},{}) out of bounds ({},{})", x, y, width, height));
            }
        }
        else {
            // This should never be reached - but just in case...
            self.fatal_error("grid size has not been set".to_string());
        }
    }
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = (usize, usize);

    /// Gives the next parsed tuple.
    fn next(&mut self) -> Option<Self::Item> {
        // Firstly return cells that have been provided by use of
        // symbols...
        if !self.state.buffered_cells.is_empty() {
            if let Some((x, y)) = self.state.buffered_cells.pop() {
                self.validate_cell(x, y);
                return Some((x, y));
            }
        }

        // Loop through the lines until a cell can be returned...
        while let Some(line) = self.read_line() {
            let first_char = line.chars().nth(0).unwrap_or('~');
            if first_char.is_numeric() {
                // Cell...
                if let Some(cell) = self.parse_cell(line) {
                    return Some(cell);
                }
            }
            else if first_char == ':' {
                // Start of symbol definition...
                self.parse_symbol_define_start(line);
            }
            else if first_char == ';' {
                // End of symbol definition...
                self.parse_symbol_define_end();
            }
            else if first_char.is_alphanumeric() {
                // Symbol use...
                if let Some(cell) = self.parse_symbol_use(line) {
                    return Some(cell);
                }
            }
            else {
                self.fatal_error(format!("unrecognised character '{}'", first_char));
            }
        }

        None
    }
}


//---------------------------------------------------------------------------//


struct SymbolDefinition {
    name: String,
    cells: Vec<CellCoords>
}

impl SymbolDefinition {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            cells: Vec::new()
        }
    }

    fn push_cell(&mut self, cell: CellCoords) {
        self.cells.push(cell);
    }

    fn push_cells(&mut self, cells: &[CellCoords]) {
        for cell in cells {
            self.cells.push(*cell);
        }
    }    

    fn iter(&self) -> impl Iterator + '_ {
        self.cells.iter()
    }
}
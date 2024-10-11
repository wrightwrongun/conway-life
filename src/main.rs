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

#![allow(dead_code, unused)]

mod env;
mod file;
mod life;
mod grid;
mod tests;

use std::io::{BufRead, IsTerminal};

use env::{ArgsHelper, OptionUnwrapExit, ResultUnwrapExit};
use file::FileParser;
use life::{LifeCell, LifeGrid};
use grid::{Grid, GridCell, SimpleGrid};


fn main() {
    // Expect 2 command-line arguments (excluding options) - so exit the
    // program if the incorrect number of arguments are found...
    let args = ArgsHelper::expect(2, "expected [-v] <input-file-path> <iterations>");

    // Assign the given command-line arguments...
    let path = &args[0];
    let is_debug = args.has_option("-d");
    let is_verbose = args.has_option("-v");
    let cycles = args[1].parse::<usize>().unwrap_or_exit(format!("error: argument '{}' is not a valid iteration value", args[1]));

    // Open the file containing the grid/cell info...
    let mut parser = FileParser::from_path(path.as_str()).unwrap_or_exit(format!("error: cannot open file '{}'", path));
    let mut cells = parser.iter();
    
    // Create an empty 'life' grid with the dimensions given in the file...
    let (width, height) = cells.next().unwrap_or_exit(format!("error: cannot find width+height from file '{}'", path));
    let mut life_grid = SimpleGrid::init_life(width, height);

    // Loop through the cell info given in the file, setting a grid-cell to
    // 'live' for each cell...
    for (x, y) in cells {
        life_grid.set_live(x, y);
    }

    // Print the starting grid...
    println!("Starting:");
    life_grid.write(&mut std::io::stdout());
    
    // Iterate for the given number of cycles...
    for count in 1..=cycles {
        let mut neighbours_grid = SimpleGrid::init(width, height, 0);
        for cell in &life_grid {
            neighbours_grid.set(cell.get_x(), cell.get_y(), cell.count_neighbours());
        }

        let mut new_grid = SimpleGrid::init_life(width, height);
        for cell in &life_grid {
            let neighbour_count = neighbours_grid[(cell.get_x(), cell.get_y())];
            
            if (cell.is_live()) {
                if [2, 3].contains(&neighbour_count) {
                    new_grid.set_live(cell.get_x(), cell.get_y());
                }
            }
            else if neighbour_count == 3 {
                new_grid.set_live(cell.get_x(), cell.get_y());
            }
        }        
    
        // DEBUG: Print the neighbour-count grid...
        if is_debug {
            neighbours_grid.write(&mut std::io::stdout());
        }

        if is_verbose {
            println!("iteration: {}", count);
            new_grid.write(&mut std::io::stdout());
        }

        // Swap to the new grid...
        life_grid = new_grid;
    }

    // Print the ending grid...
    println!("Final iteration {}:", cycles);
    life_grid.write(&mut std::io::stdout());
}

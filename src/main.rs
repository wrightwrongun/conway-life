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

mod file;
mod life;
mod grid;

use file::FileParser;
use life::{LifeCell, LifeGrid};
use grid::{Grid, SimpleGrid};


fn main() {
    let path = "example/simple_grid.life";

    if let Ok(mut parser) = FileParser::init(path) {
        let mut cells = parser.iter();
        
        if let Some((width, height)) = cells.next() { 
            let mut grid = SimpleGrid::init_life(width, height);

            for (x, y) in cells {
                grid.set_on(x, y);
            }

            grid.write(&mut std::io::stdout());

            let mut output = SimpleGrid::init(20, 12, 0);

            for cell in &grid {
                output.set(cell.get_x(), cell.get_y(), cell.count_neighbours());
            }
        
            output.write(&mut std::io::stdout());
        }
        else {
            panic!("cannot find width+height from file '{}'", path);
        }
    }
    else {
        panic!("cannot open file '{}'", path);
    }
}

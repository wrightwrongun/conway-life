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
mod grid;

use grid::{Grid, GridCell, SimpleGrid, SizedGrid, SparseGrid};


type LifeCellType = char;
type LifeGridType = SimpleGrid<LifeCellType>;

trait LifeGrid {
    const DEAD_CELL: LifeCellType = ' ';
    const LIVE_CELL: LifeCellType = '*';
    
    fn init_life(width: usize, height: usize) -> Self;

    fn set_on(&mut self, x: usize, y: usize);
    
    fn set_off(&mut self, x: usize, y: usize);
}

impl LifeGrid for SimpleGrid<LifeCellType> {
    fn init_life(width: usize, height: usize) -> Self {
        Self::init(width, height, <LifeGridType as LifeGrid>::DEAD_CELL)
    }
    
    fn set_on(&mut self, x: usize, y: usize) {
        self.set(x, y, <LifeGridType as LifeGrid>::LIVE_CELL);
    }
    
    fn set_off(&mut self, x: usize, y: usize) {
        self.set(x, y, <LifeGridType as LifeGrid>::DEAD_CELL);
    }
}

trait LifeCell {
    fn is_live(&self) -> bool;

    fn is_dead(&self) -> bool;

    fn count_neighbours(&self) -> i16;
}

impl<'a> LifeCell for GridCell<'a, SimpleGrid<LifeCellType>> {
    fn is_live(&self) -> bool {
        self.get() == &<LifeGridType as LifeGrid>::LIVE_CELL
    }

    fn is_dead(&self) -> bool {
        self.get() == &<LifeGridType as LifeGrid>::DEAD_CELL
    }
        
    fn count_neighbours(&self) -> i16 {
        let adjust = |ax: isize, ay: isize| -> i16 {
            match self.get_relative(ax, ay) {
                Some(x) if x == &<LifeGridType as LifeGrid>::LIVE_CELL => 1,
                Some(_) => 0,
                None => 0
            }
        };

        adjust(-1, -1) + adjust(0, -1) + adjust(1, -1)
            + adjust(-1, 0) + adjust(1, 0)
            + adjust(-1, 1) + adjust(0, 1) + adjust(1, 1)
    }
}


fn main() {
    let mut grid = SimpleGrid::init_life(20, 12);

    grid.set_on(10, 5);
    grid.set_on(11, 5);
    grid.set_on(12, 5);

    grid.write(&mut std::io::stdout());

    println!("-------------");

    let mut output = SimpleGrid::init(20, 12, 0);

    for cell in &grid {
        output.set(cell.get_x(), cell.get_y(), cell.count_neighbours());
    }

    output.write(&mut std::io::stdout());
}

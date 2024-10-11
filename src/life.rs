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

use crate::grid::{Grid, GridCell, SimpleGrid, SizedGrid, SparseGrid};


type LifeCellType = char;
type LifeGridType = SimpleGrid<LifeCellType>;

pub trait LifeGrid {
    const DEAD_CELL: LifeCellType = ' ';
    const LIVE_CELL: LifeCellType = '*';
    
    /// Initialises a grid with 'dead' cells.
    fn init_life(width: usize, height: usize) -> Self;

    /// Sets a cell to 'live'.
    /// 
    /// See `is_live()`.
    fn set_live(&mut self, x: usize, y: usize);
    

    /// Sets a cell to 'dead'.
    /// 
    /// See `is_dead()`.
    fn set_dead(&mut self, x: usize, y: usize);
}

impl LifeGrid for LifeGridType {
    fn init_life(width: usize, height: usize) -> Self {
        Self::init(width, height, <LifeGridType as LifeGrid>::DEAD_CELL)
    }
    
    fn set_live(&mut self, x: usize, y: usize) {
        self.set(x, y, <LifeGridType as LifeGrid>::LIVE_CELL);
    }
    
    fn set_dead(&mut self, x: usize, y: usize) {
        self.set(x, y, <LifeGridType as LifeGrid>::DEAD_CELL);
    }
}

pub trait LifeCell {
    /// Indicates if this cell is 'live'.
    fn is_live(&self) -> bool;

    /// Indicates if this cell is 'dead'.
    fn is_dead(&self) -> bool;

    /// Indicates how many of the cell's neighbours are 'live'.
    fn count_neighbours(&self) -> i32;
}

impl<'a> LifeCell for GridCell<'a, LifeGridType> {
    fn is_live(&self) -> bool {
        self.get() == &<LifeGridType as LifeGrid>::LIVE_CELL
    }

    fn is_dead(&self) -> bool {
        self.get() == &<LifeGridType as LifeGrid>::DEAD_CELL
    }
    
    fn count_neighbours(&self) -> i32 {
        // Use an 'adjust' closure to convert the neighbouring cells (which
        // may not be within bounds) to a simple 1 for existing & live, or
        // 0 for non-existing or dead...
        let adjust = |ax: isize, ay: isize| -> i32 {
            match self.get_relative(ax, ay) {
                // Could use `is_live()` here but that would require a
                // call to `get_relative_cell()` instead of `get_relative()`
                // - doing so for all of a cell's neighbours would mean
                // instantiation of 8 new `GridCell`'s. So, instead, just do
                // a direct comparison of the relative's (if any) contents... 
                Some(x) if x == &<LifeGridType as LifeGrid>::LIVE_CELL => 1,
                _ => 0
            }
        };

        // Sum all of the neighbouring cells...
        adjust(-1, -1) + adjust(0, -1) + adjust(1, -1)
            + adjust(-1, 0) + adjust(1, 0)
            + adjust(-1, 1) + adjust(0, 1) + adjust(1, 1)
    }
}

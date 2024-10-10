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

mod grid;

use std::io::stdout;

use grid::{Grid, GridCell, SimpleGrid, SizedGrid, SparseGrid};


fn count_neighbours<T>(cell: &GridCell<T>, f: impl Fn(&T::Item) -> bool) -> u32 where T: SizedGrid {
    let adjust = |ax: isize, ay: isize| -> u32 {
        match cell.get_relative(ax, ay) {
            Some(x) if f(x) => 1,
            Some(_) => 0,
            None => 0
        }
    };

    adjust(-1, -1) + adjust(0, -1) + adjust(1, -1)
        + adjust(-1, 0) + adjust(1, 0)
        + adjust(-1, 1) + adjust(0, 1) + adjust(1, 1)
}


fn main() {
    let mut grid = SimpleGrid::<u32>::new(32, 12);
    // let mut grid = SparseGrid::<u32>::new();

    // grid.set(1, 1, 7);
    // grid.set(10, 4, 7);
    // grid.set(11, 4, 7);
    // grid.set(12, 4, 7);
    // grid.set(20, 8, 7);
    // grid.set(11, 4, 3);

    // grid[(25, 15)] = 1;

    grid.set(1, 1, 7);
    grid.set(2, 1, 7);
    grid.set(3, 1, 7);

    let c = grid.get_cell(2, 1);
    let r = c.get_relative_cell(-1, -1);

    println!("{:?}", c);
    println!("{:?}", r.unwrap());

    // for cell in &grid {
    //     println!("{:?}", cell);
    // }


    // grid.iter()
    //     .filter(|c| *c.get() > 0)
    //     // .filter(|c| count_neighbours(&c, |x| *x > 0) > 0)
    //     .inspect(|c| println!("GridCell({},{}) = {:?} [{}]", c.get_x(), c.get_y(), c.get(), count_neighbours(&c, |x| *x > 0)))
    //     .collect::<Vec<_>>();

    let mut output = SimpleGrid::<u32>::new(32, 12);

    for cell in &grid {
        output.set(cell.get_x(), cell.get_y(), count_neighbours(&cell, |x| *x > 0));
    }

    grid.write(&mut stdout());
    println!("------------------------------");
    output.write(&mut stdout());
}

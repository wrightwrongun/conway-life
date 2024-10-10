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

#[cfg(test)]

mod test_gridcell {
    use crate::grid::*;

    #[test]
    fn gridcell_get() {
        let grid = SimpleGrid::init(8, 4, 9);

        let x = grid.iter().next().unwrap();

        assert_eq!(x.get(), &9);
    }

    #[test]
    fn gridcell_get_x() {
        let grid = SimpleGrid::init(8, 4, 9);

        let mut i = grid.iter();
        i.next();
        i.next();
        let x = i.next().unwrap();

        assert_eq!(x.get_x(), 2);
    }

    #[test]
    fn gridcell_get_y() {
        let grid = SimpleGrid::init(8, 4, 9);

        let x = grid.iter().next().unwrap();

        assert_eq!(x.get_y(), 0);
    }

    #[test]
    fn gridcell_get_relative_good() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        grid.set(1, 1, 1);

        let x = grid.iter().next().unwrap();

        assert!(x.get_relative(1, 1).is_some());
    }

    #[test]
    fn gridcell_get_relative_good_value() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        grid.set(1, 1, 1);

        let x = grid.iter().next().unwrap();

        assert_eq!(x.get_relative(1, 1).unwrap(), &1);
    }

    #[test]
    fn gridcell_get_relative_bad() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        grid.set(1, 1, 1);

        let x = grid.iter().next().unwrap();

        assert!(x.get_relative(-1, -1).is_none());
    }

    #[test]
    fn gridcell_get_relative_cell_good() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        grid.set(1, 1, 1);

        let x = grid.iter().next().unwrap();

        assert!(x.get_relative_cell(1, 1).is_some());
    }

    #[test]
    fn gridcell_get_relative_cell_good_value() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        grid.set(1, 1, 1);

        let x = grid.iter().next().unwrap();

        assert_eq!(x.get_relative_cell(1, 1).unwrap().get(), &1);
    }

    #[test]
    fn gridcell_get_relative_cell_bad() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        grid.set(1, 1, 1);

        let x = grid.iter().next().unwrap();

        assert!(x.get_relative_cell(-1, -1).is_none());
    }
}
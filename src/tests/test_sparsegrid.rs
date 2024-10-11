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

mod test_sparsegrid {
    use crate::grid::*;

    #[test]
    fn sparse_new() {
        let grid: SparseGrid<i8> = SparseGrid::new();
    
        assert_eq!(grid.get(5, 5), &i8::default());
    }

    #[test]
    fn sparse_iter() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        let mut x = grid.iter();
        
        let i = x.next().unwrap();
        assert_eq!(i.get(), &9);
        assert_eq!(i.get_x(), 1);
        assert_eq!(i.get_y(), 2);
    }

    #[test]
    fn sparse_iter_none() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        let mut x = grid.iter();
        x.next();

        assert!(x.next().is_none());
    }

    #[test]
    fn sparse_iter_all() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        let mut x = grid.iter_all(2, 2);
        
        let i = x.next().unwrap();
        assert_eq!(i.get(), &i8::default());
        assert_eq!(i.get_x(), 0);
        assert_eq!(i.get_y(), 0);
    }

    #[test]
    fn sparse_iter_all_none() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        let mut x = grid.iter_all(1, 1);
        x.next();

        assert!(x.next().is_none());
    }

    #[test]
    fn sparse_has_item_yes() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);


        assert!(grid.has_item(1, 2));
    }

    #[test]
    fn sparse_has_item_no() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        assert!(!grid.has_item(2, 3));
    }

    #[test]
    fn sparse_get_option_good() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        assert!(grid.get_option(1, 2).is_some());
    }

    #[test]
    fn sparse_get_option_good_value() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        assert_eq!(grid.get_option(1, 2).unwrap(), &9);
    }

    #[test]
    fn sparse_get_option_bad() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        assert!(grid.get_option(2, 3).is_none());
    }

    #[test]
    fn sparse_get_mut_option_good() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        assert!(grid.get_mut_option(1, 2).is_some());
    }

    #[test]
    fn sparse_get_mut_option_good_value() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);
        *grid.get_mut_option(1, 2).unwrap() = 5;

        assert_eq!(grid.get(1, 2), &5);
    }

    #[test]
    fn sparse_get_mut_option_bad() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        assert!(grid.get_mut_option(2, 3).is_none());
    }

    #[test]
    fn sparse_get() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        assert_eq!(grid.get(3, 2), &i8::default());
    }

    #[test]
    fn sparse_get_mut() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        *grid.get_mut(3, 2) = 7;

        assert_eq!(grid.get(3, 2), &7);
    }

    #[test]
    fn sparse_set() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(3, 2, 7);

        assert_eq!(grid.get(3, 2), &7);
    }

    #[test]
    fn sparse_into_iter() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(1, 2, 9);

        let mut x = grid.into_iter().next().unwrap();

        assert_eq!(x.get(), &9);
        assert_eq!(x.get_x(), 1);
        assert_eq!(x.get_y(), 2);
    }

    #[test]
    fn sparse_index_default() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        assert_eq!(grid[(3, 2)], i8::default());
    }

    #[test]
    fn sparse_index_value() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid.set(3, 2, 9);

        assert_eq!(grid[(3, 2)], 9);
    }

    #[test]
    fn sparse_index_mut() {
        let mut grid: SparseGrid<i8> = SparseGrid::new();

        grid[(3, 2)] = 7;

        assert_eq!(grid[(3, 2)], 7);
    }
}

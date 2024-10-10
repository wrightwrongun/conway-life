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

mod test_simple {
    use crate::grid::*;

    #[test]
    fn simple_init_from() {
        let grid = SimpleGrid::init_from(20, 15, || '*');
    
        // grid.write(&mut std::io::stdout());

        assert_eq!(grid.get(5, 5), &'*');
    }

    #[test]
    fn simple_new_default() {
        let grid: SimpleGrid<i8> = SimpleGrid::new(20, 15);
    
        assert_eq!(grid.get(5, 5), &i8::default());
    }

    #[test]
    fn simple_init_clone() {
        let grid = SimpleGrid::init(20, 15, '*');
    
        assert_eq!(grid.get(5, 5), &'*'.clone());
    }

    #[test]
    fn simple_iter() {
        let grid = SimpleGrid::init(8, 4, 9);

        let mut x = grid.iter();

        assert_eq!(x.next().unwrap().get(), &9);
        assert_eq!(x.next().unwrap().get_x(), 1);
        assert_eq!(x.next().unwrap().get_y(), 0);
    }

    #[test]
    fn simple_iter_none() {
        let grid = SimpleGrid::init(1, 1, 9);


        let mut x = grid.iter();
        x.next();

        assert!(x.next().is_none());
    }

    #[test]
    fn simple_get_cell() {
        let grid = SimpleGrid::init(8, 4, 9);

        let x = grid.get_cell(3, 2);

        assert_eq!(x.get(), &9);
        assert_eq!(x.get_x(), 3);
        assert_eq!(x.get_y(), 2);
    }

    #[test]
    fn simple_get() {
        let grid = SimpleGrid::init(8, 4, 9);

        assert_eq!(grid.get(3, 2), &9);
    }

    #[test]
    fn simple_get_mut() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        assert_eq!(grid.get(3, 2), &9);

        *grid.get_mut(3, 2) = 7;

        assert_eq!(grid.get(3, 2), &7);
    }

    #[test]
    fn simple_set() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        assert_eq!(grid.get(3, 2), &9);

        grid.set(3, 2, 7);

        assert_eq!(grid.get(3, 2), &7);
    }

    #[test]
    fn simple_get_width() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        assert_eq!(grid.get_width(), 8);
    }

    #[test]
    fn simple_get_height() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        assert_eq!(grid.get_height(), 4);
    }

    #[test]
    fn simple_into_iter() {
        let grid = SimpleGrid::init(8, 4, 9);

        let mut x = grid.into_iter();

        assert_eq!(x.next().unwrap().get(), &9);
        assert_eq!(x.next().unwrap().get_x(), 1);
        assert_eq!(x.next().unwrap().get_y(), 0);
    }

    #[test]
    fn simple_index() {
        let grid = SimpleGrid::init(8, 4, 9);

        assert_eq!(grid[(3, 2)], 9);
    }

    #[test]
    fn simple_index_mut() {
        let mut grid = SimpleGrid::init(8, 4, 9);

        grid[(3, 2)] = 7;

        assert_eq!(grid[(3, 2)], 7);
    }
}
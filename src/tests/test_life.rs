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

mod test_life {
    use crate::grid::*;
    use crate::life::*;

    #[test]
    fn lifegrid_init() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        assert_eq!(grid.get(5, 5), &' ');
    }

    #[test]
    fn lifegrid_set_on() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);

        assert_eq!(grid.get(10, 6), &'*');
    }

    #[test]
    fn lifegrid_set_off() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);
        grid.set_off(10, 6);

        assert_eq!(grid.get(10, 6), &' ');
    }

    #[test]
    fn lifecell_is_live() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);

        assert!(grid.get_cell(10, 6).is_live());
        assert!(!grid.get_cell(10, 6).is_dead());
    }

    #[test]
    fn lifecell_is_dead() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);
        grid.set_off(10, 6);

        assert!(grid.get_cell(10, 6).is_dead());
        assert!(!grid.get_cell(10, 6).is_live());
    }

    #[test]
    fn lifecell_count_neighbours_all() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);
        grid.set_on(11, 6);
        grid.set_on(12, 6);
        grid.set_on(10, 7);
        grid.set_on(12, 7);
        grid.set_on(10, 8);
        grid.set_on(11, 8);
        grid.set_on(12, 8);
        
        assert_eq!(grid.get_cell(11, 7).count_neighbours(), 8);
    }

    #[test]
    fn lifecell_count_neighbours_some_1() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);
        grid.set_on(11, 6);
        grid.set_on(12, 6);
        grid.set_on(10, 7);
        grid.set_on(12, 7);
        grid.set_on(10, 8);
        grid.set_on(11, 8);
        grid.set_on(12, 8);
        
        assert_eq!(grid.get_cell(13, 7).count_neighbours(), 3)
    }

    #[test]
    fn lifecell_count_neighbours_some_2() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);
        grid.set_on(11, 6);
        grid.set_on(12, 6);
        grid.set_on(10, 7);
        grid.set_on(12, 7);
        grid.set_on(10, 8);
        grid.set_on(11, 8);
        grid.set_on(12, 8);
        
        assert_eq!(grid.get_cell(9, 5).count_neighbours(), 1);
    }

    #[test]
    fn lifecell_count_neighbours_some_3() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);
        grid.set_on(11, 6);
        grid.set_on(12, 6);
        grid.set_on(10, 7);
        grid.set_on(12, 7);
        grid.set_on(10, 8);
        grid.set_on(11, 8);
        grid.set_on(12, 8);
        
        assert_eq!(grid.get_cell(11, 9).count_neighbours(), 3);
    }

    #[test]
    fn lifecell_count_neighbours_some_corner_1() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(1, 0);
        grid.set_on(1, 1);
        
        assert_eq!(grid.get_cell(0, 0).count_neighbours(), 2);
    }

    #[test]
    fn lifecell_count_neighbours_some_corner_2() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(18, 10);
        grid.set_on(18, 11);
        
        assert_eq!(grid.get_cell(19, 11).count_neighbours(), 2);
    }

    #[test]
    fn lifecell_count_neighbours_none() {
        let mut grid = SimpleGrid::init_life(20, 12);    
        
        grid.set_on(10, 6);
        grid.set_on(11, 6);
        grid.set_on(12, 6);
        grid.set_on(10, 7);
        grid.set_on(12, 7);
        grid.set_on(10, 8);
        grid.set_on(11, 8);
        grid.set_on(12, 8);
        
        assert_eq!(grid.get_cell(15, 3).count_neighbours(), 0);
    }
}
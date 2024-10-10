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

mod test_gridcell;
mod test_life;
mod test_simple;
mod test_sparse;


use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::default;
use std::fmt::{Debug, Display};
use std::io::Write;
use std::ops::{Index, IndexMut};


/// Provides `Grid` getter/setter functionality.
/// 
/// Grid types should implement this trait.
pub trait Grid {
    type Item;

    /// Retrieves an item at given co-ordinates from the grid.
    /// 
    /// If the given co-ordinates are out of bounds, panics.
    fn get<'a>(&'a self, x: usize, y: usize) -> &'a Self::Item;

    /// Retrieves a mutable item at given co-ordinates from the grid.
    /// 
    /// If the given co-ordinates are out of bounds, panics.
    fn get_mut<'a>(&'a mut self, x: usize, y: usize) -> &'a mut Self::Item;

    /// Sets the value at the given co=ordinates in the grid.
    /// 
    /// If the given co-ordinates are out of bounds, panics.
    fn set(&mut self, x: usize, y: usize, value: Self::Item);
}

/// Provides bounds information for a `Grid` where the bounds are known.
/// 
/// Grid types where the size is explicitly set at initialisation should
/// implement this trait.
pub trait SizedGrid : Grid {
    /// Width of the Grid.
    fn get_width(&self) -> usize;

    /// Height of the grid.
    fn get_height(&self) -> usize;
}

type GridIndexTuple = (usize, usize);


//---------------------------------------------------------------------------//


/// A simple `Grid` implementation.
/// 
/// The contents of the Grid are stored in a `Vector` of `Vector`s. Therefore,
/// space is allocated, on initialisation, for every item in the grid.
pub struct SimpleGrid<T> {
    rows: Vec<Vec<T>>,
    width: usize,
    height: usize
}

impl<T> SimpleGrid<T> where T: Default {
    /// Initialises a fixed-size `Grid`.
    /// 
    /// All items in the grid are set to the default value.
    pub fn new(width: usize, height: usize) -> Self {
        SimpleGrid::init_from(width, height, || T::default())
    }
}

impl<T> SimpleGrid<T> where T: Clone {
    /// Initialises a fixed-size `Grid`.
    /// 
    /// All items in the grid are set to the clone of the given
    /// value.
    pub fn init(width: usize, height: usize, value: T) -> Self {
        SimpleGrid::init_from(width, height, || value.clone())
    }
}

impl<T> SimpleGrid<T> {
    /// Initialises a fixed-size `Grid`.
    /// 
    /// All items in the grid are set to result of the call to
    /// to the supplied function.
    pub fn init_from(width: usize, height: usize, f: impl Fn() -> T) -> Self {
        let mut rows: Vec<Vec<T>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut column: Vec<T> = Vec::with_capacity(width);
            for _ in 0..width {
                column.push(f());
            }
            rows.push(column);
        }

        Self {
            rows,
            width,
            height
        }
    }

    /// Gives an `Iterator` over the contents of the `Grid`.
    pub fn iter(&self) -> SimpleGridIterator<'_, T> {
        SimpleGridIterator {
            grid: self,
            x: 0,
            y: 0
        }
    }
}

impl<'a, T> SimpleGrid<T> {
    /// Gives a `GridCell` for a cell in the `Grid`.
    pub fn get_cell(&'a self, x: usize, y: usize) -> GridCell<'a, SimpleGrid<T>> {
        if (x >= self.width) || (y >= self.height) {
            panic!("SimpleGrid.get_cell({},{}) index out of bounds (width={}, height={})", x, y, self.width, self.height);
        }

        GridCell {
            grid: self,
            x,
            y,
            item: self.get(x, y)
        }
    }
}

impl<T> Grid for SimpleGrid<T> {
    type Item = T;
    
    fn get<'a>(&'a self, x: usize, y: usize) -> &'a Self::Item {
        if (x >= self.width) || (y >= self.height) {
            panic!("SimpleGrid.get({},{}) index out of bounds (width={}, height={})", x, y, self.width, self.height);
        }

        &self.rows[y][x]
    }

     fn get_mut<'a>(&'a mut self, x: usize, y: usize) -> &'a mut Self::Item {
        if (x >= self.width) || (y >= self.height) {
            panic!("SimpleGrid.get_mut({},{}) index out of bounds (width={}, height={})", x, y, self.width, self.height);
        }

        &mut self.rows[y][x]
    }

     fn set(&mut self, x: usize, y: usize, value: Self::Item) {
        if (x >= self.width) || (y >= self.height) {
            panic!("SimpleGrid.set({},{}) index out of bounds (width={}, height={})", x, y, self.width, self.height);
        }

        // *self.get_mut(x, y) = value;
        self.rows[y][x] = value;
    }
}

impl<T> SizedGrid for SimpleGrid<T> {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

impl<T> SimpleGrid<T> where T: Display {
    /// Writes the contents of the `Grid` as a grid to the given
    /// output stream.
    /// 
    /// To write to standard-out, `grid.write(&mut stdout())`.
    pub fn write(&self, w: &mut dyn Write) {
        for row in &self.rows {
            for column in row {
                w.write(format!("{}", column).as_bytes());
            }
            
            w.write("\n".as_bytes());
        }
    }
}

impl<T> Debug for SimpleGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("SimpleGrid {{ width: {}, height: {} }}", self.get_width(), self.get_height()))
    }
}

impl<'a, T> IntoIterator for &'a SimpleGrid<T> {
    type Item = GridCell<'a, SimpleGrid<T>>;
    type IntoIter = SimpleGridIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Index<GridIndexTuple> for SimpleGrid<T> {
    type Output = T;
    
    fn index(&self, index: GridIndexTuple) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<T> IndexMut<GridIndexTuple> for SimpleGrid<T> {
    fn index_mut(&mut self, index: GridIndexTuple) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
    }
}


//---------------------------------------------------------------------------//


/// Represents a single item in a `Grid`.
/// 
/// Generally used with an `Iterator`.
#[derive(Debug)]
pub struct GridCell<'a, T> where T: Grid {
    item: &'a T::Item,
    x: usize,
    y: usize,
    grid: &'a T
}

impl<'a, T> GridCell<'a, T> where T: Grid {
    /// Gives the value of the cell.
    pub fn get(&self) -> &'a T::Item {
        self.item
    }

    /// Gives the x-coordinate of the cell.
    pub fn get_x(&self) -> usize {
        self.x
    }

    /// Gives the y-coordinate of the cell.
    pub fn get_y(&self) -> usize {
        self.y
    }
}

impl<'a, T> GridCell<'a, T> where T: SizedGrid {
    /// Gives the value of a cell located relative to this cell.
    /// 
    /// If the given relative coordinates are out of bounds, then `None` is
    /// returned.
    pub fn get_relative(&'a self, dx: isize, dy: isize) -> Option<&'a T::Item> {
        let x = (self.x as isize) + dx;
        let y = (self.y as isize) + dy;

        if (x >= 0) && (x < self.grid.get_width() as isize)
            && (y >= 0) && (y < self.grid.get_height() as isize) {
            Some(self.grid.get(x as usize, y as usize))
        }
        else {
            None
        }
    }
}

impl<'a, T> GridCell<'a, SimpleGrid<T>> {
    /// Gives a cell located relative to this cell.
    /// 
    /// If the given relative coordinates are out of bounds, then `None` is
    /// returned.
    pub fn get_relative_cell(&'a self, dx: isize, dy: isize) -> Option<GridCell<'a, SimpleGrid<T>>> {
        let x = (self.x as isize) + dx;
        let y = (self.y as isize) + dy;

        if (x >= 0) && (x < self.grid.get_width() as isize)
            && (y >= 0) && (y < self.grid.get_height() as isize) {
            Some(GridCell {
                grid: self.grid,
                x: x as usize,
                y: y as usize,
                item: self.grid.get(x as usize, y as usize)
            })
        }
        else {
            None
        }
    }
}


//---------------------------------------------------------------------------//


/// `Iterator` for the contents of a `SimpleGrid`.
/// 
/// Each call to `next()` gives an instance of `GridCell`.
pub struct SimpleGridIterator<'a, T> {
    grid: &'a SimpleGrid<T>,
    x: usize,
    y: usize
}

impl<'a, T> Iterator for SimpleGridIterator<'a, T> {
    type Item = GridCell<'a, SimpleGrid<T>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if (self.x == self.grid.width) {
            self.x = 0;
            self.y += 1;
        }

        if (self.y < self.grid.height) {
            let result = GridCell {
                item: self.grid.get(self.x, self.y),
                x: self.x,
                y: self.y,
                grid: self.grid
            };

            self.x += 1;

            Some(result)
        }
        else {
            None
        }
    }
}


//---------------------------------------------------------------------------//


/// A sparse `Grid` implementation.
/// 
/// This `Grid` does not have bounds.
/// 
/// Only items in the grid that are explicitly set have storage-space
/// allocated.
pub struct SparseGrid<T> where T: Default {
    default: T,
    items: HashMap<(usize, usize), T>
}

impl<T> SparseGrid<T> where T: Default {
    /// Allocates a new grid.
    /// 
    /// Note that no bounds are given.
    pub fn new() -> Self {
        SparseGrid {
            default: T::default(),
            items: HashMap::new()
        }
    }

    /// Gives an `Iterator` over the contents of the `Grid`.
    /// 
    /// Only items that have been explicitly set are provided
    /// by the `Iterator`.
    pub fn iter(&self) -> SparseGridIterator<'_, T> {
        SparseGridIterator {
            grid: self,
            items_iter: self.items.iter()
        }
    }

    /// Gives an `Iterator` over all the cells of the `Grid` that are
    /// within the given bounds.
    /// 
    /// For all cells that have not explicitly had a value set, the `Iterator`
    /// gives the `Default` value.
    pub fn iter_all(&self, width: usize, height: usize) -> BoundedGridIterator<T> {
        BoundedGridIterator {
            grid: self,
            width,
            height,
            x: 0,
            y: 0
        }
    }

    /// Indicates whether a value has been explicitly set for the given
    /// coordinates.
    pub fn has_item(&self, x: usize, y: usize) -> bool {
        self.items.contains_key(&(x, y))
    }

    /// Gives a value for the given coordinates.
    /// 
    /// If no value has been explicitly set for the given coordinates,
    /// `None` is returned.
    pub fn get_option<'a>(&'a self, x: usize, y: usize) -> Option<&'a T> {
        self.items.get(&(x, y))
    } 

    /// Gives a mutable value for the given coordinates.
    /// 
    /// If no value has been explicitly set for the given coordinates,
    /// `None` is returned.
    pub fn get_mut_option<'a>(&'a mut self, x: usize, y: usize) -> Option<&'a mut T> {
        self.items.get_mut(&(x, y))
    } 
}

impl<T> Grid for SparseGrid<T> where T: Default {
    type Item = T;

    fn get<'a>(&'a self, x: usize, y: usize) -> &'a Self::Item {
        match self.items.get(&(x, y)) {
            Some(x) => x,
            None => &self.default
        }
    }

    fn get_mut<'a>(&'a mut self, x: usize, y: usize) -> &'a mut Self::Item {
        if !self.items.contains_key(&(x, y)) {
            self.items.insert((x, y), T::default());
        }

        self.items.get_mut(&(x, y)).unwrap()
    }

    fn set(&mut self, x: usize, y: usize, value: Self::Item) {
        if self.items.contains_key(&(x, y)) {
            *self.get_mut(x, y) = value;
        }
        else {
            self.items.insert((x, y), value);
        }
    }
}

impl<T> Debug for SparseGrid<T> where T: Default {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("SparseGrid {{ count: {} }}", self.items.len()))
    }
}

impl<'a, T> IntoIterator for &'a SparseGrid<T> where T: Default {
    type Item = GridCell<'a, SparseGrid<T>>;
    type IntoIter = SparseGridIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Index<GridIndexTuple> for SparseGrid<T> where T: Default {
    type Output = T;
    
    fn index(&self, index: GridIndexTuple) -> &Self::Output {
        self.get(index.0, index.1)
    }
}

impl<T> IndexMut<GridIndexTuple> for SparseGrid<T> where T: Default {
    fn index_mut(&mut self, index: GridIndexTuple) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
    }
}


//---------------------------------------------------------------------------//


/// An `Iterator` over the explicitly cells of a `SparseGrid`.
/// 
/// Only cells that have been explicitly set will be provided by the
/// `Iterator`.
pub struct SparseGridIterator<'a, T> where T: Default {
    grid: &'a SparseGrid<T>,
    items_iter: Iter<'a, (usize, usize), T>
}

impl<'a, T> Iterator for SparseGridIterator<'a, T> where T: Default {
    type Item = GridCell<'a, SparseGrid<T>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(((x, y), value)) = self.items_iter.next() {
            Some(GridCell {
                item: value,
                x: *x,
                y: *y,
                grid: self.grid
            })
        }
        else {
            None
        }
    }
}


//---------------------------------------------------------------------------//


/// An `Iterator` over a given range of a `SparseGrid`.
/// 
/// Any cells of the `Grid` that have not had a value explicitly set,
/// will give the `Default` value for the grid's items.
pub struct BoundedGridIterator<'a, T> where T: Default {
    grid: &'a SparseGrid<T>,
    width: usize,
    height: usize,
    x: usize,
    y: usize
}

impl<'a, T> Iterator for BoundedGridIterator<'a, T> where T: Default {
    type Item = GridCell<'a, SparseGrid<T>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if (self.x == self.width) {
            self.x = 0;
            self.y += 1;
        }

        if (self.y < self.height) {
            let result = GridCell {
                item: self.grid.get(self.x, self.y),
                x: self.x,
                y: self.y,
                grid: self.grid
            };

            self.x += 1;

            Some(result)
        }
        else {
            None
        }
    }
}

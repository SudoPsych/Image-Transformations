// `Array2` provides a fixed-size 2-dimensional array.
// An error that can arise during the use of an [`Array2D`].
//
// [`Array2D`]: struct.Array2D.html
// #[derive(Debug, Eq, PartialEq)]
// pub enum Error {
//     /// The indices (coordinates) were out of bounds.
//     IndicesOutOfBounds(usize, usize),
// }

/// Elements contained must support `Clone`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}


impl<T: Clone> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// * `val`: the value to fill every element with
    pub fn new(width: usize, height: usize, val: T) -> Self {
        Array2 {
            width,
            height,
            data: vec![val; width * height],
        }
    }

    /// Creates a new `Array2` from a Vec<T>.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`
    /// * `height`: the height of the `Array2`
    /// * `values`: A Vec<T>, in row-major order, whose
    ///             length must be `width` * `height`.
    pub fn from_row_major(width: usize, height: usize, values: Vec<T>) -> Self {
        Array2 {
            width,
            height,
            data: values,
        }    
    }

    /// The height
    pub fn height(&self) -> usize {
        self.height
    }

    /// The width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns a reference to the element at the given `column` and `row`
    /// as long as that index is in bounds
    /// (wrapped in [`Some`]). Returns [`None`] if out of bounds.
    pub fn get(&self, c: usize, r: usize) -> Option<&T> {
        self.get_index(c, r).map(|index| &self.data[index])
    }

    pub fn get_mut(&mut self, c: usize, r: usize) -> Option<&mut T> {
        self.get_index(c, r).map(move |index| &mut self.data[index])
    }

    fn get_index(&self, c: usize, r: usize) -> Option<usize> {
        if c < self.width && r < self.height {
            Some(r * self.width + c)
        } else {
            None
        }
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // The compiler knows to optimize away the div-mod ops.
        self.data
            .iter()
            .enumerate()
            .map(move |(i, v)| (i % self.width, i / self.width, v))
    }

    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width)
            // get the start of every column as a fresh iter and keep the index of the column
            // skip advances the iterator without yielding items
            .map(move |c| (c, self.data.iter().skip(c)))
            // do a flat_map for all the columns 
            .flat_map(move |(c, col)| {
                // for each iterator on a column, step forward by width for the correct next element in that column
                // step_by yields an item and then advances the iterator
                col.step_by(self.width)
                    // enumerate down the columns to get the index of the row
                    .enumerate()
                    .map(move |(r, val)| (c, r, val))
            })
    }
}

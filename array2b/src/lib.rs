use std::mem;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2b<T: Clone> {
    width: usize,
    height: usize,
    blocksize: usize,
    data: Vec<T>
}

impl<T: Clone> Array2b<T> {

    pub fn new(width: usize, height: usize, blocksize: usize, val: T) -> Self {
        Array2b {
            width,
            height,
            blocksize,
            data: vec![val; width * height]
        }
    }

    pub fn new16k_block(width: usize, height: usize, val: T) -> Self {
        Array2b {
            width,
            height,
            blocksize: ((16000 / mem::size_of::<T>()) as f64).sqrt() as usize,
            data: vec![val; width * height]
        }
    }

    pub fn from_row_major_16k_block(width: usize, height: usize, values: Vec<T>) -> Self {
        Array2b {
            width,
            height,
            blocksize: ((16000 / mem::size_of::<T>()) as f64).sqrt() as usize,
            data: values
        }
    }

    pub fn from_row_major(width: usize, height: usize, blocksize: usize, values: Vec<T>) -> Self {
        
        Array2b {
            width,
            height,
            blocksize,
            data: values
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0 .. self.width / self.blocksize)
            .flat_map(|i|
                self.data
                // group iterator into nested iterator of blocksize length
                .chunks(self.blocksize)
                // get iterators of first row
                .skip(i)
                // get chunk below it, all the way down the grid
                // each 'blocksize' iterators taken will be a full block stored
                // this works because blocks are stored in column major order
                .step_by(self.width / self.blocksize)
                // flatten each of the 'blocksize' nested iterators
                .flatten()
                // idk how to borrow properly XD
            ).enumerate()
            .map(move |(i, value)| {
                // row position
                (((i % (self.blocksize * self.height)) % self.blocksize) + ((i / (self.blocksize * self.height)) * self.blocksize),
                // column position
                (i % (self.blocksize * self.height)) / self.blocksize,
                // value
                value)
        })
    }
    
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.data.iter_mut().enumerate().map(move |(i, value)| (i, i, value))    
    }

    fn get_index(&self, row_pos: usize, col_pos: usize) -> Option<usize> {
        if col_pos < self.width && row_pos < self.height {
            Some((col_pos * self.blocksize) + ((row_pos / self.blocksize) * (self.blocksize * self.height)) + (row_pos % self.blocksize))
        } else {
            None
        }
    }

    pub fn get(&self, row_pos: usize, col_pos: usize) -> Option<&T> {
        self.get_index(row_pos, col_pos).map(|index| &self.data[index])
    }
    pub fn get_mut(&mut self, row_pos: usize, col_pos: usize) -> Option<&mut T> {
        self.get_index(row_pos, col_pos).map(move |index| &mut self.data[index])
    }
}

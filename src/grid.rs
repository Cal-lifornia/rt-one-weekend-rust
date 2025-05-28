pub struct Grid<T, const W: usize, const H: usize> {
    array: [[T; W]; H],
}

impl<T, const W: usize, const H: usize> Grid<T, W, H> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.array[x][y]
    }
    pub fn size(&self) -> usize {
        W * H
    }

    pub fn width(&self) -> usize {
        W
    }

    pub fn height(&self) -> usize {
        H
    }

    pub fn set_all<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(usize, usize) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        self.array.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                *item = setter(x, y);
            }
        });
    }
}

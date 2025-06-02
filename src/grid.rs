use std::fmt::Debug;

use tracing::{info_span, Span};
use tracing_indicatif::{span_ext::IndicatifSpanExt, style::ProgressStyle};

#[derive(Debug)]
pub struct Grid<T, const W: usize, const H: usize> {
    array: [[T; W]; H],
}

impl<T, const W: usize, const H: usize> Grid<T, W, H> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.array[y][x]
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

    pub fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(usize, usize) -> T,
        T: Send + Debug,
    {
        use rayon::prelude::*;

        let span_header = tracing::info_span!("writing pixels");
        span_header.pb_set_style(&ProgressStyle::default_bar());
        span_header.pb_set_length((self.width() * self.height()) as u64);
        let span_header_entered = span_header.enter();

        self.array.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, item) in row.iter_mut().enumerate() {
                *item = setter(x, y);
                Span::current().pb_inc(1);
            }
        });

        std::mem::drop(span_header_entered);
        std::mem::drop(span_header);
    }
}

impl<T, const W: usize, const H: usize> Default for Grid<T, W, H>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            array: [[Default::default(); W]; H],
        }
    }
}

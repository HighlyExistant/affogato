use std::{slice::Iter, iter::FilterMap};

use super::{filter_scan::FilterScan, unique_copy::UniqueCopy};

pub trait ExtendIterator: Iterator {
    fn filter_scan<St, B, F>(self, initial_state: St, f: F) -> FilterScan<Self, St, F> 
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> Option<B>,
    {
        FilterScan::new(self, initial_state, f)
    }
    fn unique_copy(self) -> UniqueCopy<Self, Self::Item>
        where Self: Sized {
        UniqueCopy::new(self)
    }
}

impl<T> ExtendIterator for std::slice::Iter<'_, T> {}
use core::fmt;
use std::ops::ControlFlow;


#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct FilterScan<I, St, F> {
    iter: I,
    f: F,
    state: St,
}

impl<I, St, F> FilterScan<I, St, F> {
    pub(in crate::data) fn new(iter: I, state: St, f: F) -> Self {
        Self { iter, f, state }
    }
}

impl<I: fmt::Debug, St: fmt::Debug, F> fmt::Debug for FilterScan<I, St, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilterScan").field("iter", &self.iter).field("state", &self.state).finish()
    }
}

impl<B, I, St, F> Iterator for FilterScan<I, St, F>
where
    I: Iterator,
    F: FnMut(&mut St, I::Item) -> Option<B>,
{
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|a|{
            (self.f)(&mut self.state, a)
        })
    }
}
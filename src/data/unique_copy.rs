#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct UniqueCopy<I, B> {
    iter: I,
    last: Option<B>,
}
impl<I, St> UniqueCopy<I, St> {
    pub(in crate::data) fn new(iter: I) -> Self {
        Self { iter, last: None }
    }
}
impl<I: Iterator<Item = B>, B> Iterator for UniqueCopy<I, B>
where
    B: Eq + Clone,
{
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|a|{
            if self.last == Some(a.clone()) {
                self.last = Some(a);
                None
            } else {
                self.last = Some(a.clone());
                Some(a.clone())
            }
        })
    }
}
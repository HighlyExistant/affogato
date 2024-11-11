mod types;
pub use types::*;
pub struct CartesianProduct<I: Iterator, J: Iterator> {
    a: I,
    b: J,
    saved_a: I,
    item_b: Option<J::Item>,
}
impl<I: Iterator + Clone, J: Iterator + Clone> Clone for CartesianProduct<I, J> 
    where J::Item: Clone {
    fn clone(&self) -> Self {
        Self { 
            a: self.a.clone(), 
            b: self.b.clone(), 
            saved_a: self.saved_a.clone(), 
            item_b: self.item_b.clone() 
        }
    }
}
impl<I: Iterator + Clone, J: Iterator> CartesianProduct<I, J> {
    pub fn new(a: I, mut b: J) -> Self {
        let item_b = b.next().unwrap();
        Self { a: a.clone(), b, saved_a: a, item_b: Some(item_b) }
    }
}
impl<I: Iterator + Clone, J: Iterator> Iterator for CartesianProduct<I, J> 
    where I::Item: Clone, 
        J::Item: Clone {
    type Item = (I::Item, J::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let b = if let Some(b) = self.item_b.clone() {
            b
        } else {
            let b = self.b.next()?;
            self.item_b = Some(b.clone());
            b
        };
        if let Some(a) = self.a.next() {
            Some((a, b))
        } else {
            self.item_b = None;
            self.a = self.saved_a.clone();
            self.next()
        }
    }
}

impl<I: ExactSizeIterator + Clone, J: ExactSizeIterator> ExactSizeIterator for CartesianProduct<I, J> 
    where I::Item: Clone, 
        J::Item: Clone {
    fn len(&self) -> usize {
        (self.saved_a.len()*self.b.len())+self.a.len()
    }
}
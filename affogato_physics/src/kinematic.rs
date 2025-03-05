use affogato_math::{vector::Vector, Real};
#[derive(Clone)]
pub struct KinematicSegment<T: Real, V: Vector<Scalar = T>> {
    pos: V,
    length: T,
}
impl<T: Real, V: Vector<Scalar = T>> KinematicSegment<T, V> {
    pub fn new(pos: V, prev: &V) -> Self {
        let length = pos.distance(prev);
        Self { pos: pos, length }
    }
    pub fn as_root(root: V) -> Self {
        Self { pos: root, length: T::ZERO }
    }
}
pub struct KinematicSegmentList<T: Real, V: Vector<Scalar = T>> {
    segments: Vec<KinematicSegment<T, V>>,
}

impl<T: Real, V: Vector<Scalar = T>> KinematicSegmentList<T, V> {
    pub fn new(root: V) -> Self {
        Self { segments: vec![KinematicSegment::as_root(root)] }
    }
    fn fabrik_front(&mut self, to: V) {
        let mut iter = self.segments.iter_mut();
        let mut prev_segment: V = to.clone();
        let mut prev_length: T =
        { // set last point to destination
            let first = iter.next().unwrap();
            first.pos = prev_segment.clone();
            first.length
        };
        for current in iter {
            let next_point = prev_segment.point_at(&current.pos, prev_length);

            prev_segment = current.pos.clone();
            prev_length = current.length;

            current.pos = next_point;
        }
    }
    fn fabrik_back(&mut self, start: V) {
        let mut iter = self.segments.iter_mut();
        let mut prev_segment: V = start.clone();
        { // set first point to start
            let first = iter.next().unwrap();
            first.pos = start.clone();
        }
        for current in iter {
            let length = current.length;
            let next_point = prev_segment.point_at(&current.pos, length);
            
            prev_segment = current.pos.clone();
            current.pos = next_point;
        }
    }
    pub fn fabrik(&mut self, to: &V, iterations: usize, error_margin: T) {
        let start = self.segments[0].pos.clone();
        for _ in 0..iterations {
            {
                let last = self.segments.last().unwrap();
                let distance = last.pos.distance(to);
                if distance-error_margin <= error_margin {
                    break;
                }
            }
            self.fabrik_front(to.clone());
            self.fabrik_back(start.clone());
        }
    }
}
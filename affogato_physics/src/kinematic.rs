use std::fmt::Debug;

use affogato_math::{vector::Vector, FromPrimitive, Real, Zero};
#[derive(Clone, Debug)]
struct KinematicSegment<V: Vector> 
    where V::Scalar: Real {
    pos: V,
    length: V::Scalar,
}
impl<V: Vector> KinematicSegment<V> 
    where V::Scalar: Real {
    pub fn new(pos: V, prev: &V) -> Self {
        let length = pos.distance(prev);
        Self { pos: pos, length }
    }
    pub fn as_root(root: V) -> Self {
        Self { pos: root, length: <V::Scalar as Zero>::ZERO }
    }
}
pub struct KinematicSegmentList<V: Vector> 
    where V::Scalar: Real {
    segments: Vec<KinematicSegment<V>>,
    length: V::Scalar,
}
impl<V: Vector + Debug> Debug for KinematicSegmentList<V>
    where V::Scalar: Real + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KinematicSegmentList")
            .field("segments", &self.segments)
            .finish()
    }
}

impl<V: Vector> KinematicSegmentList<V> 
    where V::Scalar: Real {
    pub const ITERATIONS: usize = 15;
    pub fn new(root: V) -> Self {
        let root = KinematicSegment::as_root(root);
        Self { 
            length: root.length, 
            segments: vec![root] 
        }
    }
    fn fabrik_front(&mut self, to: V, edge_index: usize) {
        let len = self.segments.len();
        let mut iter = self.segments[edge_index..len].iter_mut().rev();
        let mut prev_segment: V = to.clone();
        let mut prev_length: V::Scalar =
        { // set last point to destination
            let last = iter.next().unwrap();
            last.pos = prev_segment.clone();
            last.length
        };
        for current in iter {
            let next_point = prev_segment.point_at(&current.pos, prev_length);

            prev_segment = next_point.clone();
            prev_length = current.length;

            current.pos = next_point;
        }
    }
    fn fabrik_back(&mut self, start: V, edge_index: usize) {
        let len = self.segments.len();
        let mut iter = self.segments[edge_index..len].iter_mut();
        let mut prev_segment: V = start.clone();
        { // set first point to start
            let first = iter.next().unwrap();
            first.pos = start.clone();
        }
        for current in iter {
            let length = current.length;
            let next_point = prev_segment.point_at(&current.pos, length);
            
            prev_segment = next_point.clone();
            current.pos = next_point;
        }
    }
    pub fn fabrik(&mut self, to: &V, edge_index: usize, iterations: usize, error_margin: V::Scalar) {
        let start = self.segments[0].pos.clone();
        for _ in 0..iterations {
            {
                let last = self.segments.last().unwrap();
                let distance = last.pos.distance(to);
                if distance-error_margin <= error_margin {
                    break;
                }
            }
            self.fabrik_front(to.clone(), edge_index);
            self.fabrik_back(start.clone(), edge_index);
        }
    }
    pub fn in_place_move_to(&mut self, to: &V, edge: usize) {
        self.fabrik(to, edge, Self::ITERATIONS, <V::Scalar as FromPrimitive>::from_f64(0.001));
    }
    pub fn move_to(&mut self, to: &V, edge: usize) {
        self.fabrik_front(to.clone(), edge);
    }
    pub fn add_segment(&mut self, pos: V) {
        let segment = KinematicSegment::new(pos, &self.segments.last().unwrap().pos);
        self.length += segment.length;
        self.segments.push(segment);
    }
    pub fn length(&self) -> V::Scalar {
        self.length
    }
    pub fn len(&self) -> usize {
        self.segments.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.segments.iter().map(|v|{
            &v.pos
        })
    }
}

impl<V: Vector, I: Iterator<Item = V>> From<I> for KinematicSegmentList<V> 
    where V::Scalar: Real {
    fn from(value: I) -> Self {
        let mut iter = value.into_iter();
        let mut ret = KinematicSegmentList::new(iter.next().unwrap());
        for i in iter {
            ret.add_segment(i);
        }
        ret
    }
}
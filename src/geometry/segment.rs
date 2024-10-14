use std::{fmt::Debug, ops::{Deref, Mul, Sub}};

use num_traits::FromPrimitive;

use crate::{linear::{Vector, Vector2}, FloatingPoint, Number};
// TODO THINGS TO ADD:
// 1. LinearSegment2D
// 2. QuadraticSegment2D
// 3. CubicSegment2D
// 4. Segment2D (Has the same layout as a CubicSegment but can become a linear, quadratic or cubic)
// 5. LinearSegment3D
// 6. QuadraticSegment3D
// 7. CubicSegment3D
// 8. Segment3D (Has the same layout as a CubicSegment but can become a linear, quadratic or cubic)

pub trait Segment<T: Vector> {
    /// Represents the order of the curve
    fn order(&self) -> usize;
    fn start(&self) -> T;
    fn end(&self) -> T;
    fn get(&self, t: f64) -> T
        where T: FloatingPoint + FromPrimitive;
    fn control_point(&self, idx: usize) -> T;
    fn direction_at_start(&self) -> T {
        self.control_point(1) - self.control_point(0)
    }
    fn direction_at_end(&self) -> T {
        self.control_point(self.order()) - self.control_point(self.order()-1)
    }
    fn direction_at(&self, t: f64) -> T;
    fn adjust_end_point(&mut self, p: T);
    fn adjust_start_point(&mut self, p: T);
    fn split_in_thirds(&self) -> [Box<dyn Segment<T>>; 3] 
        where T: FloatingPoint + FromPrimitive;
}

#[derive(Clone, Copy)]
pub struct Segment2D<T> {
    pub start: Vector2<T>,
    pub end: Vector2<T>,
}
impl<T: Debug> Debug for Segment2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Segment2D")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}
impl<T: Default> Default for Segment2D<T> {
    fn default() -> Self {
        Segment2D { start: Vector2::default(), end: Vector2::default() }
    }
}
impl<T: Number> Segment2D<T> {
    pub const fn new(start: Vector2<T>, end: Vector2<T>) -> Self {
        Self { start, end }
    }
    pub fn angle(&self) -> T 
        where T: FloatingPoint {
        let dir = self.end.sub(self.start).normalize();
        let dot = dir.dot(&Vector2::right());
        dot.acos()
    }
    pub fn length(&self) -> T
        where T: FloatingPoint {
        self.start.sub(self.end).length()
    }
    pub fn from_length_angle(start: Vector2<T>, length: T, angle: T) -> Self
        where T: FloatingPoint {
        let dx = angle.cos()*length;
        let dy = angle.sin()*length;
        Self { start, end: Vector2::new(start.x+dx, start.y+dy) }
    }
    pub fn recalculate_endpoint(&self, length: T, angle: T) -> Self
        where T: FloatingPoint + Debug {
        println!("{:?}", angle);
        let dx = angle.cos()*length;
        let dy = angle.sin()*length;
        Self { start: self.start, end: Vector2::new(self.start.x+dx, self.start.y+dy) }
    }
    // obtained from https://www.geeksforgeeks.org/program-for-point-of-intersection-of-two-lines/
    pub fn intersection(&self, other: &Self) -> Option<Vector2<T>> {
        // Line AB represented as a1x + b1y = c1
        let a1 = self.end.y-self.start.y;
        let b1 = self.start.x-self.end.x;
        let c1 = (self.start.x)*a1 + self.start.y*b1;

        // Line CD represented as a2x + b2y = c2
        let a2 = other.end.y-other.start.y;
        let b2 = other.start.x-other.end.x;
        let c2 = (other.start.x)*a2 + other.start.y*b2;

        let determinant = a1*b2 - a2*b1;

        if determinant == T::zero() {
            None
        } else {
            let x = (b2*c1-b1*c2)/determinant;
            let y = (a1*c2-a2*c1)/determinant;
            Some(Vector2::new(x, y))
        }
    }
}
impl<T: Number> Segment<Vector2<T>> for Segment2D<T> {
    fn start(&self) -> Vector2<T> {
        self.start
    }
    fn end(&self) -> Vector2<T> {
        self.end
    }
    fn get(&self, t: f64) -> Vector2<T>
            where Vector2<T>: FloatingPoint + FromPrimitive + Mul<T, Output = Vector2<T>>{
        self.start + (self.end-self.start)*T::from_f64(t).unwrap()
    }
    fn control_point(&self, idx: usize) -> Vector2<T> {
        self[idx]
    }
    fn order(&self) -> usize { 2 }
    fn direction_at(&self, t: f64) -> Vector2<T> {
        self[1] - self[0]
    }
    fn adjust_end_point(&mut self, p: Vector2<T>) {}
    fn adjust_start_point(&mut self, p: Vector2<T>) {}
    fn split_in_thirds(&self) -> [Box<dyn Segment<Vector2<T>>>; 3] 
        where Vector2<T>: FloatingPoint + FromPrimitive + Mul<T, Output = Vector2<T>> {
        let a = self.get(1.0/3.0);
        let b = self.get(1.0/2.0);
        [
            Box::new(Self::new(self[0], a)),
            Box::new(Self::new(a, b)),
            Box::new(Self::new(b, self[1]))
        ]
    }

}
impl<T> Deref for Segment2D<T> {
    type Target = [Vector2<T>; 2];
    fn deref(&self) -> &Self::Target {
        unsafe { (self as *const _ as *const Vector2<T>).cast::<[Vector2<T>; 2]>().as_ref().unwrap() }
    }
}
use std::{fmt::Debug, ops::{Deref, Sub}};

use crate::{linear::{Vector, Vector2}, FloatingPoint, Number};
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

impl<T> Deref for Segment2D<T> {
    type Target = [Vector2<T>; 2];
    fn deref(&self) -> &Self::Target {
        unsafe { (self as *const _ as *const Vector2<T>).cast::<[Vector2<T>; 2]>().as_ref().unwrap() }
    }
}
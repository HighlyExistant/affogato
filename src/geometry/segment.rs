#![allow(unused)]
use std::{fmt::{Debug, Display}, ops::{Deref, Sub}};

use crate::{lerp, vector::{CrossProduct, Vector, Vector2}, Number, Real, Zero};

pub trait Segment {
    type VectorType: Vector;
    
    /// Represents the order of the curve
    fn order(&self) -> usize;
    fn start(&self) -> Self::VectorType;
    fn end(&self) -> Self::VectorType;
    fn get(&self, t: f64) -> Self::VectorType
        where <Self::VectorType as Vector>::Scalar: Real;
    fn control_point(&self, idx: usize) -> Self::VectorType;
    fn direction_at_start(&self) -> Self::VectorType {
        self.control_point(1) - self.control_point(0)
    }
    fn direction_at_end(&self) -> Self::VectorType {
        self.control_point(self.order()) - self.control_point(self.order()-1)
    }
    fn direction_at(&self, t: f64) -> Self::VectorType
    where <Self::VectorType as Vector>::Scalar: Real;
    fn adjust_end_point(&mut self, to: Self::VectorType);
    fn adjust_start_point(&mut self, to: Self::VectorType);
    fn split_in_thirds(&self) -> [Box<dyn Segment<VectorType = Self::VectorType>>; 3] 
        where <Self::VectorType as Vector>::Scalar: Real,
        Self: 'static;
}

#[derive(Clone, Copy)]
pub struct LinearSegment2D<T: Number> {
    pub start: Vector2<T>,
    pub end: Vector2<T>,
}
impl<T: Debug + Number> Debug for LinearSegment2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinearSegment2D")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}
impl<T: Default + Number> Default for LinearSegment2D<T> {
    fn default() -> Self {
        LinearSegment2D { start: Vector2::default(), end: Vector2::default() }
    }
}
impl<T: Number> LinearSegment2D<T> {
    pub const fn new(start: Vector2<T>, end: Vector2<T>) -> Self {
        Self { start, end }
    }
    pub fn angle(&self) -> T 
        where T: Real {
        let dir = self.end.sub(self.start).normalize();
        let dot = dir.dot(&Vector2::right());
        dot.acos()
    }
    pub fn length(&self) -> T
        where T: Real {
        self.start.sub(self.end).length()
    }
    pub fn from_length_angle(start: Vector2<T>, length: T, angle: T) -> Self
        where T: Real {
        let dx = angle.cos()*length;
        let dy = angle.sin()*length;
        Self { start, end: Vector2::new(start.x+dx, start.y+dy) }
    }
    pub fn recalculate_endpoint(&self, length: T, angle: T) -> Self
        where T: Real + Debug {
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

        if determinant == T::ZERO {
            None
        } else {
            let x = (b2*c1-b1*c2)/determinant;
            let y = (a1*c2-a2*c1)/determinant;
            Some(Vector2::new(x, y))
        }
    }
    pub fn split_in_thirds_static(&self) -> [Self; 3] 
        where <Vector2<T> as Vector>::Scalar: Real,
        Self: 'static {
        let a = self.get(1.0/3.0);
        let b = self.get(1.0/2.0);
        [
            Self::new(self[0], a),
            Self::new(a, b),
            Self::new(b, self[1])
        ]
    }
}
impl<T: Number> Segment for LinearSegment2D<T> {
    type VectorType = Vector2<T>;
    fn start(&self) -> Vector2<T> {
        self.start
    }
    fn end(&self) -> Vector2<T> {
        self.end
    }
    fn get(&self, t: f64) -> Vector2<T>
        where  <Self::VectorType as Vector>::Scalar: Real {
        self.start + (self.end-self.start)*<Self::VectorType as Vector>::Scalar::from_f64(t)
    }
    fn control_point(&self, idx: usize) -> Vector2<T> {
        self[idx]
    }
    fn order(&self) -> usize { 2 }
    fn direction_at(&self, _: f64) -> Vector2<T> {
        self[1] - self[0]
    }
    fn adjust_end_point(&mut self, to: Vector2<T>) {
        self.end = to;
    }
    fn adjust_start_point(&mut self, to: Vector2<T>) {
        self.start = to;
    }
    fn split_in_thirds(&self) -> [Box<dyn Segment<VectorType = Self::VectorType>>; 3] 
        where <Self::VectorType as Vector>::Scalar: Real,
        Self: 'static {
        let a = self.get(1.0/3.0);
        let b = self.get(1.0/2.0);
        [
            Box::new(Self::new(self[0], a)),
            Box::new(Self::new(a, b)),
            Box::new(Self::new(b, self[1]))
        ]
    }

}
impl<T: Number> Deref for LinearSegment2D<T> {
    type Target = [Vector2<T>; 2];
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&Self, &[Vector2<T>; 2]>(self) }
    }
}

#[derive(Clone, Copy)]
pub struct QuadraticSegment2D<T: Number> {
    pub start: Vector2<T>,
    pub control: Vector2<T>,
    pub end: Vector2<T>,
}
impl<T: Debug + Number> Debug for QuadraticSegment2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuadraticSegment2D")
            .field("start", &self.start)
            .field("control", &self.control)
            .field("end", &self.end)
            .finish()
    }
}
impl<T: Number> QuadraticSegment2D<T> {
    pub fn new(start: Vector2<T>, control: Vector2<T>, end: Vector2<T>) -> Self {
        Self { start, control, end }
    }
    pub fn split_in_thirds_static(&self) -> [Self; 3] 
            where <Vector2<T> as Vector>::Scalar: Real,
            Self: 'static {
        let p0p01_13 = lerp(self[0], self[1], T::from_f64(1.0/3.0));
        let p_13 = self.get(1.0/3.0);
        let p_23 = self.get(2.0/3.0);
        let part1 = QuadraticSegment2D::new(self[0], p0p01_13, p_13);
        let part2 = QuadraticSegment2D::new(p_13, lerp(lerp(self[0], self[1], T::from_f64(5.0/9.0)), lerp(self[1], self[2], T::from_f64(4.0/9.0)), T::from_f64(0.5)), p_23);
        let part3 = QuadraticSegment2D::new(p_23, lerp(self[1], self[2], T::from_f64(2.0/3.0)), self[2]);
        [
            part1,
            part2,
            part3,
        ]
    }
}
impl<T: Number> Segment for QuadraticSegment2D<T> {
    type VectorType = Vector2<T>;
    fn start(&self) -> Self::VectorType {
        self.start
    }
    fn end(&self) -> Self::VectorType {
        self.end
    }
    fn get(&self, t: f64) -> Self::VectorType
            where <Self::VectorType as Vector>::Scalar: Real {
        let t = T::from_f64(t);
        lerp(lerp(self.start, self.control, t), lerp(self.control, self.end, t), t)
    }
    fn control_point(&self, idx: usize) -> Vector2<T> {
        self[idx]
    }
    fn order(&self) -> usize { 3 }
    fn direction_at(&self, t: f64) -> Self::VectorType
        where <Self::VectorType as Vector>::Scalar: Real {
        let t = T::from_f64(t);
        let tangent = lerp(self.control-self.start, self.end-self.control, t);
        if !tangent.is_zero() {
            return self.end-self.start;
        }
        tangent
    }
    fn adjust_start_point(&mut self, to: Self::VectorType) {
        let orig_sdir = self.start-self.control;
        let orig_p1 = self.control;
        self.control = (self.end-self.control)*orig_sdir.cross(&(to-self.start))/orig_sdir.cross(&(self.end-self.control));
        self.start = to;
        if orig_sdir.dot(&(self.start-self.control)) < T::ZERO {
            self.control = orig_p1;
        }
    }
    fn adjust_end_point(&mut self, to: Self::VectorType) {
        let orig_sdir = self.end-self.control;
        let orig_p1 = self.control;
        self.control = (self.start-self.control)*orig_sdir.cross(&(to-self.end))/orig_sdir.cross(&(self.start-self.control));
        self.end = to;
        if orig_sdir.dot(&(self.end-self.control)) < T::ZERO {
            self.control = orig_p1;
        }
    }
    fn split_in_thirds(&self) -> [Box<dyn Segment<VectorType = Self::VectorType>>; 3] 
            where <Self::VectorType as Vector>::Scalar: Real,
            Self: 'static {
        let p0p01_13 = lerp(self[0], self[1], T::from_f64(1.0/3.0));
        let p_13 = self.get(1.0/3.0);
        let p_23 = self.get(2.0/3.0);
        let part1 = QuadraticSegment2D::new(self[0], p0p01_13, p_13);
        let part2 = QuadraticSegment2D::new(p_13, lerp(lerp(self[0], self[1], T::from_f64(5.0/9.0)), lerp(self[1], self[2], T::from_f64(4.0/9.0)), T::from_f64(0.5)), p_23);
        let part3 = QuadraticSegment2D::new(p_23, lerp(self[1], self[2], T::from_f64(2.0/3.0)), self[2]);
        [
            Box::new(part1),
            Box::new(part2),
            Box::new(part3),
        ]
    }
}
impl<T: Number> Deref for QuadraticSegment2D<T> {
    type Target = [Vector2<T>; 3];
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&Self, &[Vector2<T>; 3]>(self) }
    }
}

#[derive(Clone, Copy)]
pub struct CubicSegment2D<T: Number> {
    pub start: Vector2<T>,
    pub control1: Vector2<T>,
    pub control2: Vector2<T>,
    pub end: Vector2<T>,
}
impl<T: Debug + Number> Debug for CubicSegment2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CubicSegment2D")
            .field("start", &self.start)
            .field("control1", &self.control1)
            .field("control2", &self.control2)
            .field("end", &self.end)
            .finish()
    }
}
impl<T: Number> CubicSegment2D<T> {
    pub fn new(start: Vector2<T>, control1: Vector2<T>, control2: Vector2<T>, end: Vector2<T>) -> Self {
        Self { start, control1, control2, end }
    }
    fn split_in_thirds_static(&self) -> [Self; 3]  
        where <Vector2<T> as Vector>::Scalar: Real,
        Self: 'static {
        let t_1_3 = T::from_f64(1.0/3.0);
        let t_2_3 = T::from_f64(2.0/3.0);
        let part1_1 = if self[0] == self[1] {
            self[0]
        } else {
            lerp(self[0], self[1], t_1_3)
        };
        let part1_2 = lerp(lerp(self[0], self[1], t_1_3), lerp(self[1], self[2], t_1_3), t_1_3);
        let part1_3 = self.get(1.0/3.0);
        let part1 = CubicSegment2D::new(self[0], part1_1, part1_2, part1_3);
        let part_2_1 = lerp(part1_2, lerp(lerp(self[1], self[2], t_1_3), lerp(self[2], self[3], t_1_3), t_1_3), t_2_3);
        let part_2_2 = lerp(lerp(lerp(self[0], self[1], t_1_3), lerp(self[1], self[2], t_1_3), t_1_3), lerp(lerp(self[1], self[2], t_2_3), lerp(self[2], self[3], t_2_3), t_2_3), t_1_3);
        let part_2_3 = self.get(2.0/3.0);
        let part2 = CubicSegment2D::new(part1_3, part_2_1, part_2_2, part_2_3);
        let part_3_1 = lerp(lerp(self[1], self[2], t_2_3), lerp(self[2], self[3], t_2_3), t_2_3);
        let part_3_2 = if self[2] == self[3] {
            self[3]
        } else {
            lerp(self[2], self[3], t_2_3)
        };
        let part3 = CubicSegment2D::new(part_2_3, part_3_1, part_3_2, self[3]);

        [
            part1,
            part2,
            part3,
        ]
    }
}

impl<T: Number> Segment for CubicSegment2D<T> {
    type VectorType = Vector2<T>;
    fn start(&self) -> Self::VectorType {
        self.start
    }
    fn end(&self) -> Self::VectorType {
        self.end
    }
    fn get(&self, t: f64) -> Self::VectorType
            where <Self::VectorType as Vector>::Scalar: Real {
        let t = T::from_f64(t);
        let a = lerp(self.start, self.control1, t);
        let b = lerp(self.control1, self.control2, t);
        let c = lerp(self.control2, self.end, t);
        let e = lerp(a, b, t);
        let f = lerp(b, c, t);
        lerp(e, f, t)
    }
    fn control_point(&self, idx: usize) -> Vector2<T> {
        self[idx]
    }
    fn order(&self) -> usize { 4 }
    fn direction_at(&self, t: f64) -> Self::VectorType
        where <Self::VectorType as Vector>::Scalar: Real {
        let t = T::from_f64(t);
        let tangent = lerp(lerp(self.control1-self.start, self.control2-self.control1, t), lerp(self.control2-self.control1, self.end-self.control2, t), t);
        if !tangent.is_zero() {
            if t == T::ZERO { return self.control2-self.start; }
            if t == T::ONE { return self.end-self.control1; }
        }
        tangent
    }
    fn adjust_start_point(&mut self, to: Self::VectorType) {
        self.control1 += to-self.start;
        self.start = to;
    }
    fn adjust_end_point(&mut self, to: Self::VectorType) {
        self.control2 += to-self.end;
        self.end = to;
    }
    fn split_in_thirds(&self) -> [Box<dyn Segment<VectorType = Self::VectorType>>; 3] 
            where <Self::VectorType as Vector>::Scalar: Real,
            Self: 'static {
        let t_1_3 = T::from_f64(1.0/3.0);
        let t_2_3 = T::from_f64(2.0/3.0);
        let part1_1 = if self[0] == self[1] {
            self[0]
        } else {
            lerp(self[0], self[1], t_1_3)
        };
        let part1_2 = lerp(lerp(self[0], self[1], t_1_3), lerp(self[1], self[2], t_1_3), t_1_3);
        let part1_3 = self.get(1.0/3.0);
        let part1 = CubicSegment2D::new(self[0], part1_1, part1_2, part1_3);
        let part_2_1 = lerp(part1_2, lerp(lerp(self[1], self[2], t_1_3), lerp(self[2], self[3], t_1_3), t_1_3), t_2_3);
        let part_2_2 = lerp(lerp(lerp(self[0], self[1], t_1_3), lerp(self[1], self[2], t_1_3), t_1_3), lerp(lerp(self[1], self[2], t_2_3), lerp(self[2], self[3], t_2_3), t_2_3), t_1_3);
        let part_2_3 = self.get(2.0/3.0);
        let part2 = CubicSegment2D::new(part1_3, part_2_1, part_2_2, part_2_3);
        let part_3_1 = lerp(lerp(self[1], self[2], t_2_3), lerp(self[2], self[3], t_2_3), t_2_3);
        let part_3_2 = if self[2] == self[3] {
            self[3]
        } else {
            lerp(self[2], self[3], t_2_3)
        };
        let part3 = CubicSegment2D::new(part_2_3, part_3_1, part_3_2, self[3]);

        [
            Box::new(part1),
            Box::new(part2),
            Box::new(part3),
        ]
    }
}

impl<T: Number> Deref for CubicSegment2D<T> {
    type Target = [Vector2<T>; 4];
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&Self, &[Vector2<T>; 4]>(self) }
    }
}
#[derive(Clone, Debug)]
pub enum Segment2D<T: Number> {
    Linear(LinearSegment2D<T>),
    Quadratic(QuadraticSegment2D<T>),
    Cubic(CubicSegment2D<T>),
}
impl<T: Debug + Number> Display for Segment2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Segment2D::Linear(linear) => f.write_fmt(format_args!("{:?}", linear)),
            Segment2D::Quadratic(quadratic) => f.write_fmt(format_args!("{:?}", quadratic)),
            Segment2D::Cubic(cubic) => f.write_fmt(format_args!("{:?}", cubic)),
        }
    }
}
impl<T: Number> Segment2D<T> {
    pub fn get(&self) -> &dyn Segment<VectorType = Vector2<T>> {
        match self {
            Segment2D::Linear(linear) => linear,
            Segment2D::Quadratic(quadratic) => quadratic,
            Segment2D::Cubic(cubic) => cubic,
        }
    }
    pub fn get_mut(&mut self) -> &mut dyn Segment<VectorType = Vector2<T>> {
        match self {
            Segment2D::Linear(linear) => linear,
            Segment2D::Quadratic(quadratic) => quadratic,
            Segment2D::Cubic(cubic) => cubic,
        }
    }
    pub fn linear(start: Vector2<T>, end: Vector2<T>) -> Self {
        Segment2D::Linear(LinearSegment2D::new(start, end))
    }
    pub fn quadratic(start: Vector2<T>, control: Vector2<T>, end: Vector2<T>) -> Self {
        Segment2D::Quadratic(QuadraticSegment2D::new(start, control, end))
    }
    pub fn cubic(start: Vector2<T>, control1: Vector2<T>, control2: Vector2<T>, end: Vector2<T>) -> Self {
        Segment2D::Cubic(CubicSegment2D::new(start, control1, control2, end))
    }
    fn split_in_thirds_static(&self) -> [Self; 3] 
        where <Vector2<T> as Vector>::Scalar: Real,
        Self: 'static {
        match self {
            Segment2D::Linear(linear) => {
                let linear = linear.split_in_thirds_static();
                [
                    Self::Linear( linear[0]),
                    Self::Linear( linear[1]),
                    Self::Linear( linear[2]),
                ]
            },
            Segment2D::Quadratic(quadratic) => {
                let quadratic = quadratic.split_in_thirds_static();
                [
                    Self::Quadratic(quadratic[0]),
                    Self::Quadratic(quadratic[1]),
                    Self::Quadratic(quadratic[2]),
                ]
            },
            Segment2D::Cubic(cubic) => {
                let cubic = cubic.split_in_thirds_static();
                [
                    Self::Cubic(cubic[0]),
                    Self::Cubic(cubic[1]),
                    Self::Cubic(cubic[2]),
                ]
            },
        }
    }
}
impl<T: Number> Segment for Segment2D<T> {
    type VectorType = Vector2<T>;
    fn start(&self) -> Self::VectorType {
        self.get().start()
    }
    fn end(&self) -> Self::VectorType {
        self.get().end()
    }
    fn get(&self, t: f64) -> Self::VectorType
            where <Self::VectorType as Vector>::Scalar: Real {
        
        self.get().get(t)
    }
    fn control_point(&self, idx: usize) -> Vector2<T> {
        self.get().control_point(idx)
    }
    fn order(&self) -> usize { self.get().order() }
    fn direction_at(&self, t: f64) -> Self::VectorType
        where <Self::VectorType as Vector>::Scalar: Real {
        self.get().direction_at(t)
    }
    fn adjust_start_point(&mut self, to: Self::VectorType) {
        self.get_mut().adjust_start_point(to)
    }
    fn adjust_end_point(&mut self, to: Self::VectorType) {
        self.get_mut().adjust_end_point(to)
    }
    fn split_in_thirds(&self) -> [Box<dyn Segment<VectorType = Self::VectorType>>; 3] 
            where <Self::VectorType as Vector>::Scalar: Real,
            Self: 'static {
        match self {
            Segment2D::Linear(linear) => linear.split_in_thirds(),
            Segment2D::Quadratic(quadratic) => quadratic.split_in_thirds(),
            Segment2D::Cubic(cubic) => cubic.split_in_thirds(),
        }
    }
}
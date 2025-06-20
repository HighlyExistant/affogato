#![allow(unused)]
use std::{fmt::Debug, ops::{Deref, Div}};

use bytemuck::{Pod, Zeroable};

use crate::{sdf::{RoundSignedDistance, SignedDistance}, vector::{DVec3, FVec3, Vector, Vector2, Vector3}, HasNegatives, Number, Real, Zero};

use super::{CalculateCentroid, Triangle2D, Triangle3D};

macro_rules! impl_ops_rect {
    ($structure:tt, $vector:tt) => {
        impl<T: Number> std::ops::Add<$vector<T>> for $structure<T> {
            type Output = Self;
            fn add(self, rhs: $vector<T>) -> Self::Output {
                Self { min: self.min+rhs, max: self.max+rhs }
            }
        }
        impl<T: Number> std::ops::Sub<$vector<T>> for $structure<T> {
            type Output = Self;
            fn sub(self, rhs: $vector<T>) -> Self::Output {
                Self { min: self.min-rhs, max: self.max-rhs }
            }
        }
        impl<T: Number> std::ops::Mul<$vector<T>> for $structure<T> {
            type Output = Self;
            fn mul(self, rhs: $vector<T>) -> Self::Output {
                Self { min: self.min*rhs, max: self.max*rhs }
            }
        }
        impl<T: Number> std::ops::Div<$vector<T>> for $structure<T> {
            type Output = Self;
            fn div(self, rhs: $vector<T>) -> Self::Output {
                Self { min: self.min/rhs, max: self.max/rhs }
            }
        }
        impl<T: Number> std::ops::Mul<T> for $structure<T> {
            type Output = Self;
            fn mul(self, rhs: T) -> Self::Output {
                Self { min: self.min*rhs, max: self.max*rhs }
            }
        }
        impl<T: Number> std::ops::Div<T> for $structure<T> {
            type Output = Self;
            fn div(self, rhs: T) -> Self::Output {
                Self { min: self.min/rhs, max: self.max/rhs }
            }
        }
        impl<T: Number> std::cmp::PartialEq for $structure<T> {
            fn eq(&self, other: &Self) -> bool {
                self.min == other.min && self.max == other.max
            }
        }
    };
}

/// Represents an abstract N-Dimensional cube
pub trait HyperCube<T: Number> {
    const DIMENSION: usize;
}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Rect3D<T: Number> {
    min: Vector3<T>,
    max: Vector3<T>,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct ConstRect3D<T: Number> {
    pub min: Vector3<T>,
    pub max: Vector3<T>,
}
impl<T: Number> Deref for Rect3D<T> {
    type Target = ConstRect3D<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: Number> HyperCube<T> for Rect3D<T> {
    const DIMENSION: usize = 3;
}
impl<T: Number> Default for Rect3D<T> {
    fn default() -> Self {
        Self { min: Vector3::from(T::MIN), max: Vector3::from(T::MAX) }
    }
}
impl<T: Number> Rect3D<T> {
    pub const fn edge_indices() -> [u32; 24] {
        [
            0, 2,
            2, 1,
            1, 3,
            3, 0,
            0, 4,
            4, 7,
            7, 3,
            7, 5,
            5, 6,
            6, 4,
            5, 1,
            6, 2,
        ]
    }
    pub const fn tri_indices() -> [u32; 36] {
        [
            0, 2, 1,
            1, 3, 0,
            2, 6, 5,
            5, 1, 2,
            6, 4, 7,
            7, 5, 6,
            4, 0, 3,
            3, 7, 4,
            0, 2, 6,
            6, 4, 0,
            3, 1, 5,
            5, 7, 3
        ]
    }

    pub fn new(min: Vector3<T>, max: Vector3<T>) -> Self {
        Self { min: min, max: max }
    }
    pub fn volume(&self) -> T {
        let origin = self.max-self.min;
        origin.x()*origin.y()*origin.z()
    }
    pub fn from_lengths(width: T, height: T, depth: T) -> Self {
        Self::new(Vector3::ZERO, Vector3::new(width, height, depth))
    }
    pub fn size(&self) -> Vector3<T> {
        Vector3::new(self.width(), self.height(), self.depth())
    }
    pub fn width(&self) -> T {
        self.max.x() - self.min.x()
    }
    pub fn height(&self) -> T {
        self.max.y() - self.min.y()
    }
    pub fn depth(&self) -> T {
        self.max.z() - self.min.z()
    }
    /// merge 2 [`Rect3D`] objects together, so that both can fit within eachother.
    pub fn merge(&self, aabb: &Self) -> Self 
        where T: Debug {
        let mut t = *self;
        t.min = t.min.min(&aabb.min);
        t.max = t.max.max(&aabb.max);
        t
    }
    /// Using a [`Vector3`], adjust the bounds of the [`Rect3D`] to fit at least the vector.
    pub fn vector_adjust_bounds(&self, v: Vector3<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&v);
        t.max = t.min.max(&v);
        t
    }
    /// Using a [`Triangle3D`], adjust the bounds of the [`Rect3D`] to fit at least the triangle.
    pub fn triangle_adjust_bounds(&self, triangle: &Triangle3D<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&triangle[0]);
        t.min = t.min.min(&triangle[1]);
        t.min = t.min.min(&triangle[2]);
        t.max = t.max.max(&triangle[0]);
        t.max = t.max.max(&triangle[1]);
        t.max = t.max.max(&triangle[2]);
        t
    }
    pub fn normalize(&self) -> Self 
        where T: Real {
        let rect = Vector3::new(self.width(), self.height(), self.depth()).normalize();
        Self::from_lengths(rect.x(), rect.y(), rect.z())
    }
    fn fix_bounds(&self) -> Self {
        let mut t = *self;
        t.min = self.min.min(&self.max);
        t.max = self.max.max(&self.min);
        t
    }
    pub fn minimum(&self) -> &Vector3<T> {
        &self.min
    }
    pub fn maximum(&self) -> &Vector3<T> {
        &self.max
    }
    /// Initializes a [`Rect3D`] to have the minimum be the largest value and the
    /// maximum to be the smallest value, Useful in scenarios when garunteeing 
    /// that the first call to adjust bounds will adjust it correctly.
    pub unsafe fn inverted_bounds_default() -> Self {
        Self { min: Vector3::from(T::MAX), max: Vector3::from(T::MIN) }
    }
    pub fn min(&self, other: &Self) -> Self {
        Self { min: self.min.min(&other.min), max: self.max.min(&other.max) }
    }
    pub fn max(&self, other: &Self) -> Self {
        Self { min: self.min.max(&other.min), max: self.max.max(&other.max) }
    }
    /// gets the vertices of the cube in the following order
    /// ```no_run,ignore
    ///         7───────────────────5
    ///        ╱                   ╱│
    ///       ╱                   ╱ │
    ///      ╱                   ╱  │
    ///     3───────────────────1   │
    ///     │                   │   │
    ///     │   4               │   6
    ///     │                   │  ╱
    ///     │                   │ ╱
    ///     │                   │╱
    ///     0───────────────────2
    /// ```
    pub fn get_vertices(&self) -> Vec<Vector3<T>> {
        vec![
            Vector3::new(self.min.x(), self.min.y(), self.min.z()),
            Vector3::new(self.max.x(), self.max.y(), self.min.z()),
            Vector3::new(self.max.x(), self.min.y(), self.min.z()),
            Vector3::new(self.min.x(), self.max.y(), self.min.z()),
            Vector3::new(self.min.x(), self.min.y(), self.max.z()),
            Vector3::new(self.max.x(), self.max.y(), self.max.z()),
            Vector3::new(self.max.x(), self.min.y(), self.max.z()),
            Vector3::new(self.min.x(), self.max.y(), self.max.z())
        ]
    }
    pub fn get_edge_indices(&self) -> Vec<u32> {
        vec![
            0, 2,
            2, 1,
            1, 3,
            3, 0,
            0, 4,
            4, 7,
            7, 3,
            7, 5,
            5, 6,
            6, 4,
            5, 1,
            6, 2,
        ]
    }
    pub fn get_tri_indices(&self) -> Vec<u32> {
        vec![
            0, 2, 1,
            1, 3, 0,
            2, 6, 5,
            5, 1, 2,
            6, 4, 7,
            7, 5, 6,
            4, 0, 3,
            3, 7, 4,
            0, 2, 6,
            6, 4, 0,
            3, 1, 5,
            5, 7, 3
        ]
    }
    pub fn intersect_point(&self, point: &Vector3<T>) -> bool {
        point.x() >= self.min.x()  &&
        point.x() <= self.max.x() &&
        point.y() >= self.min.y() &&
        point.y() <= self.max.y() &&
        point.z() >= self.min.z() &&
        point.z() <= self.max.z()
    }
    pub fn intersect(&self, rect: &Self) -> bool {
        self.min.x() <= rect.max.x() &&
        self.max.x() >= rect.min.x() &&
        self.min.y() <= rect.max.y() &&
        self.max.y() >= rect.min.y() &&
        self.min.z() <= rect.max.z() &&
        self.max.z() >= rect.min.z()
    }
    pub fn center_to_origin(&self) -> Self 
        where T: Real {
        let origin = (self.max-self.min).div(T::from_f64(2.0));
        Self { min: -origin, max: origin }
    }
    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: std::ops::Range<T>) -> Self 
        where T: rand::distr::uniform::SampleUniform {
            Self::new(Vector3::random(generator, range.clone()), Vector3::random(generator, range)).fix_bounds()
    }
}

impl<T: Real> CalculateCentroid for Rect3D<T> {
    type VectorType = Vector3<T>;
    fn centroid(&self) -> Vector3<T> {
        Vector3::new(
            (self.min.x() + self.max.x())*T::from_f64(0.5), 
            (self.min.y() + self.max.y())*T::from_f64(0.5), 
            (self.min.z() + self.max.z())*T::from_f64(0.5)
        )
    }
}

impl<T: Number> From<Vector3<T>> for Rect3D<T> {
    fn from(value: Vector3<T>) -> Self {
        let min = Vector3::ZERO.min(&value);
        let max = Vector3::ZERO.max(&value);
        Self { min, max }
    }
}

unsafe impl<T: Number> Zeroable for Rect3D <T> {
    fn zeroed() -> Self {
        Rect3D { min: Vector3::ZERO, max: Vector3::ZERO }
    }
}
unsafe impl<T: Number + Pod> Pod for Rect3D <T> {}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Rect<T: Number> {
    min: Vector2<T>,
    max: Vector2<T>,
}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct ConstRect<T: Number> {
    pub min: Vector2<T>,
    pub max: Vector2<T>,
}
impl<T: Number> Deref for Rect<T> {
    type Target = ConstRect<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T: Number> HyperCube<T> for Rect<T> {
    const DIMENSION: usize = 2;
}
impl<T: Number> Default for Rect<T> {
    fn default() -> Self {
        Self { min: Vector2::from(T::MIN), max: Vector2::from(T::MAX) }
    }
}
impl<T: Number> Rect<T> {
    pub const fn edge_indices() -> [u32; 8] {
        [
            0, 1,
            1, 2,
            2, 3,
            3, 0
        ]
    }
    pub const fn tri_indices() -> [u32; 6] {
        [
            0, 1, 2,
            2, 3, 0
        ]
    }

    pub fn new(min: Vector2<T>, max: Vector2<T>) -> Self {
        Self { min: min, max: max }
    }
    pub fn area(&self) -> T {
        let origin = self.max-self.min;
        origin.x()*origin.y()
    }
    pub fn from_lengths(width: T, height: T) -> Self {
        Self::new(Vector2::ZERO, Vector2::new(width, height))
    }
    pub fn size(&self) -> Vector2<T> {
        Vector2::new(self.width(), self.height())
    }
    pub fn width(&self) -> T {
        self.max.x() - self.min.x()
    }
    pub fn height(&self) -> T {
        self.max.y() - self.min.y()
    }
    pub fn merge(&self, aabb: &Self) -> Self {
        let mut t = *self;
        t.min = t.min.min(&aabb.min);
        t.max = t.min.min(&aabb.max);
        t
    }
    pub fn vector_adjust_bounds(&self, v: Vector2<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&v);
        t.max = t.min.max(&v);
        t
    }
    pub fn triangle_adjust_bounds(&self, triangle: &Triangle2D<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&triangle[0]);
        t.min = t.min.min(&triangle[1]);
        t.min = t.min.min(&triangle[2]);
        t.max = t.max.max(&triangle[0]);
        t.max = t.max.max(&triangle[1]);
        t.max = t.max.max(&triangle[2]);
        t
    }
    pub fn normalize(&self) -> Self 
        where T: Real {
        let rect = Vector2::new(self.width(), self.height()).normalize();
        Self::from_lengths(rect.x(), rect.y())
    }
    pub fn to_origin(&self) -> Self 
        where T: Real {
        Self::from_lengths(self.width(), self.height())
    }
    fn fix_bounds(&self) -> Self {
        let mut t = *self;
        t.min = self.min.min(&self.max);
        t.max = self.max.max(&self.min);
        t
    }
    pub fn minimum(&self) -> &Vector2<T> {
        &self.min
    }
    pub fn maximum(&self) -> &Vector2<T> {
        &self.max
    }
    /// Initializes a [`Rect`] to have the minimum be the largest value and the
    /// maximum to be the smallest value, Useful in scenarios when garunteeing 
    /// that the first call to adjust bounds will adjust it correctly.
    pub unsafe fn inverted_bounds_default() -> Self {
        Self { min: Vector2::from(T::MAX), max: Vector2::from(T::MIN) }
    }
    pub fn min(&self, other: &Self) -> Self {
        Self { min: self.min.min(&other.min), max: self.max.min(&other.max) }
    }
    pub fn max(&self, other: &Self) -> Self {
        Self { min: self.min.max(&other.min), max: self.max.max(&other.max) }
    }
    /// gets the vertices of the rectangle in the following order
    /// ```no_run,ignore
    /// 3─────────2
    /// │         │
    /// │         │
    /// │         │
    /// 0─────────1
    /// ```
    pub fn get_vertices(&self) -> Vec<Vector2<T>> {
        vec![
            Vector2::new(self.min.x(), self.min.y()),
            Vector2::new(self.max.x(), self.min.y()),
            Vector2::new(self.max.x(), self.max.y()),
            Vector2::new(self.min.x(), self.max.y()),
        ]
    }
    pub fn get_edge_indices(&self) -> Vec<u32> {
        vec![
            0, 1,
            1, 2,
            2, 3,
            3, 0
        ]
    }
    pub fn get_tri_indices(&self) -> Vec<u32> {
        vec![
            0, 1, 2,
            2, 3, 0
        ]
    }
    pub fn intersect_point(&self, point: &Vector2<T>) -> bool {
        point.x() >= self.min.x()  &&
        point.x() <= self.max.x() &&
        point.y() >= self.min.y() &&
        point.y() <= self.max.y()
    }
    pub fn intersect(&self, rect: &Self) -> bool {
        self.min.x() <= rect.max.x() &&
        self.max.x() >= rect.min.x() &&
        self.min.y() <= rect.max.y() &&
        self.max.y() >= rect.min.y()
    }
    pub fn center_to_origin(&self) -> Self 
        where T: Real {
        let origin = (self.max-self.min).div(T::from_f64(2.0));
        Self { min: -origin, max: origin }
    }
    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: std::ops::Range<T>) -> Self 
        where T: rand::distr::uniform::SampleUniform {
            Self::new(Vector2::random(generator, range.clone()), Vector2::random(generator, range)).fix_bounds()
    }
}
impl_ops_rect!(Rect3D, Vector3);
impl_ops_rect!(Rect, Vector2);
impl<T: Number> From<Vector2<T>> for Rect<T> {
    fn from(value: Vector2<T>) -> Self {
        let rect = Rect::default();
        rect.vector_adjust_bounds(value)
    }
}
impl<T: Number> From<Triangle2D<T>> for Rect<T> {
    fn from(value: Triangle2D<T>) -> Self {
        let rect = Rect::default();
        rect.triangle_adjust_bounds(&value)
    }
}

impl<T: Real> CalculateCentroid for Rect<T> {
    type VectorType = Vector2<T>;
    fn centroid(&self) -> Vector2<T> {
        Vector2::new(
            (self.min.x() + self.max.x())*T::from_f64(0.5), 
            (self.min.y() + self.max.y())*T::from_f64(0.5), 
        )
    }
}

impl<T: Real> From<Triangle3D<T>> for Rect3D<T> {
    fn from(value: Triangle3D<T>) -> Self {
        
        Self { min: value[0].min(&value[1].min(&value[2])), max: value[0].max(&value[1].max(&value[2])) }
    }
}

impl<T: Real> SignedDistance<Vector3<T>> for Rect3D<T> {
    type Distance = T;
    fn sdf(&self, object: &Vector3<T>) -> Self::Distance {
        self.round_sdf(object, T::ZERO)
    }
}
impl<T: Real> RoundSignedDistance<Vector3<T>> for Rect3D<T> {
    type Radius = T;
    fn round_sdf(&self, object: &Vector3<T>, r: Self::Radius) -> Self::Distance {
        let centroid = self.centroid();
        let translated_object = object.clone()-centroid + r;
        let size = self.size().div(T::from_f64(2.0));
        let q = translated_object.abs() - size;
        q.max(&Vector3::ZERO).length() + q.x().max(q.y()).min(T::ZERO) - r
    }
}

impl<T: Real> SignedDistance<Vector2<T>> for Rect<T> {
    type Distance = T;
    fn sdf(&self, object: &Vector2<T>) -> Self::Distance {
        self.round_sdf(object, T::ZERO)
    }
}

impl<T: Real> RoundSignedDistance<Vector2<T>> for Rect<T> {
    type Radius = T;
    fn round_sdf(&self, object: &Vector2<T>, r: Self::Radius) -> Self::Distance {
        let centroid = self.centroid();
        let translated_object = object.clone()-centroid + r;
        let size = self.size().div(T::from_f64(2.0));
        let q = translated_object.abs() - size;
        q.max(&Vector2::ZERO).length() + q.x().max(q.y()).min(T::ZERO) - r
    }
}

unsafe impl<T: Number> Zeroable for Rect <T> {
    fn zeroed() -> Self {
        Rect { min: Vector2::ZERO, max: Vector2::ZERO }
    }
}
unsafe impl<T: Number + Pod> Pod for Rect <T> {}

pub struct PackingInfo<T> {
    /// Estimated width it will take to pack objects.
    pub width_hint: T,
    /// Estimated height it will take to pack objects.
    pub height_hint: T,
    // provides padding to the left and right sides of each rectangle
    pub padding_x: T,
    // provides padding to the top and bottom of each rectangle
    pub padding_y: T,
    // provides a margin over the entire packed rectangle on the left and right side
    pub margin_x: T,
    // provides a margin over the entire packed rectangle on the top and bottom
    pub margin_y: T,
}
/// similar to `pack_rects_rows` packs the rectangles in place, while also sorting a payload.
pub fn pack_rects_rows_payload<T: Number + Ord, K: Clone>(rects: &mut [(Rect<T>, K)], info: PackingInfo<T>) -> Rect<T> {
    let PackingInfo { 
        width_hint: width, 
        height_hint: height, 
        padding_x, 
        padding_y, 
        margin_x, 
        margin_y 
    } = info;

    const GROWTH_FACTOR: f64 = (f64::PHI-1.0)*0.5+1.0;
    rects.sort_by_cached_key(|(rect, k)|{
        T::MAX-rect.height()
    });
    let mut x_pos = T::ZERO;
    let mut y_pos = margin_y;
    let mut largest_height = T::ZERO;
    let mut tight_width = T::ZERO;
    // let mut tight_height = T::ZERO;
    for i in 0..rects.len() {
        {
            x_pos += margin_x;
            let rect = &rects[i].0;

            if x_pos + rect.width() > width {
                y_pos += largest_height+margin_y;
                x_pos = margin_x;
                largest_height = T::ZERO;
            }

            let pos_width = x_pos + rect.width();

            if tight_width < pos_width {
                tight_width = pos_width;
            }

            if y_pos + rect.height() > height {
                println!("{} {}", width.to_f64()*GROWTH_FACTOR, height.to_f64()*GROWTH_FACTOR);
                let info = PackingInfo {
                    width_hint: T::from_f64(width.to_f64()*GROWTH_FACTOR),
                    height_hint: T::from_f64(height.to_f64()*GROWTH_FACTOR),
                    ..info
                };
                return pack_rects_rows_payload(rects, info);
            }
        }

        rects[i].0 = Rect::from_lengths(rects[i].0.width(), rects[i].0.height())+Vector2::new(x_pos, y_pos);

        x_pos += rects[i].0.width();
        if rects[i].0.height() > largest_height {
            largest_height = rects[i].0.height();
        }
    }
    Rect::from_lengths(tight_width+margin_x, y_pos+largest_height+margin_y)
}

/// Packs the `rects` with a padding and margin according to a given [`PackingInfo`]. This uses a naive packing approach
/// from [the following blog post](https://www.david-colson.com/2020/03/10/exploring-rect-packing.html). It organizes the
/// rectangles according to their height, giving a staircase pattern. 
/// 
/// # Notes
/// 
/// * The `width_hint` and `height_hint` variables inside [`PackingInfo`] are not the final width and height, instead 
/// it is the [`Rect`] returned by the function.
/// * This function will order in place
pub fn pack_rects_rows<T: Number + Ord>(rects: &mut [Rect<T>], info: PackingInfo<T>) -> Rect<T> {
    let rects: &mut [(Rect<T>, ())] = unsafe { std::slice::from_raw_parts_mut(rects.as_mut() as *mut _ as _, rects.len()) };
    pack_rects_rows_payload(rects, info)
}
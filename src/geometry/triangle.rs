use std::ops::{Index, IndexMut};

use crate::{vector::{Vector2, Vector3}, Number, Real};

use super::CalculateCentroid;

macro_rules! impl_triangle_ops {
    ($structure:tt, $vector:tt, $trait:tt, $func:tt) => {
        impl<T: Number> std::ops::$trait<$vector<T>> for $structure<T>  {
            type Output = Self;
            fn $func(self, rhs: $vector<T>) -> Self::Output {
                Self {
                    v: [
                        self[0].$func(rhs),
                        self[1].$func(rhs),
                        self[2].$func(rhs),
                    ]
                }
            }
        }
    };
}

#[repr(C, align(16))]
#[derive(Default, Debug, Clone, Copy)]
pub struct Triangle3D<T: Number> {
    v: [Vector3<T>; 3],
}

impl<T: Number> Index<usize> for Triangle3D<T> {
    type Output = Vector3<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}
impl<T: Number> IndexMut<usize> for Triangle3D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl<T: Number> Triangle3D<T> {
    pub fn new(v0: Vector3<T>, v1: Vector3<T>, v2: Vector3<T>) -> Self {
        Self { v: [v0,v1,v2] }
    }
}
impl<T: Real> CalculateCentroid for Triangle3D<T> {
    type VectorType = Vector3<T>;
    fn centroid(&self) -> Vector3<T> {
        Vector3::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*T::from_f64(1.0/3.0),
            (self.v[0].y + self.v[1].y + self.v[2].y)*T::from_f64(1.0/3.0), 
            (self.v[0].z + self.v[1].z + self.v[2].z)*T::from_f64(1.0/3.0)
        )
    }
}
impl_triangle_ops!(Triangle3D, Vector3, Add, add);
impl_triangle_ops!(Triangle3D, Vector3, Sub, sub);
impl_triangle_ops!(Triangle3D, Vector3, Mul, mul);
impl_triangle_ops!(Triangle3D, Vector3, Div, div);

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Triangle2D<T: Number> {
    v: [Vector2<T>; 3],
}
impl<T: Number> Index<usize> for Triangle2D<T> {
    type Output = Vector2<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}
impl<T: Number> IndexMut<usize> for Triangle2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl<T: Number> Triangle2D<T> {
    pub fn new(v0: Vector2<T>, v1: Vector2<T>, v2: Vector2<T>) -> Self {
        Self { v: [v0,v1,v2] }
    }
}
impl<T: Real> CalculateCentroid for Triangle2D<T> {
    type VectorType = Vector2<T>;
    fn centroid(&self) -> Vector2<T> {
        Vector2::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*T::from_f64(1.0/3.0),
            (self.v[0].y + self.v[1].y + self.v[2].y)*T::from_f64(1.0/3.0), 
        )
    }
}
impl_triangle_ops!(Triangle2D, Vector2, Add, add);
impl_triangle_ops!(Triangle2D, Vector2, Sub, sub);
impl_triangle_ops!(Triangle2D, Vector2, Mul, mul);
impl_triangle_ops!(Triangle2D, Vector2, Div, div);
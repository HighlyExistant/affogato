use std::ops::{Index, IndexMut, Sub};

use crate::{vector::{OuterProduct, Vector, Vector2, Vector3}, Number, Real};

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
macro_rules! impl_tetrahedron_ops {
    ($structure:tt, $vector:tt, $trait:tt, $func:tt) => {
        impl<T: Number> std::ops::$trait<$vector<T>> for $structure<T>  {
            type Output = Self;
            fn $func(self, rhs: $vector<T>) -> Self::Output {
                Self {
                    v: [
                        self[0].$func(rhs),
                        self[1].$func(rhs),
                        self[2].$func(rhs),
                        self[3].$func(rhs),
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
    pub fn normal(&self) -> Vector3<T> 
        where T: Real {
        self.v[1].sub(self.v[0]).cross(&self.v[2].sub(self.v[1]))
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

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Tetrahedron<T: Number> {
    v: [Vector3<T>; 4],
}
impl<T: Number> Index<usize> for Tetrahedron<T> {
    type Output = Vector3<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}
impl<T: Number> IndexMut<usize> for Tetrahedron<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl<T: Number> Tetrahedron<T> {
    pub fn new(v0: Vector3<T>, v1: Vector3<T>, v2: Vector3<T>, v3: Vector3<T>) -> Self {
        Self { v: [v0,v1,v2,v3] }
    }
    pub fn normal(&self, n: usize) -> Vector3<T> 
        where T: Real {
        let v0 = (n)%4;
        let v1 = (n+1)%4;
        let v2 = (n+2)%4;
        let v3 = (n+3)%4;

        let normal = self.v[v1].sub(self.v[v0]).cross(&self.v[v2].sub(self.v[v1]));
        if self.v[v3].dot(&normal).is_negative() {
            normal.normalize()
        } else {
            -normal.normalize()
        }
    }
    pub fn normal_const<const N: usize>(&self) -> Vector3<T> 
        where T: Real {
        let v0 = const {
            (N)%4
        };
        let v1 = const {
            (N+1)%4
        };
        let v2 = const {
            (N+2)%4
        };
        let v3 = const {
            (N+3)%4
        };
        let normal = self.v[v1].sub(self.v[v0]).cross(&self.v[v2].sub(self.v[v1]));
        if self.v[v3].dot(&normal).is_negative() {
            normal.normalize()
        } else {
            -normal.normalize()
        }
    }
}
impl<T: Real> CalculateCentroid for Tetrahedron<T> {
    type VectorType = Vector3<T>;
    fn centroid(&self) -> Vector3<T> {
        Vector3::new(
            (self.v[0].x + self.v[1].x + self.v[2].x + self.v[3].x)*T::from_f64(1.0/4.0),
            (self.v[0].y + self.v[1].y + self.v[2].y + self.v[3].y)*T::from_f64(1.0/4.0), 
            (self.v[0].z + self.v[1].z + self.v[2].z + self.v[3].z)*T::from_f64(1.0/4.0), 
        )
    }
}
impl_tetrahedron_ops!(Tetrahedron, Vector3, Add, add);
impl_tetrahedron_ops!(Tetrahedron, Vector3, Sub, sub);
impl_tetrahedron_ops!(Tetrahedron, Vector3, Mul, mul);
impl_tetrahedron_ops!(Tetrahedron, Vector3, Div, div);
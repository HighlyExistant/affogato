use num_traits::{AsPrimitive, Float, One, Zero};

use crate::{algebra::Quaternion, FloatingPoint, Number};

use super::{Matrix3, Matrix4, SquareMatrix, Vector2, Vector3};

pub trait Transformation<T: Number> {
    fn matrix(&self) -> Matrix4<T> { Matrix4::identity() }
    fn translation(&self) -> Vector3<T>;
    fn rotation(&self) -> Quaternion<T>
        where T: FloatingPoint;
    fn scaling(&self) -> Vector3<T>;
    fn identity(&mut self);
}
#[derive(Clone)]
pub struct Transform3D<T: FloatingPoint> {
    pub translation: Vector3<T>,
    pub scaling: Vector3<T>,
    pub rotation: Quaternion<T>,
}

impl<T: FloatingPoint> Default for Transform3D<T> 
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T> {
    fn default() -> Self {
        Self { 
            translation: Vector3::new(T::zero(), T::zero(), T::zero()), 
            scaling: Vector3::new(T::one(), T::one(), T::one()), 
            rotation: Quaternion::from_euler(Vector3::new(T::zero(), T::zero(), T::zero()))
        }
    }
}

impl<T: FloatingPoint> Transform3D<T> {
    pub fn new(translation: Vector3<T>, scaling: Vector3<T>, rotation: Quaternion<T>) -> Self {
        Self { translation, scaling, rotation }
    }
}

fn translate<T: Number>(m: &Matrix4<T>, v: Vector3<T>) -> Matrix4<T> {
    let mut result = *m;
    result.w = m.x * v.x + m.y * v.y + m.z * v.z + m.w;
    result
}
impl<T: FloatingPoint> Transformation<T> for Transform3D<T> 
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T> {
    fn identity(&mut self) {
        self.translation = Vector3::from(T::zero());
        self.scaling = Vector3::from(T::one());
        self.rotation = Quaternion::from_euler(Vector3::<T>::zero());
    }
    fn matrix(&self) -> Matrix4<T> {
        let mut mat = Matrix4::identity();
        mat = translate(&mat, self.translation);

        let rotation = Matrix4::from(self.rotation);
        mat = mat * rotation;

        let scale = Matrix4::from_scale(self.scaling);

        mat = mat * scale;
        mat
    }
    fn rotation(&self) -> Quaternion<T>
            where T: FloatingPoint {
        self.rotation
    }
    fn scaling(&self) -> Vector3<T> {
        self.scaling
    }
    fn translation(&self) -> Vector3<T> {
        self.translation
    }
}

pub struct Transform2D<T> {
    pub translation: Vector2<T>,
    pub scaling: Vector2<T>,
    pub rotation: T,
}
pub trait Transformation2D<T: Number> {
    fn matrix(&self) -> Matrix3<T> { Matrix3::identity() }
    fn translation(&self) -> Vector2<T>;
    fn rotation(&self) -> T
        where T: FloatingPoint;
    fn scaling(&self) -> Vector2<T>;
    fn identity(&mut self);
}
impl<T: Number> Default for Transform2D<T> {
    fn default() -> Self {
        Self { 
            translation: Vector2::zero(), 
            scaling: Vector2::one(), 
            rotation: T::zero() 
        }
    }
}
impl<T: FloatingPoint> Transformation2D<T> for Transform2D<T> {
    fn matrix(&self) -> Matrix3<T> {
        Matrix3::new(
            self.rotation.cos()*self.scaling.x, -(self.rotation.sin()), T::zero(), 
            self.rotation.sin(), self.rotation.cos()*self.scaling.y, T::zero(), 
            self.translation.x, self.translation.y, T::one()
        )
    }
    fn translation(&self) -> Vector2<T> {
        self.translation
    }
    fn rotation(&self) -> T
            where T: FloatingPoint {
        self.rotation
    }
    fn scaling(&self) -> Vector2<T> {
        self.scaling
    }
    fn identity(&mut self) {
        self.translation = Vector2::from(T::zero());
        self.scaling = Vector2::from(T::one());
        self.rotation = T::zero();
    }
}

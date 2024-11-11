use num_traits::{AsPrimitive, Float, One, Zero};

use crate::{algebra::Quaternion, sets::Number, RationalNumber};

use super::{Matrix3, Matrix4, SquareMatrix, Vector2, Vector3};

pub trait Transformation<T: Number> {
    fn matrix(&self) -> Matrix4<T> { Matrix4::identity() }
    fn translation(&self) -> Vector3<T>;
    fn rotation(&self) -> Quaternion<T>
        where T: RationalNumber;
    fn scaling(&self) -> Vector3<T>;
    fn identity(&mut self);
}
#[derive(Clone)]
pub struct Transform3D<T: RationalNumber> {
    pub translation: Vector3<T>,
    pub scaling: Vector3<T>,
    pub rotation: Quaternion<T>,
}

impl<T: RationalNumber> Default for Transform3D<T> {
    fn default() -> Self {
        Self { 
            translation: Vector3::new(T::ZERO, T::ZERO, T::ZERO), 
            scaling: Vector3::new(T::ONE, T::ONE, T::ONE), 
            rotation: Quaternion::from_euler(Vector3::new(T::ZERO, T::ZERO, T::ZERO))
        }
    }
}

impl<T: RationalNumber> Transform3D<T> {
    pub fn new(translation: Vector3<T>, scaling: Vector3<T>, rotation: Quaternion<T>) -> Self {
        Self { translation, scaling, rotation }
    }
}

fn translate<T: Number>(m: &Matrix4<T>, v: Vector3<T>) -> Matrix4<T> {
    let mut result = *m;
    result.w = m.x * v.x + m.y * v.y + m.z * v.z + m.w;
    result
}
impl<T: RationalNumber> Transformation<T> for Transform3D<T>  {
    fn identity(&mut self) {
        self.translation = Vector3::from(T::ZERO);
        self.scaling = Vector3::from(T::ONE);
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
            where T: RationalNumber {
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
        where T: RationalNumber;
    fn scaling(&self) -> Vector2<T>;
    fn identity(&mut self);
}
impl<T> Transform2D<T> {
    pub fn new(translation: Vector2<T>, scaling: Vector2<T>, rotation: T) -> Self {
        Self { translation, scaling, rotation }
    }
}
impl<T: Number> Default for Transform2D<T> {
    fn default() -> Self {
        Self { 
            translation: Vector2::zero(), 
            scaling: Vector2::one(), 
            rotation: T::ZERO 
        }
    }
}
impl<T: RationalNumber> Transformation2D<T> for Transform2D<T> {
    fn matrix(&self) -> Matrix3<T> {
        Matrix3::new(
            self.rotation.cos()*self.scaling.x, -(self.rotation.sin()), T::ZERO, 
            self.rotation.sin(), self.rotation.cos()*self.scaling.y, T::ZERO, 
            self.translation.x, self.translation.y, T::ONE
        )
    }
    fn translation(&self) -> Vector2<T> {
        self.translation
    }
    fn rotation(&self) -> T
            where T: RationalNumber {
        self.rotation
    }
    fn scaling(&self) -> Vector2<T> {
        self.scaling
    }
    fn identity(&mut self) {
        self.translation = Vector2::from(T::ZERO);
        self.scaling = Vector2::from(T::ONE);
        self.rotation = T::ZERO;
    }
}

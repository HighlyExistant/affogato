#![allow(unused)]
#![no_std]
use affogato_core::{groups::vector_spaces::VectorSpace, sets::Real};
use affogato_math::{geometry::{Rect, Rect3D}, matrix::{Matrix3, SquareMatrix}, vector::Vector3};

pub mod kinematics;
pub mod collision;
pub mod rigidbody;

pub trait Inertia {
    type Tensor;
    type Vector: VectorSpace;
    fn inverse_inertia(&self, density: <Self::Vector as VectorSpace>::Scalar) -> Self::Tensor;
    fn inverse_inertia_with_mass(&self, mass: <Self::Vector as VectorSpace>::Scalar) -> Self::Tensor;
}
impl<T: Real> Inertia for Rect3D<T> {
    type Tensor = Matrix3<T>;
    type Vector = Vector3<T>;
    fn inverse_inertia(&self, density: T) -> Self::Tensor {
        let lengths = self.max-self.min;
        let volume = lengths.x()*lengths.y()*lengths.z();
        let mass = density*volume;

        let x2 = lengths.x()*lengths.x();
        let y2 = lengths.y()*lengths.y();
        let z2 = lengths.z()*lengths.z();

        let diag = Matrix3::diagonal(Vector3::new(
            T::from_f64(1.0)/((y2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+y2)*T::from_f64(1.0/12.0)*mass)
        ));
        diag
    }
    fn inverse_inertia_with_mass(&self, mass: T) -> Self::Tensor {
        let lengths = self.max-self.min;

        let x2 = lengths.x()*lengths.x();
        let y2 = lengths.y()*lengths.y();
        let z2 = lengths.z()*lengths.z();

        let diag = Matrix3::diagonal(Vector3::new(
            T::from_f64(1.0)/((y2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+y2)*T::from_f64(1.0/12.0)*mass)
        ));
        diag
    }
}

impl<T: Real> Inertia for Rect<T> {
    type Tensor = T;
    type Vector = Vector3<T>;
    fn inverse_inertia(&self, density: T) -> Self::Tensor {
        let lengths = self.max-self.min;
        let volume = lengths.x()*lengths.y();
        let mass = density*volume;
        
        T::ONE/((mass*(lengths.x()*lengths.x() + lengths.y()*lengths.y()))/T::from_f64(12.0))
    }
    fn inverse_inertia_with_mass(&self, mass: T) -> Self::Tensor {
        let lengths = self.max-self.min;
        T::ONE/((mass*(lengths.x()*lengths.x() + lengths.y()*lengths.y()))/T::from_f64(12.0))
    }
}
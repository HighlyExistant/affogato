use affogato_math::{geometry::Rect3D, matrix::{Matrix3, SquareMatrix}, vector::{Vector, Vector3}, Real};

pub mod kinematics;
pub mod collision;
pub mod rigidbody;

pub trait Inertia {
    type Tensor: SquareMatrix;
    fn inertia(&self, density: <<Self::Tensor as SquareMatrix>::Column as Vector>::Scalar) -> Self::Tensor;
    fn inertia_with_mass(&self, mass: <<Self::Tensor as SquareMatrix>::Column as Vector>::Scalar) -> Self::Tensor;
}

impl<T: Real> Inertia for Rect3D<T> {
    type Tensor = Matrix3<T>;
    fn inertia(&self, density: <<Self::Tensor as SquareMatrix>::Column as Vector>::Scalar) -> Self::Tensor {
        let lengths = self.max-self.min;
        let volume = lengths.x*lengths.y*lengths.z;
        let mass = density*volume;

        let x2 = lengths.x*lengths.x;
        let y2 = lengths.y*lengths.y;
        let z2 = lengths.z*lengths.z;

        let diag = Matrix3::diagonal(Vector3::new(
            T::from_f64(1.0)/((y2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+y2)*T::from_f64(1.0/12.0)*mass)
        ));
        diag
    }
    fn inertia_with_mass(&self, mass: <<Self::Tensor as SquareMatrix>::Column as Vector>::Scalar) -> Self::Tensor {
        let lengths = self.max-self.min;

        let x2 = lengths.x*lengths.x;
        let y2 = lengths.y*lengths.y;
        let z2 = lengths.z*lengths.z;

        let diag = Matrix3::diagonal(Vector3::new(
            T::from_f64(1.0)/((y2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+z2)*T::from_f64(1.0/12.0)*mass), 
            T::from_f64(1.0)/((x2+y2)*T::from_f64(1.0/12.0)*mass)
        ));
        diag
    }
}

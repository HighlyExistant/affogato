use core::{fmt::Display, ops::{Index, IndexMut}};

use affogato_core::{groups::vector_spaces::VectorSpace, num::{Number, One, Signed, Zero}, sets::Real};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use bytemuck::{Pod, Zeroable};
use crate::{algebra::Quaternion, vector::{Vector2, Vector3, Vector4}};

mod mat2;
mod mat3;
mod mat4;
pub use mat2::*;
pub use mat3::*;
pub use mat4::*;

pub trait SquareMatrix: Sized {
    type Column: VectorSpace;
    type LowerDimension;
    fn set_identity(&mut self) { *self = Self::identity(); }
    /// The identity of the matrix is one that when multiplied does nothing to a matrix. 
    /// The components of this matrix look like:
    /// ```no_run, ignore
    /// ┌1     0┐
    /// │  .    │
    /// │    .  │
    /// └0     1┘
    /// ```
    fn identity() -> Self;
    fn transpose(&self) -> Self;
    /// The determinant can be understood as the area of the parallelogram formed by
    /// the vectors represented in a matrix. It can help distinguish if the matrix has
    /// been scaled in any way. A matrix whos scale has not changed will have a determinant
    /// of 1.
    fn determinant(&self) -> <Self::Column as VectorSpace>::Scalar;
    fn cofactor(&self, column: usize, row: usize) -> Self::LowerDimension;
    fn cofactor_matrix(&self) -> Self 
        where <Self::Column as VectorSpace>::Scalar: Signed;
    fn adjoint(&self) -> Self 
        where <Self::Column as VectorSpace>::Scalar: Signed {
        self.cofactor_matrix().transpose()
    }
    // Doesn't check whether the determinant is zero
    unsafe fn inverse_unchecked(&self) -> Self 
        where <Self::Column as VectorSpace>::Scalar: Real, 
            Self: core::ops::Mul<<Self::Column as VectorSpace>::Scalar, Output = Self> {
        self.cofactor_matrix().transpose()*(<Self::Column as VectorSpace>::Scalar::ONE/self.determinant())
    }
    // Returns None if the determinant is zero
    fn inverse(&self) -> Option<Self> 
        where <Self::Column as VectorSpace>::Scalar: Real, 
            Self: core::ops::Mul<<Self::Column as VectorSpace>::Scalar, Output = Self> {
        let det = self.determinant();
        if det == <Self::Column as VectorSpace>::Scalar::ZERO {
            None
        } else {
            Some(self.cofactor_matrix().transpose()*(<Self::Column as VectorSpace>::Scalar::ONE/self.determinant()))
        }
    }
    /// Returns a matrix where the diagonal of the matrix is given by a vector.
    fn diagonal(diagonal: Self::Column) -> Self;
}

#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2x3<T: Number> {
    x: Vector2<T>,
    y: Vector2<T>,
    z: Vector2<T>,
}
impl<T: Number> Matrix2x3<T>  {
    pub const fn new(xx: T, xy: T, yx: T, yy: T, zx: T, zy: T) -> Self {
        Self::from_vec(Vector2::new(xx, xy), Vector2::new(yx, yy), Vector2::new(zx, zy))
    }
    pub const fn from_vec(x: Vector2<T>, y: Vector2<T>, z: Vector2<T>) -> Self {
        Self { x, y, z }
    }
}

impl<T: Real> Matrix2x3<T>  {
    pub fn from_transform(translation: Vector2<T>, scaling: Vector2<T>, rotation: T) -> Self {
        Self::new(
            rotation.cos()*scaling.x(), rotation.sin(), 
            -rotation.sin(), rotation.cos()*scaling.y(), 
            translation.x(), translation.y()
        )
    }
}

impl<T: Number + Zero> Zero for Matrix2x3<T> {
    const ZERO: Self = Self::from_vec(Vector2::ZERO, Vector2::ZERO, Vector2::ZERO);
    fn is_zero(&self) -> bool {
        self.x.is_zero() && 
        self.y.is_zero() && 
        self.z.is_zero() 
    }
}

impl<T: Number> core::ops::Add for Matrix2x3<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_vec(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
    type Output = Self;
}
impl<T: Number> core::ops::Sub for Matrix2x3<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_vec(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
    type Output = Self;
}

impl<T: Number> core::ops::Mul<Vector3<T>> for Matrix2x3<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector2::new(
            self.x.x()*rhs.x() + self.y.x()*rhs.y() + self.z.x()*rhs.z(), 
            self.x.y()*rhs.x() + self.y.y()*rhs.y() + self.z.y()*rhs.z(), 
        )
    }
}
unsafe impl<T: Number> Zeroable for Matrix2x3<T> {
    fn zeroed() -> Self {
        Self::ZERO
    }
}
unsafe impl<T: Number + Pod> Pod for Matrix2x3<T> {}

#[cfg(feature="alloc")]
mod alloc_feature {
    extern crate alloc;
    use core::fmt::Display;
    use affogato_core::num::Number;
    use alloc::{string::String};

    use crate::matrix::{Matrix2, Matrix2x3, Matrix3, Matrix4};
    
    impl<T: Number + Display> Display for Matrix2x3<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut row1 = alloc::string::String::from('┌');
            let mut row2 = String::from('└');
            {
                let mut str_row1 = alloc::format!("{}, ", self.x.x());
                let mut str_row2 = alloc::format!("{}, ", self.x.y());
                let max = str_row1.len().max(str_row2.len());
                str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
                row1.push_str(str_row1.as_str());
                row2.push_str(str_row2.as_str());
            }
            let mut str_row1 = alloc::format!("{}", self.y.x());
            let mut str_row2 = alloc::format!("{}", self.y.y());
            let max = str_row1.len().max(str_row2.len());
            str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
            row1.push_str(str_row1.as_str());
            row2.push_str(str_row2.as_str());
            row1.push_str("┐\n");
            row2.push_str("┘\n");
            f.write_str(row1.as_str())?;
            f.write_str(row2.as_str())
        }
    }

    impl<T: Number + Display> Display for Matrix3<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut row1 = String::from('┌');
            let mut row2 = String::from('│');
            let mut row3 = String::from('└');

            for i in 0..2 {
                let mut str_row1 = alloc::format!("{}, ", self[i].x());
                let mut str_row2 = alloc::format!("{}, ", self[i].y());
                let mut str_row3 = alloc::format!("{}, ", self[i].z());
                let max = str_row1.len().max(str_row2.len().max(str_row3.len()));
                str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
                row1.push_str(str_row1.as_str());
                row2.push_str(str_row2.as_str());
                row3.push_str(str_row3.as_str());
            }
            let mut str_row1 = alloc::format!("{}", self.z.x());
            let mut str_row2 = alloc::format!("{}", self.z.y());
            let mut str_row3 = alloc::format!("{}", self.z.z());
            let max = str_row1.len().max(str_row2.len().max(str_row3.len()));
            str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
            row1.push_str(str_row1.as_str());
            row2.push_str(str_row2.as_str());
            row3.push_str(str_row3.as_str());
            
            row1.push_str("┐\n");
            row2.push_str("│\n");
            row3.push_str("┘\n");
            f.write_str(row1.as_str())?;
            f.write_str(row2.as_str())?;
            f.write_str(row3.as_str())
        }
    }

    impl<T: Number + Display> Display for Matrix4<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut row1 = String::from('┌');
            let mut row2 = String::from('│');
            let mut row3 = String::from('│');
            let mut row4 = String::from('└');
            for i in 0..3 {
                let mut str_row1 = alloc::format!("{}, ", self[i].x());
                let mut str_row2 = alloc::format!("{}, ", self[i].y());
                let mut str_row3 = alloc::format!("{}, ", self[i].z());
                let mut str_row4 = alloc::format!("{}, ", self[i].w());
                let max = str_row1.len().max(str_row2.len().max(str_row3.len().max(str_row4.len())));
                str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row4.push_str((0..(max-str_row4.len())).map(|_|{' '}).collect::<String>().as_str());
                row1.push_str(str_row1.as_str());
                row2.push_str(str_row2.as_str());
                row3.push_str(str_row3.as_str());
                row4.push_str(str_row4.as_str());
            }
            let mut str_row1 = alloc::format!("{}", self.w.x());
            let mut str_row2 = alloc::format!("{}", self.w.y());
            let mut str_row3 = alloc::format!("{}", self.w.z());
            let mut str_row4 = alloc::format!("{}", self.w.w());
            let max = str_row1.len().max(str_row2.len().max(str_row3.len().max(str_row4.len())));
            str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row4.push_str((0..(max-str_row4.len())).map(|_|{' '}).collect::<String>().as_str());
            row1.push_str(str_row1.as_str());
            row2.push_str(str_row2.as_str());
            row3.push_str(str_row3.as_str());
            row4.push_str(str_row4.as_str());
            
            row1.push_str("┐\n");
            row2.push_str("│\n");
            row3.push_str("│\n");
            row4.push_str("┘\n");
            f.write_str(row1.as_str())?;
            f.write_str(row2.as_str())?;
            f.write_str(row3.as_str())?;
            f.write_str(row4.as_str())
        }
    }

    impl<T: Number> From<Matrix4<T>> for alloc::vec::Vec<T> {
        fn from(value: Matrix4<T>) -> Self {
            alloc::vec![
                value.x().x(), value.x().y(), value.x().z(), value.x().w(), 
                value.y().x(), value.y().y(), value.y().z(), value.y().w(), 
                value.z().x(), value.z().y(), value.z().z(), value.z().w(),
                value.w().x(), value.w().y(), value.w().z(), value.w().w(),
            ]
        }
    }
    impl<T: Number> From<Matrix3<T>> for alloc::vec::Vec<T> {
        fn from(value: Matrix3<T>) -> Self {
            alloc::vec![
                value.x().x(), value.x().y(), value.x().z(), 
                value.y().x(), value.y().y(), value.y().z(), 
                value.z().x(), value.z().y(), value.z().z(),
            ]
        }
    }
    impl<T: Number> From<Matrix2<T>> for alloc::vec::Vec<T> {
        fn from(value: Matrix2<T>) -> Self {
            alloc::vec![
                value.x().x(), value.x().y(), 
                value.y().x(), value.y().y(), 
            ]
        }
    }
}

impl<T: Number> From<Matrix4<T>> for [T; 4*4] {
    fn from(value: Matrix4<T>) -> Self {
        [
            value.x().x(), value.x().y(), value.x().z(), value.x().w(), 
            value.y().x(), value.y().y(), value.y().z(), value.y().w(), 
            value.z().x(), value.z().y(), value.z().z(), value.z().w(),
            value.w().x(), value.w().y(), value.w().z(), value.w().w(),
        ]
    }
}
impl<T: Number> From<Matrix4<T>> for [Vector4<T>; 4] {
    fn from(value: Matrix4<T>) -> Self {
        [
            value.x(), 
            value.y(), 
            value.z(),
            value.w(),
        ]
    }
}

#[cfg(feature = "alloc")]
pub use alloc_feature::*;

#[cfg(test)]
mod tests {
    use core::ops::Mul;

    use crate::{matrix::{Matrix2x3, SquareMatrix}, vector::{FMat2, FMat3, FMat4, FVec2, FVec3, FVec4, Vector2, Vector3, VectorSpace}};
    #[test]
    fn determinant_test() {
        fn inner_test<M: SquareMatrix>(matrix: M, expected: <M::Column as VectorSpace>::Scalar) {
            let determinant = matrix.determinant();
            assert!(determinant == expected, "determinant function is not implemented correctly");
        }
        inner_test(FMat2::new(1.0, 2.0, 3.0, 4.0), -2.0);
        inner_test(FMat3::new(1.0, 4.0, 5.0, 4.0, 5.0, 4.0, 1.0, 8.0, 6.0), 53.0);
        inner_test(FMat4::new(
            1.0, 4.0, 6.0, 3.0, 
            12.0, 3.0, 5.0, 6.0,
            0.0, 0.0, 45.0, 1.0,
            3.0, 2.0, 0.0, 2.0 
        ), 535.0);
    }
    #[test]
    fn transpose_test() {
        assert!(
            FMat2::new(1.0, 2.0, 3.0, 4.0).transpose().epsilon_eq(&FMat2::new(1.0, 3.0, 2.0, 4.0), 0.00001), 
            "The transpose of a matrix was implemented incorrectly"
        );
        assert!(
            FMat3::new(1.0, 4.0, 5.0, 4.0, 5.0, 4.0, 1.0, 8.0, 6.0).transpose().epsilon_eq( 
                &FMat3::new(1.0, 4.0, 1.0, 4.0, 5.0, 8.0, 5.0, 4.0, 6.0),
                0.00001
            ), "The transpose of a matrix was implemented incorrectly"
        );
        assert!(
            FMat4::new(
                1.0, 4.0, 6.0, 3.0, 
                12.0, 3.0, 5.0, 6.0,
                0.0, 0.0, 45.0, 1.0,
                3.0, 2.0, 0.0, 2.0 
            ).transpose().epsilon_eq(
                &FMat4::new(
                    1.0, 12.0, 0.0, 3.0,
                    4.0, 3.0, 0.0, 2.0, 
                    6.0, 5.0, 45.0, 0.0, 
                    3.0, 6.0, 1.0, 2.0
                ),
                0.00001
            ), "The transpose of a matrix was implemented incorrectly"
        );
    }
    #[test]
    fn inverse_test() {
        let matrix = FMat2::new(1.0, 2.0, 3.0, 4.0);
        assert!(
            matrix*matrix.inverse().unwrap() == FMat2::identity(), "The inverse of a matrix was implemented incorrectly"
        );
        let matrix = FMat3::new(1.0, 3.0, 5.0, 4.0, 7.0, 8.0, 1.0, 1.0, 1.0);
        assert!(
            matrix*matrix.inverse().unwrap() == FMat3::identity(), "The inverse of a matrix was implemented incorrectly"
        );
        let matrix = FMat4::new(
            1.0, 1.0, 1.0, 1.0,
            2.0, 4.0, 6.0, 4.0,
            6.0, 4.0, 7.0, 1.0,
            1.0, 1.0, 1.0, 2.0
        );
        assert!(
            (matrix*matrix.inverse().unwrap()).epsilon_eq(&FMat4::identity(), 0.00001), "The inverse of a matrix was implemented incorrectly"
        );
    }
    #[test]
    fn mul_test() {
        // Matrix2 mul test
        let a = FMat2::new(1.0, 2.0, 3.0, 4.0);
        let b = FMat2::new(2.0, 4.0, 6.0, 8.0);
        assert!(a.mul(b) == FMat2::new(14.0, 20.0, 30.0, 44.0), "Implemented mul incorrectly");
        assert!(b.mul(a) == FMat2::new(14.0, 20.0, 30.0, 44.0), "Implemented mul incorrectly");
        // Matrix3 mul test
        let a = FMat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let b = FMat3::new(2.0, 4.0, 6.0, 2.0, 4.0, 6.0, 2.0, 4.0, 6.0);
        assert!(a.mul(b) == FMat3::new(60.0, 72.0, 84.0, 60.0, 72.0, 84.0, 60.0, 72.0, 84.0), "Implemented mul incorrectly");
        assert!(b.mul(a) == FMat3::new(12.0, 24.0, 36.0, 30.0, 60.0, 90.0, 48.0, 96.0, 144.0), "Implemented mul incorrectly");
        // Matrix4 mul test
        let a = FMat4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = FMat4::new(2.0, 4.0, 6.0, 8.0, 2.0, 4.0, 6.0, 8.0,  2.0, 4.0, 6.0, 8.0,  2.0, 4.0, 6.0, 8.0);
        assert!(a.mul(b) == FMat4::new(180.0, 200.0, 220.0, 240.0, 180.0, 200.0, 220.0, 240.0, 180.0, 200.0, 220.0, 240.0, 180.0, 200.0, 220.0, 240.0), "Implemented mul incorrectly");
        assert!(b.mul(a) == FMat4::new(20.0, 40.0, 60.0, 80.0, 52.0, 104.0, 156.0, 208.0, 84.0, 168.0, 252.0, 336.0, 116.0, 232.0, 348.0, 464.0), "Implemented mul incorrectly");
        
    }
    #[test]
    fn mul_vector_test() {
        let a = FMat2::new(1.0, 2.0, 3.0, 4.0);
        let vb = FVec2::new(1.0, 2.0);
        assert!(a.mul(vb) == FVec2::new(7.0, 10.0), "Implemented mul for vectors incorrectly");
        let a = FMat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let vb = FVec3::new(1.0, 2.0, 3.0);
        assert!(a.mul(vb) == FVec3::new(30.0, 36.0, 42.0), "Implemented mul for vectors incorrectly");
        let a = FMat4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let vb = FVec4::new(1.0, 2.0, 3.0, 4.0);
        assert!(a.mul(vb) == FVec4::new(90.0, 100.0, 110.0, 120.0), "Implemented mul for vectors incorrectly");
        let a = Matrix2x3::new(1, 3, 3, 1, 4, 5);
        let vb = Vector3::new(1, 2, 3);
        assert!(a.mul(vb) == Vector2::new(19, 20), "Implemented mul for vectors incorrectly");
    }
}
#![feature(negative_impls, auto_traits)]
mod sets;
mod animation;
pub mod matrix;
pub mod algebra;
pub mod spaces;
pub mod vector;
pub mod mappings;
pub mod geometry;
pub use sets::*;
pub use animation::*;

#[cfg(test)]
mod test {
    use crate::{matrix::{Matrix2, Matrix3, Matrix4, SquareMatrix}, vector::Vector4};


    #[test]
    fn tests() {
        // println!("{}", cofactor.cofactor_matrix());
    }
}
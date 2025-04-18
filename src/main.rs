use core::{f32, f64, prelude::v1};

use affogato::{algebra::Quaternion, geometry::Rect3D, matrix::SquareMatrix, vector::{DMat2, DVec2, DVec3, FMat3, FVec2, FVec3, FVec4, OuterProduct, Vector, Vector3}, Rotation};
use affogato_physics::collision::Collision;

fn main() {
    let v0 = FMat3::diagonal(FVec3::new(1.0, 2.0, 3.0));
    println!("{}", v0.inverse().unwrap());
    println!("{}", v0.inverse().unwrap()*v0);
}
use affogato::{geometry::{Ray3D, Rect3D, Sphere}, matrix::{Matrix4, SquareMatrix}, vector::{FVec3, Vector}, One, Zero};
use affogato_physics::collision::{Collision, GJKColliderSolid};
use graphics_feature::Geometry;

fn main() {
    let pos = FVec3::from(0.5);
    let a = GJKColliderSolid::new(vec![
        FVec3::new(0.0, 0.0, 0.0),
        FVec3::new(0.0, 0.0, 1.0),
        FVec3::new(0.0, 1.0, 0.0),
        FVec3::new(1.0, 0.0, 0.0),
        FVec3::new(1.0, 1.0, 0.0),
        FVec3::new(0.0, 1.0, 1.0),
        FVec3::new(1.0, 0.0, 1.0),
        FVec3::new(1.0, 1.0, 1.0),
    ], Matrix4::identity());
    let b = GJKColliderSolid::new(vec![
        FVec3::new(0.0, 0.0, 0.0)+pos,
        FVec3::new(0.0, 0.0, 1.0)+pos,
        FVec3::new(0.0, 1.0, 0.0)+pos,
        FVec3::new(1.0, 0.0, 0.0)+pos,
        FVec3::new(1.0, 1.0, 0.0)+pos,
        FVec3::new(0.0, 1.0, 1.0)+pos,
        FVec3::new(1.0, 0.0, 1.0)+pos,
        FVec3::new(1.0, 1.0, 1.0)+pos,
    ], Matrix4::identity());
    println!("{:#?}", a.collides(&b))
}
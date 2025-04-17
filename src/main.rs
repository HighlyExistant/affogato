use affogato::{geometry::Rect3D, vector::{FVec3, Vector3}};
use affogato_physics::collision::Collision;

fn main() {
    let a = Rect3D::from(FVec3::new(1.0, 1.0, 1.0));
    let b = Rect3D::new(FVec3::from(0.5), FVec3::from(1.5));
    println!("{:#?}", a.collides(&b));
}
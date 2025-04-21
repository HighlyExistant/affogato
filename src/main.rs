use affogato::{geometry::{Ray3D, Sphere}, vector::{FVec3, Vector}, Zero};

fn main() {
    let sphere = Sphere::new(FVec3::new(0.2, 0.2, 0.2), 100.0);
    let v0 = Ray3D::new(FVec3::ZERO, FVec3::new(0.7, 0.4, 1.0).normalize());
    println!("{:#?}", v0.intersect_sphere(&sphere));
}
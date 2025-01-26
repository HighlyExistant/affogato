use affogato::{algebra::{ComplexNumber, Imaginary, Quaternion}, geometry::{Ray, Ray3D}, lerp, vector::{FVec3, Vector2, Vector3, Vector4}, matrix::{Matrix2, Matrix3, Matrix4, SquareMatrix}, Zero};

fn main() {
    let ray = Ray3D::new(Vector3::ZERO, Vector3::new(1.0, 1.0, 1.0));
    let x = FVec3::new(1.0, 1.0, 1.0);
    let y = FVec3::new(6.0, 6.0, 6.0);
    let t = FVec3::new(0.0, 0.5, 1.0);
    println!("{:?}", lerp(x, y, t));
}
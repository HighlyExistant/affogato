use affogato::{algebra::{ComplexNumber, Imaginary, Quaternion}, geometry::{Ray, Ray3D}, matrix::{Matrix2, Matrix3, Matrix4, SquareMatrix}, linalg::{Vector2, Vector3, Vector4}, Zero};

fn main() {
    let ray = Ray3D::new(Vector3::ZERO, Vector3::new(1.0, 1.0, 1.0));
    
    println!("{:?}", ray.at(2.0));
}
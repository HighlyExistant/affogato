use affogato::{algebra::{ComplexNumber, Imaginary, Quaternion}, matrix::{Matrix2, Matrix3, Matrix4, SquareMatrix}, vector::{Vector3, Vector4}};

fn main() {
    // let c1 = ComplexNumber::new(10.0f32, 30.0);
    // let c2 = ComplexNumber::new(1.0f32, 2.0);
    // let m1 = Matrix2::new(1.0, 1.0, 0.0, 1.0);
    // let x = Quaternion::new(Vector3::new(1.0, 2.0, -3.0), 4.0f32);
    let i1 = Imaginary::from(10.0);
    let i2 = Imaginary::from(41.0);
    println!("{}", i1*i2);
}
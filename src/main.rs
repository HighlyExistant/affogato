use affogato::{lerp, smoothstep, vector::{FMat2, FVec2}};

fn main() {
    let a = 10.0f32;
    let b = 20.0f32;
    println!("{}", smoothstep::<f32>(a, b, 15.0) );
}